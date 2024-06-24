use crate::timer::{RepeatingTimer, TimerState};
use lifers::{
    engine::{Automaton, ExecutionState},
    frontend::RenderCell,
};
use raylib::{
    color::Color,
    drawing::RaylibDraw,
    ffi::{KeyboardKey, MouseButton},
    math::Vector2,
    RaylibHandle, RaylibThread,
};
use std::time::Duration;

/// Maps a function to both coordinates of all given vectors.
macro_rules! map_vecs {
    ($( $vec:expr ),+ => $f:expr) => {
        Vector2::new($f($( $vec.x ),+), $f($( $vec.y ),+))
    };
}

/// The main struct that implements the frontend capabilities.
pub struct RaylibFrontend<S, D> {
    automaton: Automaton<S, D>,
    rl: RaylibHandle,
    thread: RaylibThread,
    timer: RepeatingTimer,
    cell_margin: u32,
    rect_size: f32,
    center_translation: Vector2,
}

impl<S, D> RaylibFrontend<S, D> {
    // NOTE: This function is quite a mess
    /// Instantiates the frontend.
    ///
    /// You may want to use [`FrontendBuilder`] for convenience.
    #[allow(clippy::as_conversions)]
    pub fn new(
        automaton: Automaton<S, D>,
        update_rate: Duration,
        cell_margin: u32,
        window_size: (u32, u32),
    ) -> Self {
        let (rl, thread) = raylib::init()
            .size(window_size.0 as i32, window_size.1 as i32)
            .title("lifers")
            .build();

        let cell_margin_f = cell_margin as f32;
        let window_size = Vector2::new(window_size.0 as f32, window_size.1 as f32);

        let grid_dimensions = {
            let (x, y) = automaton.grid_size();

            Vector2::new(x as f32, y as f32)
        };
        let rect_size = {
            let Vector2 { x, y } = map_vecs!(
                window_size,
                grid_dimensions
                => |win, cells: f32| (cells + 1.).mul_add(-cell_margin_f, win) / cells
            );
            let side = x.min(y);

            Vector2::new(side, side)
        };
        let grid_size = map_vecs!(
            rect_size,
            grid_dimensions
            => |size, cells: f32| cells.mul_add(size, (cells + 1.) * cell_margin_f)
        );
        let grid_center = grid_size.scale_by(0.5);
        let window_center = window_size.scale_by(0.5);

        // NOTE: `grid_center` is calculated with respect to the window dimensions,
        // so it can't be greater than `window_center`
        #[allow(clippy::arithmetic_side_effects)]
        let center_translation = window_center - grid_center;

        Self {
            automaton,
            rl,
            thread,
            timer: RepeatingTimer::new(update_rate),
            cell_margin,
            rect_size: rect_size.x,
            center_translation,
        }
    }

    /// Checks if the window should close (e.g. `esc` pressed).
    pub fn window_should_close(&self) -> bool {
        self.rl.window_should_close()
    }

    /// Updates the inner timer to compute the next generation according
    /// to the update rate (see [`FrontendBuilder::update_rate()`]).
    pub fn tick(&mut self) -> Option<ExecutionState> {
        matches!(self.timer.update(), TimerState::Finished).then(|| self.automaton.step())
    }

    /// Computes the next generation of the automaton immediately.
    ///
    /// See [`tick()`](Self::tick()) for properly timed updating.
    pub fn step(&mut self) -> ExecutionState {
        self.automaton.step()
    }

    /// Registers default key actions:
    /// - Space -> Pause
    /// - LMB -> Toggle cell under cursor
    pub fn default_key_actions(&mut self) {
        match self.rl.get_key_pressed() {
            None => (),
            Some(key) => match key {
                KeyboardKey::KEY_SPACE => self.timer.toggle_pause(), // HACK?
                // NOTE: Minus reduces the rate (not the time taken), equals
                // increases the rate.
                KeyboardKey::KEY_MINUS => {
                    self.timer = RepeatingTimer::new(self.timer.rate() + Duration::from_millis(10))
                }
                KeyboardKey::KEY_EQUAL => {
                    let duration = self
                        .timer
                        .rate()
                        .checked_sub(Duration::from_millis(10))
                        .unwrap_or(Duration::from_millis(0));

                    self.timer = RepeatingTimer::new(duration);
                }
                _ => (),
            },
        }
    }
}

impl<S: RenderCell<Color>, D> RaylibFrontend<S, D> {
    /// Displays the cell grid using Raylib.
    ///
    /// Manages the job of clearing the background and drawing
    /// all the cells with respect to their [`RenderCell`]
    /// implementation.
    pub fn display_grid(&mut self) {
        let mut drawer = self.rl.begin_drawing(&self.thread);

        drawer.clear_background(Color::GRAY);

        #[allow(clippy::as_conversions)]
        self.automaton
            .cells()
            .iter()
            .enumerate()
            .for_each(|(y, xs)| {
                xs.iter().enumerate().for_each(|(x, cell)| {
                    let pos = map_vecs!(
                        Vector2::new(x as f32, y as f32),
                        self.center_translation
                        => |pos: f32, center_vec| pos.mul_add(self.rect_size, (pos + 1.) * self.cell_margin as f32) + center_vec
                    );

                    let rect = Vector2::new(self.rect_size, self.rect_size);
                    drawer.draw_rectangle_v(pos, rect, cell.render_cell());
                });
            });
    }
}

/// A helper struct to instantiate a [`RaylibFrontend`].
pub struct FrontendBuilder {
    window_size: (u32, u32),
    cell_margin: u32,
    update_rate: Duration,
}

impl FrontendBuilder {
    /// Creates a new builder with the given window size.
    #[must_use]
    pub const fn new(window_size: (u32, u32)) -> Self {
        Self {
            window_size,
            cell_margin: 5,
            update_rate: Duration::from_millis(100),
        }
    }

    /// Sets the cell margin (purely visual).
    #[must_use]
    pub const fn cell_margin(self, cell_margin: u32) -> Self {
        Self {
            cell_margin,
            ..self
        }
    }

    /// Sets the update rate.
    ///
    /// This is the amount of time that passes between each generation
    /// is computed and displayed.
    #[must_use]
    pub const fn update_rate(self, update_rate: Duration) -> Self {
        Self {
            update_rate,
            ..self
        }
    }

    /// Convert the builder to an actual [`RaylibFrontend`].
    pub fn finish<S, D>(self, automaton: Automaton<S, D>) -> RaylibFrontend<S, D> {
        RaylibFrontend::new(
            automaton,
            self.update_rate,
            self.cell_margin,
            self.window_size,
        )
    }
}

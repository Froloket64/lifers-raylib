//! Alternative implementation for life-like automata.

use std::time::Duration;

use life_like::Automaton;
use lifers::{engine::ExecutionState, prelude::*};
use raylib::prelude::*;

use crate::{
    map_vecs,
    timer::{RepeatingTimer, TimerState},
};

/// A version of [`RaylibFrontend`](crate::generic::RaylibFrontend)
/// that works with
/// [`life_like::Automaton`](lifers::engine::life_like::Automaton).
pub struct RaylibFrontend<S, D> {
    // TODO: Generalize
    automaton: Automaton<S, D>,
    rl: RaylibHandle,
    thread: RaylibThread,
    timer: RepeatingTimer,
    // TODO?: Use fractional size and render partial cells
    grid_size: (usize, usize),
    default_color: Color,
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
        init_grid_size: (usize, usize),
        default_color: Color,
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

        let grid_dimensions = Vector2::new(init_grid_size.0 as f32, init_grid_size.1 as f32);
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
            grid_size: init_grid_size,
            default_color,
            cell_margin,
            rect_size: rect_size.x,
            center_translation,
        }
    }

    /// Checks if the window should close (e.g. `esc` pressed).
    pub fn window_should_close(&self) -> bool {
        self.automaton.is_finished() || self.rl.window_should_close()
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
    /// Manages the job of clearing the background and drawing all the
    /// cells with respect to their [`RenderCell`] implementation.
    pub fn display_grid(&mut self) {
        let mut drawer = self.rl.begin_drawing(&self.thread);

        drawer.clear_background(Color::GRAY);

        (0..self.grid_size.0).for_each(|x| (0..self.grid_size.1).for_each(|y| {
            let pos = map_vecs!(
                Vector2::new(x as f32, y as f32),
                self.center_translation
                => |pos: f32, center_vec| pos.mul_add(self.rect_size, (pos + 1.) * self.cell_margin as f32) + center_vec
            );
            // HACK: Unify types (`usize`)
            let cell = self.automaton.cells().get(&(x, y));
            let color = cell.map_or(self.default_color, |c| c.render_cell());

            let rect = Vector2::new(self.rect_size, self.rect_size);
            drawer.draw_rectangle_v(pos, rect, color);
        }))
    }
}

/// A helper struct to instantiate a [`RaylibFrontend`].
pub struct FrontendBuilder {
    window_size: (u32, u32),
    cell_margin: u32,
    update_rate: Duration,
    init_grid_size: (usize, usize),
    default_color: Color,
}

impl FrontendBuilder {
    /// Sets the window size.
    pub const fn window_size(self, window_size: (u32, u32)) -> Self {
        Self {
            window_size,
            ..self
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

    /// Sets the initial grid size.
    ///
    /// Since the life-like automata don't care about the grid size,
    /// this value can later be changed in [`RaylibFrontend`].
    pub const fn grid_size(self, init_grid_size: (usize, usize)) -> Self {
        Self {
            init_grid_size,
            ..self
        }
    }

    /// Sets the color for "dead" cells.
    pub const fn default_color(self, default_color: Color) -> Self {
        Self {
            default_color,
            ..self
        }
    }

    /// Converts the builder to an actual [`RaylibFrontend`].
    pub fn finish<S, D>(self, automaton: Automaton<S, D>) -> RaylibFrontend<S, D> {
        RaylibFrontend::new(
            automaton,
            self.init_grid_size,
            self.default_color,
            self.update_rate,
            self.cell_margin,
            self.window_size,
        )
    }
}

impl Default for FrontendBuilder {
    fn default() -> Self {
        Self {
            window_size: (1024, 768),
            cell_margin: 5,
            update_rate: Duration::from_millis(100),
            init_grid_size: (10, 10),
            default_color: Color::BLACK
        }
    }
}

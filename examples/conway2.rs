//! An example of using [`life_like::Automaton`] for life-like
//! automata.

use lifers::prelude::*;
use lifers_raylib::life_like::FrontendBuilder;
use rand::{thread_rng, Rng};
use raylib::prelude::*;
use std::time::Duration;

const WINDOW_SIZE: (u32, u32) = (1024, 1024);
const CELLS_N: (usize, usize) = (200, 200);
const UPDATE_RATE: Duration = Duration::from_millis(10);

// Create your cell type
#[derive(Debug)]
struct Cell;

// Simply tell the frontend how cells should be drawn with color
impl RenderCell<Color> for Cell {
    fn render_cell(&self) -> Color {
        Color::WHITE
    }
}

fn main() {
    // Create your automaton
    let mut rng = thread_rng();
    let game = life_like::AutomatonBuilder::new(1)
        .init(|| {
            (0..10_000)
                .map(|_| {
                    let pos = (rng.gen_range(0..CELLS_N.0), rng.gen_range(0..CELLS_N.1));

                    (pos, Cell)
                })
                .collect()
        })
        .map(|pos, _, cells| life_like::count_neighbors(pos, 1, cells))
        .run(|_, cell, neighbors_n| match cell {
            Some(_) => (2..=3).contains(&neighbors_n).then_some(Cell),
            None => (neighbors_n == 3).then_some(Cell),
        });

    // Instantiate the frontend with your preferred settings
    let mut frontend = FrontendBuilder::default()
        .window_size(WINDOW_SIZE)
        .update_rate(UPDATE_RATE)
        .finish(game);

    // Event loop
    while !frontend.window_should_close() {
        frontend.display_grid();

        // Register the default key actions
        frontend.default_key_actions();
        // Update the timer to compute the next generations with respect to `update_rate`
        frontend.tick();
    }
}

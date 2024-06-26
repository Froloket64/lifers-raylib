use lifers::prelude::*;
use lifers_raylib::FrontendBuilder;
use rand::random;
use raylib::prelude::*;
use std::time::Duration;

const WINDOW_SIZE: (u32, u32) = (480, 480);
const CELLS_N: (usize, usize) = (20, 20);
const CELL_MARGIN: u32 = 2;
const UPDATE_RATE: Duration = Duration::from_millis(100);

// Create your cell type
struct Cell {
    is_alive: bool,
}

impl Cell {
    fn new(is_alive: bool) -> Self {
        Self { is_alive }
    }
}

// Simply tell the frontend how cells should be drawn with color
impl RenderCell<Color> for Cell {
    fn render_cell(&self) -> Color {
        if self.is_alive {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }
}

fn main() {
    // Create your automaton
    let game = Automaton::build(CELLS_N)
        .init(|_| Cell::new(random::<bool>()))
        .map(|(x, y), _, cells| count_neighbors(cells, (x, y), 1, |cell| cell.is_alive))
        .run(|_, cell, neighbors_n| {
            Cell::new(match cell.is_alive {
                true => (2..=3).contains(&neighbors_n),
                false => neighbors_n == 3,
            })
        });

    // Instantiate the frontend with your preferred settings
    let mut frontend = FrontendBuilder::new(WINDOW_SIZE)
        .cell_margin(CELL_MARGIN)
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

//! A more sohpisticated example using different rules based on a cell's
//! position.
//!
//! It makes cells above the main diagonal follow normal Conway's Game
//! of Life rules, and cells below the diagonal follow [Life without
//! Death](https://en.wikipedia.org/wiki/Life_without_Death)'s
//! rules.

use std::time::Duration;

use lifers::prelude::*;
use lifers_raylib::generic::FrontendBuilder;
use rand::random;
use raylib::color::Color;

const WINDOW_SIZE: (u32, u32) = (1024, 1024);
const CELLS_PER_AXIS: usize = 100;
const CELL_MARGIN: u32 = 2;
const UPDATE_RATE: Duration = Duration::from_millis(50);

struct Cell {
    kind: CellKind,
    is_alive: bool,
}

enum CellKind {
    Normal,
    WithoutDeath,
}

impl RenderCell<Color> for Cell {
    fn render_cell(&self) -> Color {
        match self.is_alive {
            false => Color::BLACK,
            true => match self.kind {
                CellKind::Normal => Color::PINK,
                CellKind::WithoutDeath => Color::GREENYELLOW,
            },
        }
    }
}

fn main() {
    let game = generic::Automaton::build((CELLS_PER_AXIS, CELLS_PER_AXIS))
        .init(|(x, y)| {
            let kind = if x + y >= CELLS_PER_AXIS {
                CellKind::WithoutDeath
            } else {
                CellKind::Normal
            };

            Cell {
                kind,
                is_alive: random(),
            }
        })
        .map(|pos, _, cells| generic::count_neighbors(cells, pos, 1, |cell| cell.is_alive))
        .run(|_, cell, neighbors_n| {
            let is_alive = match cell.kind {
                CellKind::Normal => match cell.is_alive {
                    true => (2..=3).contains(&neighbors_n),
                    false => neighbors_n == 3,
                },
                CellKind::WithoutDeath => cell.is_alive || neighbors_n == 3,
            };

            Cell { is_alive, ..cell }
        });

    let mut frontend = FrontendBuilder::new(WINDOW_SIZE)
        .cell_margin(CELL_MARGIN)
        .update_rate(UPDATE_RATE)
        .finish(game);

    while !frontend.window_should_close() {
        frontend.display_grid();

        frontend.default_key_actions();
        frontend.tick();
    }
}

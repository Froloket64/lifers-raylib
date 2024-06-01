lifers-raylib
-------------
Raylib frontend for [`lifers`](https://crates.io/crates/lifers).

# Usage
To use this frontend, simply create a cell type:
```rust
// Simple Game of Life-like cell
struct Cell {
    is_alive: bool
}
```
Your cells don't have to be this simple, they can have many attributes (type, group, etc.).

Now just implement `RenderCell<Color>` for it:
```rust
use raylib::color::Color;
use lifers::frontend::RenderCell;

impl RenderCell<Color> for Cell {
    fn render_cell(&self) -> Color {
        if cell.is_alive {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }
}
```
Then you can use it to create an `Automaton` and pass it to `RaylibFrontend` (or use `FrontendBuilder`).

Also, see [`examples`](/examples/) and [docs](https://docs.rs/lifers-raylib).

//! Raylib frontend for [lifers].
//!
//! Provides `RaylibFrontend` implementations that do all the
//! displaying and rendering work.  It's recommended to use
//! `FrontendBuilder` for convenience.
//!
//! _NOTE:_ See corresponding modules for different kinds of automata.
//!
//! The frontend can be used in an event loop, similar to a typical
//! Raylib
//! application:
//!
//! ```ignore
//! # use lifers::prelude::*;
//! # use lifers_raylib::generic::FrontendBuilder;
//! let game = /* Automaton */
//! #   todo!();
//! let mut frontend = FrontendBuilder::new((480, 480))
//!     .finish(game);
//!
//! while !frontend.window_should_close() {
//!     frontend.display_grid();
//!     frontend.tick();
//! }
//! ```

#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::allow_attributes_without_reason,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::clone_on_copy,
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::default_numeric_fallback,
    clippy::default_union_representation,
    clippy::exhaustive_enums,
    clippy::expect_used,
    clippy::format_push_string,
    clippy::if_then_some_else_none
)]
#![allow(
    clippy::must_use_candidate,
    clippy::redundant_closure_call,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::module_name_repetitions
)]

pub mod generic;
pub mod life_like;
mod timer;

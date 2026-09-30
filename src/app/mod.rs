//! Ciclo de aplicación y máquina de estados.

mod application;
mod state;

pub use application::run;
pub use state::{GameState, StateMachine};

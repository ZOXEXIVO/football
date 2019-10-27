mod simulator;
pub use simulator::FootballSimulator;

pub mod context;
pub use context::SimulationContext;
pub use context::SimulationEvent;
pub use context::EventType;

pub mod visitor;
pub use visitor::Visitor;

pub use crate::utils::*;
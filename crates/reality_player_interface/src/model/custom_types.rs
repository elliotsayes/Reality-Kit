use std::fmt::Debug;

// Trait for all custom identifiers to be used by a particular game
pub trait CustomGameTrait: Clone + Debug + Sync + Send + 'static {}
// Implement CustomGameTrait for types that meet the requirements
impl<T> CustomGameTrait for T where T: Clone + Debug + Sync + Send + 'static {}

pub trait GameAction: CustomGameTrait {}
impl<T> GameAction for T where T: Clone + Debug + Sync + Send + 'static {}

pub trait GameEvent: CustomGameTrait {}
impl<T> GameEvent for T where T: Clone + Debug + Sync + Send + 'static {}

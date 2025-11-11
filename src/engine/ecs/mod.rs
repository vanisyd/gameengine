use thiserror::Error;
use crate::engine::ecs::world::EntityId;

pub mod world;
pub mod component;

#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("component {0} is not attached to entity {1}")]
    NotAttached(String, EntityId)
}
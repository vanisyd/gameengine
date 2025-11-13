use super::world::{ComponentStorage, EntityId, World};
use crate::engine::render::Position as RenderPosition;
use std::collections::{HashMap, HashSet};

pub trait Component {}

pub struct Position {
    pub x: f32,
    pub y: f32,
    pub position_type: PositionType,
}
#[derive(PartialEq)]
pub enum PositionType {
    Abs,
    Rel,
}
impl Component for Position {}
impl ComponentStorage<Position> for World {
    fn add_component(&mut self, entity_id: EntityId, component: Position) -> &mut Self {
        self.positions.insert(entity_id, component);

        self
    }

    fn fetch(&self, entity_id: &EntityId) -> Option<&Position> {
        self.positions.get(entity_id)
    }

    fn get_mut(&mut self, entity_id: &EntityId) -> Option<&mut Position> {
        self.positions.get_mut(entity_id)
    }
}

pub struct Rotation {
    pub x: u16,
}
impl Component for Rotation {}
impl ComponentStorage<Rotation> for World {
    fn add_component(&mut self, entity_id: EntityId, component: Rotation) -> &mut Self {
        self.rotation.insert(entity_id, component);

        self
    }

    fn fetch(&self, entity_id: &EntityId) -> Option<&Rotation> {
        self.rotation.get(&entity_id)
    }

    fn get_mut(&mut self, entity_id: &EntityId) -> Option<&mut Rotation> {
        self.rotation.get_mut(&entity_id)
    }
}

pub struct Collider {
    pub offset: (isize, isize),
    pub size: (usize, usize),
}
impl Component for Collider {}
impl ComponentStorage<Collider> for World {
    fn add_component(&mut self, entity_id: EntityId, component: Collider) -> &mut Self {
        self.colliders.insert(entity_id, component);

        self
    }

    fn fetch(&self, entity_id: &EntityId) -> Option<&Collider> {
        self.colliders.get(entity_id)
    }

    fn get_mut(&mut self, entity_id: &EntityId) -> Option<&mut Collider> {
        self.colliders.get_mut(entity_id)
    }
}
enum ColliderShape {
    Rectangle { width: usize, height: usize },
}

impl Collider {
    pub fn get_points(&self, entity_pos: RenderPosition) -> (RenderPosition, RenderPosition) {
        let collider_position = (
            entity_pos.0.wrapping_add_signed(self.offset.0),
            entity_pos.1.wrapping_add_signed(self.offset.1),
        );

        (
            (collider_position.0, collider_position.1),
            (
                collider_position.0 + self.size.0,
                collider_position.1 + self.size.1,
            ),
        )
    }
}

#[derive(Default, Debug)]
pub struct CollisionInfo {
    pub collision: HashMap<EntityId, Collision>,
}

impl CollisionInfo {
    pub fn add_collision(&mut self, entity_id: EntityId, collision_sides: HashSet<CollisionSide>) {
        self.collision
            .insert(entity_id, Collision::new(entity_id, collision_sides));
    }
}

#[derive(Default, Debug)]
pub struct Collision {
    pub entity_id: EntityId,
    pub sides: HashSet<CollisionSide>,
}

impl Collision {
    pub fn new(entity_id: EntityId, sides: HashSet<CollisionSide>) -> Self {
        Self { entity_id, sides }
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum CollisionSide {
    Left(usize),
    Right(usize),
    Top(usize),
    Bottom(usize),
    Contained { x: usize, y: usize },
}

#[derive(Default)]
pub struct Children {
    entities: HashSet<EntityId>,
}
impl Component for Children {}
impl ComponentStorage<Children> for World {
    fn add_component(&mut self, entity_id: EntityId, component: Children) -> &mut Self {
        self.children.insert(entity_id, component);

        self
    }

    fn fetch(&self, entity_id: &EntityId) -> Option<&Children> {
        self.children.get(entity_id)
    }

    fn get_mut(&mut self, entity_id: &EntityId) -> Option<&mut Children> {
        self.children.get_mut(entity_id)
    }
}
impl Children {
    pub fn entities(&self) -> &HashSet<EntityId> {
        &self.entities
    }

    pub fn add_entity(&mut self, entity_id: EntityId) {
        self.entities.insert(entity_id);
    }
}

pub struct Parent {
    pub entity: EntityId,
}
impl Component for Parent {}
impl ComponentStorage<Parent> for World {
    fn add_component(&mut self, entity_id: EntityId, component: Parent) -> &mut Self {
        self.parent.insert(entity_id, component);

        self
    }

    fn fetch(&self, entity_id: &EntityId) -> Option<&Parent> {
        self.parent.get(entity_id)
    }

    fn get_mut(&mut self, entity_id: &EntityId) -> Option<&mut Parent> {
        self.parent.get_mut(entity_id)
    }
}

pub struct Size {
    pub x: usize,
    pub y: usize,
}

impl Component for Size {}
impl ComponentStorage<Size> for World {
    fn add_component(&mut self, entity_id: EntityId, component: Size) -> &mut Self {
        self.sizes.insert(entity_id, component);

        self
    }

    fn fetch(&self, entity_id: &EntityId) -> Option<&Size> {
        self.sizes.get(entity_id)
    }

    fn get_mut(&mut self, entity_id: &EntityId) -> Option<&mut Size> {
        self.sizes.get_mut(entity_id)
    }
}

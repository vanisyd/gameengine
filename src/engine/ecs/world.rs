use super::component::*;
use crate::engine::ecs::ComponentError;
use crate::engine::render::Position as RenderPosition;
use std::collections::{HashMap, HashSet};
use std::default::Default;
// TODO: EntityBuilder
pub type EntityId = u32;

pub trait Entity {
    fn get_id(&self) -> EntityId;
}

pub trait ComponentStorage<T: Component> {
    fn add_component(&mut self, entity_id: EntityId, component: T) -> &mut Self;
    fn fetch(&self, entity_id: EntityId) -> Option<&T>;
    fn get_mut(&mut self, entity_id: EntityId) -> Option<&mut T>;
}

pub struct World {
    pub(crate) positions: HashMap<EntityId, Position>,
    pub(crate) colliders: HashMap<EntityId, Collider>,
    pub(crate) rotation: HashMap<EntityId, Rotation>,
    pub(crate) children: HashMap<EntityId, Children>,
    pub(crate) sizes: HashMap<EntityId, Size>,
    pub(crate) parent: HashMap<EntityId, Parent>,
    next_entity_id: EntityId,
}

impl World {
    pub fn new() -> Self {
        Self {
            positions: Default::default(),
            colliders: Default::default(),
            rotation: Default::default(),
            children: Default::default(),
            sizes: Default::default(),
            parent: Default::default(),
            next_entity_id: 1,
        }
    }

    pub fn spawn(&mut self) -> EntityId {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;

        entity_id
    }

    pub fn fetch<T: Component>(&self, id: EntityId) -> Option<&T>
    where
        Self: ComponentStorage<T>,
    {
        <Self as ComponentStorage<T>>::fetch(self, id)
    }

    pub fn get_mut<T: Component>(&mut self, id: EntityId) -> Option<&mut T>
    where
        Self: ComponentStorage<T>,
    {
        <Self as ComponentStorage<T>>::get_mut(self, id)
    }

    pub fn set_parent(&mut self, entity_id: EntityId, parent_id: EntityId) {
        let child = self
            .children
            .entry(parent_id)
            .or_insert(Children::default());
        child.add_entity(entity_id);
        self.parent.insert(entity_id, Parent { entity: parent_id });
    }

    pub fn get_abs_pos(&self, entity_id: EntityId) -> Result<RenderPosition, ComponentError> {
        let entity_pos = self
            .positions
            .get(&entity_id)
            .ok_or_else(|| ComponentError::NotAttached("Position".to_string(), entity_id))?;
        let mut pos = (entity_pos.x as usize, entity_pos.y as usize);
        if entity_pos.position_type == PositionType::Rel {
            if let Some(parent) = self.parent.get(&entity_id) {
                let parent_pos = self.get_abs_pos(parent.entity)?;
                pos = (pos.0 + parent_pos.0, pos.1 + parent_pos.1);
            }
        }

        Ok(pos)
    }

    pub fn get_collision(&self, entity_id: EntityId) -> Result<CollisionInfo, ComponentError> {
        let mut info = CollisionInfo::default();
        let collider = self
            .colliders
            .get(&entity_id)
            .ok_or_else(|| ComponentError::NotAttached("Collider".to_string(), entity_id))?;
        let childs = if let Some(child_component) = self.children.get(&entity_id) {
            child_component.entities()
        } else {
            &HashSet::new()
        };
        let pos = self.get_abs_pos(entity_id);
        let ((min_x, min_y), (max_x, max_y)) = collider.get_points(pos?);

        for (&other_id, other_collider) in &self.colliders {
            if other_id != entity_id && !childs.contains(&other_id) {
                let other_pos = self.get_abs_pos(other_id);
                let ((other_min_x, other_min_y), (other_max_x, other_max_y)) =
                    other_collider.get_points(other_pos?);

                if !(max_x <= other_min_x
                    || min_x > other_max_x
                    || max_y <= other_min_y
                    || min_y >= other_max_y)
                {
                    let overlap_x =
                        (max_x.min(other_max_x)) as isize - (min_x.max(other_min_x)) as isize;
                    let overlap_y =
                        (max_y.min(other_max_y)) as isize - (min_y.max(other_min_y)) as isize;

                    let mut sides = HashSet::new();

                    let fully_inside = min_x > other_min_x
                        && max_x < other_max_x
                        && min_y > other_min_y
                        && max_y < other_max_y;

                    if fully_inside {
                        let rel_x = min_x - other_min_x;
                        let rel_y = min_y - other_min_y;
                        sides.insert(CollisionSide::Contained { x: rel_x, y: rel_y });
                    } else {
                        if overlap_x > 0 {
                            if (min_y <= other_min_y && max_y > other_min_y)
                                || (min_y <= other_max_y && max_y > other_max_y)
                            {
                                sides.insert(CollisionSide::Top(overlap_y as usize));
                            }

                            if (max_y >= other_max_y && min_y < other_max_y)
                                || (max_y > other_min_y && min_y < other_min_y)
                            {
                                sides.insert(CollisionSide::Bottom(overlap_y as usize));
                            }
                        }

                        if (min_x <= other_max_x && max_x > other_max_x)
                            || (min_x <= other_min_x && max_x > other_min_x)
                        {
                            sides.insert(CollisionSide::Left(overlap_x as usize));
                        }

                        if (max_x >= other_min_x && min_x < other_min_x)
                            || (max_x >= other_max_x && min_x < other_max_x)
                        {
                            sides.insert(CollisionSide::Right(overlap_x as usize));
                        }
                    }

                    info.add_collision(other_id, sides);
                }
            }
        }

        Ok(info)
    }

    pub fn remove_entity(&mut self, entity_id: EntityId) {
        self.positions.remove(&entity_id);
        self.sizes.remove(&entity_id);
        self.colliders.remove(&entity_id);
        self.rotation.remove(&entity_id);
        if let Some(child_component) = self.children.get(&entity_id) {
            for child in child_component.entities() {
                self.parent.remove(child);
            }
            self.children.remove(&entity_id);
        }
        self.parent.remove(&entity_id);
    }
}

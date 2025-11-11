#![allow(unused)]

use log::info;
use rand::Rng;
use crate::engine::ecs::world::{ComponentStorage, Entity, EntityId, World};
use crate::engine::render::rect::Rectangle;
use crate::engine::render::{Color, Position, Rotation};
use crate::engine::ecs::component::{
    Position as PositionComponent,
    Collider,
    Rotation as RotationComponent,
    Children, Size, PositionType
};
use crate::engine::render::renderer::Renderer;
use super::{Shape, COLORS};
use super::BLOCK_SIZE;

#[derive(Debug, Clone)]
pub struct Block {
    entity_id: EntityId,
    shape: Shape,
}

impl Block {
    pub fn new(entity_id: EntityId, shape: Shape) -> Self {
        Self {
            entity_id,
            shape
        }
    }

    pub fn render(&self, renderer: &mut Renderer, world: &World) {
        self.shape.clone()
            .with_position(world.get_abs_pos(self.entity_id).unwrap())
            .with_outline(Renderer::rgb_to_u32(232, 222, 220)).fill(renderer);
    }
    
    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }
}
impl Entity for Block {
    fn get_id(&self) -> EntityId {
        self.entity_id
    }
}

pub enum MoveDirection {
    Right,
    Left,
    Down
}

#[derive(Clone)]
pub struct Tetromino {
    entity_id: EntityId,
    pub blocks: [Block; 4]
}

impl Tetromino {
    pub fn new(world: &mut World) -> Self {
        let entity_id = world.spawn();
        world.add_component(entity_id, Children::default())
            .add_component(entity_id, PositionComponent {
                x: (BLOCK_SIZE * 4) as f32,
                y: 0.0,
                position_type: PositionType::Rel,
            });

        let mut rng = rand::rng();
        let mut n: u8 = rng.random_range(0..4);
        let tetromino_type = TetrominoType::get_by_index(n);
        let tetromino_size = match tetromino_type {
            TetrominoType::I => (BLOCK_SIZE * 4, BLOCK_SIZE),
            TetrominoType::L => (BLOCK_SIZE * 3, BLOCK_SIZE * 2),
            TetrominoType::O => (BLOCK_SIZE * 2, BLOCK_SIZE * 2),
            TetrominoType::S => (BLOCK_SIZE * 3, BLOCK_SIZE * 2),
            TetrominoType::T => (BLOCK_SIZE * 3, BLOCK_SIZE * 2)
        };
        world.add_component(entity_id, Size {
            x: tetromino_size.0,
            y: tetromino_size.1
        });


        let color_code: usize = rng.random_range(0..3);
        let blocks = Self::get_blocks(world, tetromino_type, COLORS[color_code]);
        for block in &blocks {
            world.set_parent(block.entity_id, entity_id);
        }

        Self {
            entity_id,
            blocks: blocks.try_into().unwrap()
        }
    }

    fn get_blocks(world: &mut World, kind: TetrominoType, color: Color) -> Vec<Block> {
        let mut blocks: Vec<Block> = Vec::with_capacity(4);
        for block_shape in kind.get_shapes(color) {
            let block_entity = world.spawn();
            blocks.push(Block::new(block_entity, block_shape.to_owned()));
            let shape_size = block_shape.get_size();
            let shape_pos = block_shape.get_position();
            world
                .add_component(block_entity, Size { x: shape_size.0, y: shape_size.1 })
                .add_component(block_entity, PositionComponent {
                    x: (shape_pos.0 * BLOCK_SIZE) as f32,
                    y: (shape_pos.1 * BLOCK_SIZE) as f32,
                    position_type: PositionType::Rel,
                })
                .add_component(block_entity, Collider {
                    offset: (0, -1),
                    size: (BLOCK_SIZE, BLOCK_SIZE + 1)
                });

        }

        blocks
    }

    pub fn render(&self, renderer: &mut Renderer, world: &World) {
        for block in &self.blocks {
            block.render(renderer, world);
        }
    }

    pub fn shift(&mut self, world: &mut World, move_direction: MoveDirection, speed: f32) {
        let pos = world.get_mut::<PositionComponent>(self.entity_id)
            .unwrap();

        match move_direction {
            MoveDirection::Right => pos.x += speed,
            MoveDirection::Left => pos.x -= speed,
            MoveDirection::Down => pos.y += speed
        }
    }

    pub fn rotate(&self, world: &mut World) {
        let pos = world.fetch::<PositionComponent>(self.entity_id)
            .unwrap();
        let rotation = world.fetch::<RotationComponent>(self.entity_id)
            .unwrap();
    }

    /** Builder **/
    pub fn build(world: &mut World) {} //builder
    pub fn with_type(&mut self, kind: TetrominoType) {}
    pub fn with_position(&mut self, position: Position) {}
    pub fn with_rotation(&mut self, rotation: Rotation) {}
}
impl Entity for Tetromino {
    fn get_id(&self) -> EntityId {
        self.entity_id
    }
}

pub enum TetrominoType {
    I,
    O,
    T,
    L,
    S
}
impl TetrominoType {
    pub fn get_by_index(n: u8) -> Self {
        match n {
            0 => TetrominoType::I,
            1 => TetrominoType::O,
            2 => TetrominoType::T,
            3 => TetrominoType::L,
            4 => TetrominoType::S,
            _ => unreachable!()
        }
    }

    pub fn get_shapes(&self, color: Color) -> Vec<Shape> {
        match self {
            TetrominoType::I => {
                vec![
                    Rectangle::new((0, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((1, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((2, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((3, 0), (BLOCK_SIZE, BLOCK_SIZE), color)
                ]
            },
            TetrominoType::O => {
                vec![
                    Rectangle::new((0, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((0, 1), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((1, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((1, 1), (BLOCK_SIZE, BLOCK_SIZE), color)
                ]
            },
            TetrominoType::T => {
                vec![
                    Rectangle::new((1, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((0, 1), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((1, 1), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((2, 1), (BLOCK_SIZE, BLOCK_SIZE), color),
                ]
            },
            TetrominoType::L => {
                vec![
                    Rectangle::new((0, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((0, 1), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((1, 1), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((2, 1), (BLOCK_SIZE, BLOCK_SIZE), color)
                ]
            },
            TetrominoType::S => {
                vec![
                    Rectangle::new((0, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((1, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((1, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((2, 1), (BLOCK_SIZE, BLOCK_SIZE), color)
                ]
            }
        }
    }
}
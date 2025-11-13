#![allow(unused)]

use super::BLOCK_SIZE;
use super::{COLORS, Shape};
use crate::engine::ecs::component::{
    Children, Collider, Position as PositionComponent, PositionType, Rotation as RotationComponent,
    Size,
};
use crate::engine::ecs::world::{ComponentStorage, Entity, EntityId, World};
use crate::engine::render::rect::Rectangle;
use crate::engine::render::renderer::Renderer;
use crate::engine::render::{Color, Position, Rotation};
use log::info;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Block {
    entity_id: EntityId,
    shape: Shape,
}

impl Block {
    pub fn new(entity_id: EntityId, shape: Shape) -> Self {
        Self { entity_id, shape }
    }

    pub fn render(&self, renderer: &mut Renderer, world: &World) {
        self.shape
            .clone()
            .with_position(world.get_abs_pos(self.entity_id).unwrap())
            .with_outline(Renderer::rgb_to_u32(232, 222, 220))
            .fill(renderer);
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
    Down,
}

#[derive(Clone)]
pub struct Tetromino {
    entity_id: EntityId,
    pub blocks: Vec<Block>,
    kind: TetrominoType,
    color: u32,
}

impl Tetromino {
    pub fn spawn(world: &mut World) -> Self {
        let entity_id = world.spawn();
        world
            .add_component(entity_id, Children::default())
            .add_component(
                entity_id,
                PositionComponent {
                    x: (BLOCK_SIZE * 4) as f32,
                    y: 0.0,
                    position_type: PositionType::Rel,
                },
            );

        let mut rng = rand::rng();
        let mut n: u8 = rng.random_range(0..4);
        let tetromino_type = TetrominoType::get_by_index(n);
        let color_code: usize = rng.random_range(0..3);

        let mut tetromino = Tetromino {
            entity_id,
            blocks: Vec::with_capacity(4),
            kind: tetromino_type,
            color: COLORS[color_code],
        };
        tetromino.refresh_shape(world);

        tetromino
    }

    fn refresh_shape(&mut self, world: &mut World) {
        self.blocks = self.get_blocks(world, self.color);
        for block in &self.blocks {
            world.set_parent(block.entity_id, self.entity_id);
        }

        self.refresh_size(world);
    }

    fn refresh_size(&mut self, world: &mut World) {
        let mut width: usize = 0;
        let mut height: usize = 0;

        for block in &self.blocks {
            let block_size = &block.shape.get_size();
            width += block_size.0;
            height += block_size.1;
        }

        if width == 0 {
            width = 1;
        }
        if height == 1 {
            height = 1;
        }

        world.add_component(
            self.entity_id,
            Size {
                x: width * BLOCK_SIZE,
                y: height * BLOCK_SIZE,
            },
        );
    }

    fn get_blocks(&self, world: &mut World, color: Color) -> Vec<Block> {
        let mut blocks: Vec<Block> = Vec::with_capacity(4);
        for block_shape in self.kind.get_shapes(color) {
            let block_entity = world.spawn();
            blocks.push(Block::new(block_entity, block_shape.to_owned()));
            let shape_size = block_shape.get_size();
            let shape_pos = block_shape.get_position();
            world
                .add_component(
                    block_entity,
                    Size {
                        x: shape_size.0,
                        y: shape_size.1,
                    },
                )
                .add_component(
                    block_entity,
                    PositionComponent {
                        x: (shape_pos.0 * BLOCK_SIZE) as f32,
                        y: (shape_pos.1 * BLOCK_SIZE) as f32,
                        position_type: PositionType::Rel,
                    },
                )
                .add_component(
                    block_entity,
                    Collider {
                        offset: (0, -1),
                        size: (BLOCK_SIZE, BLOCK_SIZE + 1),
                    },
                );
        }

        blocks
    }

    pub fn render(&self, renderer: &mut Renderer, world: &World) {
        for block in &self.blocks {
            block.render(renderer, world);
        }
    }

    pub fn shift(&mut self, world: &mut World, move_direction: MoveDirection, speed: f32) {
        let pos = world.get_mut::<PositionComponent>(&self.entity_id).unwrap();

        match move_direction {
            MoveDirection::Right => pos.x += speed,
            MoveDirection::Left => pos.x -= speed,
            MoveDirection::Down => pos.y += speed,
        }
    }

    pub fn remove_block(&mut self, block_id: EntityId, world: &mut World) -> Result<(), String> {
        if let Some(idx) = self
            .blocks
            .iter()
            .position(|block| block.get_id() == block_id)
        {
            self.blocks.remove(idx);
            world.remove_entity(block_id);
            self.refresh_size(world);

            Ok(())
        } else {
            Err(String::from("Block not found"))
        }
    }

    pub fn rotate(&self, world: &mut World) {
        let pos = world.fetch::<PositionComponent>(&self.entity_id).unwrap();
        let rotation = world.fetch::<RotationComponent>(&self.entity_id).unwrap();
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

#[derive(Clone)]
pub enum TetrominoType {
    I,
    O,
    T,
    L,
    S,
}
impl TetrominoType {
    pub fn get_by_index(n: u8) -> Self {
        match n {
            0 => TetrominoType::I,
            1 => TetrominoType::O,
            2 => TetrominoType::T,
            3 => TetrominoType::L,
            4 => TetrominoType::S,
            _ => unreachable!(),
        }
    }

    pub fn get_shapes(&self, color: Color) -> Vec<Shape> {
        match self {
            TetrominoType::I => {
                vec![
                    Rectangle::new((0, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((1, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((2, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((3, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                ]
            }
            TetrominoType::O => {
                vec![
                    Rectangle::new((0, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((0, 1), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((1, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((1, 1), (BLOCK_SIZE, BLOCK_SIZE), color),
                ]
            }
            TetrominoType::T => {
                vec![
                    Rectangle::new((1, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((0, 1), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((1, 1), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((2, 1), (BLOCK_SIZE, BLOCK_SIZE), color),
                ]
            }
            TetrominoType::L => {
                vec![
                    Rectangle::new((0, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((0, 1), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((1, 1), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((2, 1), (BLOCK_SIZE, BLOCK_SIZE), color),
                ]
            }
            TetrominoType::S => {
                vec![
                    Rectangle::new((0, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((1, 0), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((1, 1), (BLOCK_SIZE, BLOCK_SIZE), color),
                    Rectangle::new((2, 1), (BLOCK_SIZE, BLOCK_SIZE), color),
                ]
            }
        }
    }
}

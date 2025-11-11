use std::collections::HashMap;
use log::info;
use winit::keyboard::KeyCode;
use crate::engine::ecs::world::{ComponentStorage, Entity, EntityId, World};
use crate::engine::ecs::component::{Children, Collider, CollisionSide, Position as PositionComponent, PositionType, Size};
use crate::engine::game::Game;
use crate::engine::input::InputState;
use crate::engine::render::renderer::{Renderer};
use crate::engine::render::rect::Rectangle;
use super::BLOCK_SIZE;
use super::tetromino::{Block, MoveDirection, Tetromino};

const BOARD_CELLS: usize = 10;
const BOARD_ROWS: usize = 20;

pub struct Board {
    entity_id: EntityId,
    shape: Rectangle,
    cells: [EntityId; BOARD_CELLS * BOARD_ROWS]
}

impl Board {
    pub fn new(world: &mut World) -> Self {
        let entity_id = world.spawn();
        let shape_pos = (100, 20);
        let shape_size = (BLOCK_SIZE * BOARD_CELLS, BLOCK_SIZE * BOARD_ROWS);
        let shape = Rectangle::new(shape_pos, shape_size,
            Renderer::rgb_to_u32(176,196,222)
        ).with_outline(Renderer::rgb_to_u32(139, 166, 163)).to_owned();

        world.add_component(entity_id, PositionComponent {
                x: shape_pos.0 as f32,
                y: shape_pos.1 as f32,
                position_type: PositionType::Abs,
            })
            .add_component(entity_id, Size { x: shape_size.0, y: shape_size.1 })
            .add_component(entity_id, Children::default())
            .add_component(entity_id, Collider {
                offset: (0, 0),
                size: (shape_size.0, shape_size.1)
            });

        Self {
            entity_id,
            shape,
            cells: [0; BOARD_CELLS * BOARD_ROWS]
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        self.shape.fill(renderer);
    }
}
impl Entity for Board {
    fn get_id(&self) -> EntityId {
        self.entity_id
    }
}

pub struct TetrisGame {
    board: Option<Board>,
    current_tetromino: Option<Tetromino>,
    landed_blocks: Vec<Block>,
    timer: f32,
}

impl Game for TetrisGame {
    fn update(&mut self, delta_time: f32, input_state: &InputState, world: &mut World) {
        self.timer += delta_time;

        if self.board.is_none() {
            self.board = Some(Board::new(world));
        }

        self.handle_lines(world);

        if self.current_tetromino.is_none() {
            if let Some(board) = &self.board {
                let tetromino = Tetromino::new(world);
                world.set_parent(tetromino.get_id(), board.get_id());
                self.current_tetromino = Some(tetromino);
            }
        }

        let move_direction: Option<MoveDirection> = {
            if input_state.keys_pressed.get(&KeyCode::ArrowRight).is_some() {
                Some(MoveDirection::Right)
            } else if input_state.keys_pressed.get(&KeyCode::ArrowLeft).is_some() {
                Some(MoveDirection::Left)
            } else if input_state.keys_pressed.get(&KeyCode::ArrowDown).is_some() {
                Some(MoveDirection::Down)
            } else {
                None
            }
        };

        if let Some(direction) = move_direction {
            self.move_tetromino(world, direction, BLOCK_SIZE as f32);
        }

        if self.timer >= 0.5 && self.current_tetromino.is_some() {
            self.timer = 0.0;
            self.move_tetromino(world, MoveDirection::Down, BLOCK_SIZE as f32);
        }
    }

    fn render(&self, renderer: &mut Renderer, world: &mut World) {
        if let Some(board) = &self.board {
            board.render(renderer);

            if let Some(current_tetromino) = &self.current_tetromino {
                current_tetromino.render(renderer, world);
            }

            for tetromino in &self.landed_blocks {
                tetromino.render(renderer, world);
            }
        }
    }
}

impl TetrisGame {
    pub fn new() -> Self {
        Self {
            board: None,
            current_tetromino: None,
            landed_blocks: Vec::new(),
            timer: 0.0
        }
    }

    fn move_tetromino(&mut self, world: &mut World, move_direction: MoveDirection, speed: f32) {
        let tetromino = self.current_tetromino.as_mut().unwrap();
        let block_ids: Vec<EntityId> = tetromino.blocks.as_ref().into_iter()
            .map(|block| block.get_id()).collect();

        for block_id in &block_ids {
            let collision_info = world.get_collision(*block_id).unwrap();

            for (_, collision) in &collision_info.collision {
                if block_ids.contains(&collision.entity_id) {
                    continue;
                }
                for collision_side in &collision.sides {
                    match collision_side {
                        CollisionSide::Left(depth) => {
                            if matches!(move_direction, MoveDirection::Left) {
                                info!("Collision left: {depth}");
                                return
                            }
                        },
                        CollisionSide::Right(depth) => {
                            if matches!(move_direction, MoveDirection::Right) {
                                info!("Collision right: {depth}");
                                return
                            }
                        },
                        CollisionSide::Bottom(depth) => {
                            info!("Move down. Bottom collision: {depth}");
                            // self.land_tetromino(world);
                            // info!("Tetromino removed");
                            // return
                        },
                        _ => {}
                    }
                }
            }
        }

        tetromino.shift(world, move_direction, speed);
    }

    fn land_tetromino(&mut self, world: &mut World) {
        let tetromino = self.current_tetromino.as_mut().unwrap();
        let board = self.board.as_mut().unwrap();
        let tetromino_pos = world.positions
            .get(&tetromino.get_id()).unwrap();
        let (tetromino_x, tetromino_y) = (
            (tetromino_pos.x as usize / BLOCK_SIZE),
            (tetromino_pos.y as usize / BLOCK_SIZE)
        );
        let tetromino_row = if tetromino_y > 0 {
            tetromino_y * BOARD_CELLS
        } else {
            tetromino_y
        };
        let tetromino_cell = tetromino_row + tetromino_x;

        for block in &tetromino.blocks {
            let block_pos = block.get_shape().get_position();
            let block_row = if block_pos.1 > 0 {
                block_pos.1 * BOARD_CELLS
            } else {
                block_pos.1
            };
            let block_cell = tetromino_cell + block_row + block_pos.0;
            board.cells[block_cell] = block.get_id();
            self.landed_blocks.push(block.clone());
        }

        self.current_tetromino = None;
    }

    fn handle_lines(&mut self, world: &mut World) {
        let board = self.board.as_mut().unwrap();

        for line in board.cells.chunks_exact_mut(BOARD_CELLS) {
            if line.contains(&0) {
                continue;
            }

            for line_block in line.into_iter() {
                world.remove_entity(*line_block);

                if let Some(idx) = self.landed_blocks.iter()
                    .position(|block| block.get_id() == *line_block)
                {
                    self.landed_blocks.remove(idx);
                }
            }
        }
    }
}
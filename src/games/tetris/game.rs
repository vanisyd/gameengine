use super::BLOCK_SIZE;
use super::tetromino::{MoveDirection, RotateDirection, Tetromino};
use crate::engine::ecs::component::{
    Children, Collider, CollisionSide, Parent, Position as PositionComponent, PositionType, Size,
};
use crate::engine::ecs::world::{ComponentStorage, Entity, EntityId, World};
use crate::engine::game::Game;
use crate::engine::input::InputState;
use crate::engine::render::rect::Rectangle;
use crate::engine::render::renderer::Renderer;
use log::info;
use std::collections::HashMap;
use winit::keyboard::KeyCode;

const BOARD_CELLS: usize = 10;
const BOARD_ROWS: usize = 20;

pub struct Board {
    entity_id: EntityId,
    shape: Rectangle,
    cells: [EntityId; BOARD_CELLS * BOARD_ROWS],
}

impl Board {
    pub fn new(world: &mut World) -> Self {
        let entity_id = world.spawn();
        let shape_pos = (100, 20);
        let shape_size = (BLOCK_SIZE * BOARD_CELLS, BLOCK_SIZE * BOARD_ROWS);
        let shape = Rectangle::new(shape_pos, shape_size, Renderer::rgb_to_u32(176, 196, 222))
            .with_outline(Renderer::rgb_to_u32(139, 166, 163))
            .to_owned();

        world
            .add_component(
                entity_id,
                PositionComponent {
                    x: shape_pos.0 as f32,
                    y: shape_pos.1 as f32,
                    position_type: PositionType::Abs,
                },
            )
            .add_component(
                entity_id,
                Size {
                    x: shape_size.0,
                    y: shape_size.1,
                },
            )
            .add_component(entity_id, Children::default())
            .add_component(
                entity_id,
                Collider {
                    offset: (0, -1),
                    size: (shape_size.0, shape_size.1 + 1),
                },
            );

        Self {
            entity_id,
            shape,
            cells: [0; BOARD_CELLS * BOARD_ROWS],
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
    landed_tetrominos: HashMap<EntityId, Tetromino>,
    timer: f32,
    paused: bool
}

impl Game for TetrisGame {
    fn update(&mut self, delta_time: f32, input_state: &InputState, world: &mut World) {
        self.timer += delta_time;

        if self.board.is_none() {
            self.board = Some(Board::new(world));
        }

        if input_state.is_pressed(&KeyCode::KeyP) {
            self.paused = !self.paused;
        }

        self.debug(world, input_state, &delta_time);

        if self.paused {
            return
        }

        self.handle_lines(world);

        if self.current_tetromino.is_none() {
            if let Some(board) = &self.board {
                let tetromino = Tetromino::spawn(world);
                world.set_parent(tetromino.get_id(), board.get_id());
                self.current_tetromino = Some(tetromino);
            }
        }

        if self.timer >= 0.2 {
            self.push_landed_tetrominos(world);
        }

        let move_direction: Option<MoveDirection> = {
            if input_state.is_pressed(&KeyCode::ArrowRight) {
                Some(MoveDirection::Right)
            } else if input_state.is_pressed(&KeyCode::ArrowLeft) {
                Some(MoveDirection::Left)
            } else if input_state.is_pressed(&KeyCode::ArrowDown) {
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

        if input_state.is_pressed(&KeyCode::Space) {
            if let Some(tetromino) = self.current_tetromino.as_mut() {
                tetromino.rotate(world, RotateDirection::Left);
            }
        }
    }

    fn render(&self, renderer: &mut Renderer, world: &mut World) {
        if let Some(board) = &self.board {
            board.render(renderer);

            if let Some(current_tetromino) = &self.current_tetromino {
                current_tetromino.render(renderer, world);
            }

            for (_, tetromino) in &self.landed_tetrominos {
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
            landed_tetrominos: HashMap::new(),
            timer: 0.0,
            paused: false
        }
    }

    fn push_landed_tetrominos(&mut self, world: &mut World) {
        for (_, tetromino) in &mut self.landed_tetrominos {
            let block_ids: Vec<EntityId> = tetromino
                .blocks
                .iter()
                .map(|block| block.get_id())
                .collect();
            let mut can_fall = true;
            for block_id in &block_ids {
                let collision_info = world.get_collision(*block_id).unwrap();

                for (_, collision) in &collision_info.collision {
                    if block_ids.contains(&collision.entity_id) {
                        continue;
                    }
                    if collision
                        .sides
                        .iter()
                        .find(|side| matches!(side, CollisionSide::Bottom(_)))
                        .is_some()
                    {
                        can_fall = false;
                    }
                }
            }
            if can_fall {
                tetromino.shift(world, MoveDirection::Down, BLOCK_SIZE as f32);
            }
        }
    }

    fn move_tetromino(&mut self, world: &mut World, move_direction: MoveDirection, speed: f32) {
        let tetromino = self.current_tetromino.as_mut().unwrap();
        let block_ids: Vec<EntityId> = tetromino
            .blocks
            .iter()
            .map(|block| block.get_id())
            .collect();

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
                                // info!("Collision left: {depth}");
                                return;
                            }
                        }
                        CollisionSide::Right(depth) => {
                            if matches!(move_direction, MoveDirection::Right) {
                                // info!("Collision right: {depth}");
                                return;
                            }
                        }
                        CollisionSide::Bottom(depth) => {
                            // info!("Move down. Bottom collision: {depth}");
                            self.landed_tetrominos
                                .insert(tetromino.get_id(), tetromino.clone());
                            self.current_tetromino = None;
                            return;
                        }
                        CollisionSide::Contained { x, y } => {
                            // info!("Contained: {x}, {y}");
                        }
                        _ => {}
                    }
                }
            }
        }

        tetromino.shift(world, move_direction, speed);
    }

    fn refresh_cells(&mut self, world: &mut World) {
        let board = self.board.as_mut().unwrap();
        board.cells.fill(0);
        
        for (_, tetromino) in &self.landed_tetrominos {
            let tetromino_pos = world.positions.get(&tetromino.get_id()).unwrap();
            let (tetromino_x, tetromino_y) = (
                (tetromino_pos.x as usize / BLOCK_SIZE),
                (tetromino_pos.y as usize / BLOCK_SIZE),
            );
            let tetromino_row = if tetromino_y > 0 {
                tetromino_y * BOARD_CELLS
            } else {
                tetromino_y
            };
            let tetromino_cell = tetromino_row + tetromino_x;

            for block in &tetromino.blocks {
                let shape_pos = world.fetch::<PositionComponent>(&block.get_id())
                    .unwrap();
                let block_pos: (usize, usize) = {
                    let x = if shape_pos.x as usize > 0 {
                        shape_pos.x as usize / BLOCK_SIZE
                    } else {
                        shape_pos.x as usize
                    };

                    let y = if shape_pos.y as usize > 0 {
                        shape_pos.y as usize / BLOCK_SIZE
                    } else {
                        shape_pos.y as usize
                    };

                    (x, y)
                };
                let block_row = if block_pos.1 > 0 {
                    block_pos.1 * BOARD_CELLS
                } else {
                    block_pos.1
                };
                let block_cell = tetromino_cell + block_row + block_pos.0;
                board.cells[block_cell] = block.get_id();
            }
        }
    }

    fn handle_lines(&mut self, world: &mut World) {
        self.refresh_cells(world);

        let board = self.board.as_mut().unwrap();
        for line in board.cells.chunks_exact_mut(BOARD_CELLS) {
            if line.contains(&0) {
                continue;
            }

            for line_block in line.into_iter() {
                let parent = world.fetch::<Parent>(line_block).unwrap();
                let tetromino = self.landed_tetrominos.get_mut(&parent.entity).unwrap();
                tetromino.remove_block(*line_block, world).unwrap();
            }

            line.fill(0);
        }
    }

    fn debug(&mut self, world: &mut World, input_state: &InputState, delta_time: &f32) {
        if !self.paused {
            return
        }

        let cursor_x: f32 = input_state.mouse_pos.0 as f32;
        let cursor_y: f32 = input_state.mouse_pos.1 as f32;

        let mut nearest: Vec<(EntityId, f32)> = world.positions.iter()
            .map(|(&id, pos)| {
                let dx = pos.x - cursor_x;
                let dy = pos.y - cursor_y;
                (id, dx * dx + dy * dy)
            }).collect();
        nearest
            .sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        for entity_id in &nearest[0..1] {
            
        }
    }
}

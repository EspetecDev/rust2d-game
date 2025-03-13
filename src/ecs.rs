use std::collections::HashMap;
use minifb::clamp;

use crate::HEIGHT;
use crate::WIDTH;

pub type Entity = usize;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: u32,
    pub y: u32
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub dx: i32,
    pub dy: i32
}

#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub x_size: usize,
    pub y_size: usize,
    pub color: u32
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tag {
    pub tag: &'static str
}

pub struct Components {
    pub positions: HashMap<Entity, Position>,
    pub velocities: HashMap<Entity, Velocity>,
    pub sprites: HashMap<Entity, Sprite>,
    pub tags: HashMap<Entity, Vec<Tag>>
}

impl Components {

    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
            velocities: HashMap::new(),
            sprites: HashMap::new(),
            tags: HashMap::new()
        }
    }
}

pub struct EntityManager {
    next_id: Entity,
    components: Components
}

impl EntityManager {
    
    pub fn new() -> Self {
        Self { next_id: 0, components: Components::new() }
    }

    pub fn create_entity(&mut self) -> Entity {
     
        let entity = self.next_id;
        self.next_id += 1;
        entity
    }

    pub fn add_position(&mut self, entity: Entity, x: u32, y: u32) {
        self.components.positions.insert(entity, Position { x, y });
    }

    pub fn add_velocity(&mut self, entity: Entity, dx: i32, dy: i32) {
        self.components.velocities.insert(entity, Velocity { dx, dy});
    }
    
    pub fn reset_velocity(&mut self, entity: Entity,  reset_x: bool, reset_y: bool) {
        let original_vel = self.components.velocities.get(&entity).unwrap();
        let mut vel = Velocity {dx: 0, dy: 0};
        if reset_x { vel.dx = 0; } else { vel.dx = original_vel.dx; }
        if reset_y { vel.dy = 0; } else { vel.dy = original_vel.dy; }

        self.components.velocities.insert(entity, vel);
    }

    pub fn add_tag(&mut self, entity: Entity, tag: Tag) {
        let tags = self.components.tags.entry(entity).or_insert_with(Vec::new);
        if !tags.contains(&tag) {
            tags.push(tag);
        } else { println!("Entity already has tag: {:?}", tag); }
    }

    pub fn add_sprite(&mut self, entity: Entity, x_size: usize, y_size: usize, color: u32) {
        self.components.sprites.insert(entity, Sprite { x_size, y_size, color });
    }
}

pub fn movement_system(em: &mut EntityManager) {

    for (entity, position) in em.components.positions.iter_mut() {
        let mut new_x: usize = 0;
        let mut new_y: usize = 0;

        if let Some(velocity) = em.components.velocities.get(entity) {
            new_x = ((position.x as i32) + velocity.dx) as usize;
            new_y = ((position.y as i32) + velocity.dy) as usize;
        }

        if let Some(sprite) = em.components.sprites.get(entity) {
            new_x = clamp(0, new_x, WIDTH);
            new_y = clamp(0, new_y, HEIGHT);

            position.x = if new_x + sprite.x_size <= WIDTH { new_x as u32 } else { position.x};
            position.y = if new_y + sprite.y_size <= HEIGHT { new_y as u32 }else { position.y};
        }
    }
}

pub fn draw_entities(em: &EntityManager, buffer: &mut [u32]) {

    for (entity, position) in em.components.positions.iter() {
        if let Some(sprite) = em.components.sprites.get(entity) {
            let x = position.x as usize;
            let y = position.y as usize;
    
            if x < WIDTH && y < HEIGHT {
                let idx = y * WIDTH + x; // first pixel pos
                for i in 0..sprite.x_size {
                    for j in 0..sprite.y_size {
                        let pixel_idx = idx + i + j * WIDTH;
                        if pixel_idx < buffer.len() {
                            buffer[pixel_idx] = sprite.color;
                        }
                    }
                }
            }
        }
    }
}

use crate::{sick_physics::Physics, user_input::UserInput};
use nphysics2d::object::{DefaultBodyHandle, DefaultColliderHandle};
use std::{cell::RefCell, collections::HashMap, mem, rc::Rc};

pub type Color = [f32; 4];

#[allow(dead_code)]
pub fn red() -> Color {
    [1.0, 0.0, 0.0, 1.0]
}

#[allow(dead_code)]
pub fn green() -> Color {
    [0.0, 1.0, 0.0, 1.0]
}

#[allow(dead_code)]
pub fn blue() -> Color {
    [0.0, 0.0, 1.0, 1.0]
}

#[allow(dead_code)]
pub fn yellow() -> Color {
    [1.0, 1.0, 0.0, 1.0]
}

#[allow(dead_code)]
pub fn pink() -> Color {
    [1.0, 0.0, 1.0, 1.0]
}

#[allow(dead_code)]
pub fn cyan() -> Color {
    [0.0, 1.0, 1.0, 1.0]
}

pub trait Entity {
    fn id(&self) -> &String;
    fn triangles(&self) -> &Vec<Triangle>;
    fn update(&mut self, _time_passed: f32, _game_state: &mut GameState) {}
    fn position(&self) -> (f32, f32) {
        return (0.0, 0.0);
    }
    fn rotation(&self) -> f32 {
        return 0.0;
    }
    fn init_physics(
        &mut self,
        _physics: &mut Physics,
    ) -> Option<(DefaultBodyHandle, DefaultColliderHandle)> {
        None
    }
}

pub struct EntityEntry {
    pub entity: Rc<RefCell<dyn Entity>>,
    pub physics_initialized: bool,
    pub physics_body: Option<DefaultBodyHandle>,
    pub physics_collision: Option<DefaultColliderHandle>,
}

#[derive(Default)]
pub struct Entities {
    map: HashMap<String, EntityEntry>,
}

impl Entities {
    // TODO: on drop remove all physics bodies and collisions

    pub fn new() -> Self {
        Default::default()
    }

    pub fn iter(&self) -> impl Iterator<Item = &mut dyn Entity> {
        self.map
            .values()
            .map(|entry| unsafe { mem::transmute(entry.entity.as_ptr()) })
    }

    pub fn entries(&mut self) -> impl Iterator<Item = &mut EntityEntry> {
        self.map.values_mut()
    }

    pub fn apply_ops(&mut self, ops: &mut EntityOps, physics: &mut Physics) {
        for op in ops.items_mut().drain(..) {
            match op {
                EntityOp::Insert(entity) => {
                    let res = entity.borrow_mut().init_physics(physics);

                    let id = entity.borrow().id().clone();

                    let old = self.map.insert(
                        id.clone(),
                        EntityEntry {
                            entity,
                            physics_initialized: false,
                            physics_body: res.map(|x| x.0),
                            physics_collision: res.map(|x| x.1),
                        },
                    );
                    if let Some(_) = &old {
                        panic!("Entity with id {} already exists", id);
                    }
                }
                // TODO: remove all physics bodies and collisions
                EntityOp::Remove(_id) => unimplemented!(),
            }
        }
    }
}

pub enum EntityOp {
    Insert(Rc<RefCell<dyn Entity>>),
    Remove(String),
}

pub struct EntityOps {
    items: Vec<EntityOp>,
}

impl EntityOps {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn insert(&mut self, entity: impl Entity + 'static) {
        self.items
            .push(EntityOp::Insert(Rc::new(RefCell::new(entity))));
    }

    pub fn remove(&mut self, id: &String) {
        self.items.push(EntityOp::Remove(id.clone()));
    }

    pub fn items_mut(&mut self) -> &mut Vec<EntityOp> {
        &mut self.items
    }
}

pub struct GameState<'a> {
    pub input: &'a UserInput,
    pub physics: &'a mut Physics,
    pub entities: &'a Entities,
    pub entity_ops: &'a mut EntityOps,
}

#[derive(Clone, Default)]
pub struct Triangle {
    pub coords: [f32; 9],
    pub color: [f32; 4],
    pub shader_id: ShaderId,
    pub wireframe: bool,
}

#[derive(Clone, Default)]
pub enum ShaderId {
    #[default]
    Default,
    Water,
}

impl Triangle {
    pub fn new(coords: [f32; 9], color: [f32; 4]) -> Triangle {
        Triangle {
            coords,
            color,
            ..Default::default()
        }
    }

    pub fn from_points(
        p1: (f32, f32),
        p2: (f32, f32),
        p3: (f32, f32),
        color: [f32; 4],
    ) -> Triangle {
        Triangle {
            coords: [p1.0, p1.1, 0.0, p2.0, p2.1, 0.0, p3.0, p3.1, 0.0],
            color,
            ..Default::default()
        }
    }
}

use std::{cell::RefCell, rc::Rc};

use js_sys::Math::random;

use crate::{
    fish::{Fish, FishRace},
    level::Level,
    log,
    player::Player,
    sick_physics::Physics,
    types::{Entities, Entity, EntityOps, GameState, ShaderId, Triangle},
    user_input::{self, InputHandler},
};

pub struct Game {
    render_buffer: Vec<Triangle>,
    entities: Entities,
    entity_ops: EntityOps,
    input_handler: InputHandler,
    physics: Rc<RefCell<Physics>>,

    last_fps_print: f64,
    frames_drawn: usize,
}

pub struct ShitItem {
    id: String,
    triangles: Vec<Triangle>,
    moving: [f32; 9],
}

impl Entity for ShitItem {
    fn id(&self) -> &String {
        &self.id
    }
    fn triangles(&self) -> &Vec<crate::types::Triangle> {
        &self.triangles
    }
    fn update(&mut self, _time_passed: f32, _game_state: &mut GameState) {
        self.update();
    }
}

impl ShitItem {
    fn update(&mut self) {
        let tri = &mut self.triangles[0].coords;
        for n in 0..9 {
            let bounds = match n % 3 {
                0 => (23.0, 46.0),
                1 => (5.0, 10.0),
                _ => (0.0, 0.0),
            };
            let hit_edge = tri[n] <= bounds.0 || tri[n] >= bounds.1;

            let movement = match hit_edge {
                true => -self.moving[n],
                false => self.moving[n],
            };

            tri[n] = tri[n] + movement;
            self.moving[n] = movement;
        }
    }
}

impl Game {
    pub fn new() -> Game {
        let mut entity_ops = EntityOps::new();

        let level = Level::load_from_svg_str(include_str!("../assets/map.svg"));
        let ground = level.ground();
        let player = Player::new(level.player_pos());
        entity_ops.insert(level);
        entity_ops.insert(player);

        let mut physics = Physics::new();

        physics.insert_ground(
            ground.0 + ground.2 / 2.0,
            ground.1 + ground.3 / 2.0,
            ground.2 / 2.0,
            ground.3 / 2.0,
        );
        let physics = Rc::new(RefCell::new(physics));

        let fish = Fish::new(FishRace::Goldfish);
        entity_ops.insert(fish);

        let mut input_handler = user_input::InputHandler::new();
        input_handler.attach();

        Game {
            render_buffer: vec![],
            physics,
            entities: Entities::new(),
            entity_ops,
            input_handler,
            frames_drawn: 0,
            last_fps_print: 0.0,
        }
    }

    pub fn tick(&mut self, time_passed: f32) {
        self.render_buffer.clear();

        {
            let input = self.input_handler.current_state();
            let mut physics = self.physics.as_ref().borrow_mut();

            // TODO: set timestep
            physics.step();

            let mut game_state = GameState {
                physics: &mut physics,
                input: &input,
                entities: &self.entities,
                entity_ops: &mut self.entity_ops,
            };

            for entity in self.entities.iter() {
                entity.update(time_passed, &mut game_state);
            }

            self.entities.apply_ops(&mut self.entity_ops, &mut *physics);
        }

        self.input_handler.after_update();
        self.handle_fps();
    }

    pub fn entities(&self) -> &Entities {
        &self.entities
    }

    fn handle_fps(&mut self) {
        self.frames_drawn += 1;

        if self.frames_drawn % 100 == 0 {
            let window = web_sys::window().expect("should have a window in this context");
            let performance = window
                .performance()
                .expect("performance should be available");
            let now = performance.now();
            let fps = 100.0 * (1.0 / ((now - self.last_fps_print) / 1000.0));
            log!("FPS {:.2}", fps);
            self.last_fps_print = now;
        }
    }
}

#[allow(dead_code)]
pub fn random_shit_items(n: usize) -> Vec<ShitItem> {
    (0..n)
        .map(|_| {
            let mut t = [0.0; 9];
            let mut moving = [0.0; 9];
            for i in 0..3 {
                t[i * 3] = (random() * 23.0 + 23.0) as f32;
                t[i * 3 + 1] = (random() * 5.0 + 5.0) as f32;

                moving[i * 3] = (random() * 0.01 - 0.01) as f32;
                moving[i * 3 + 1] = (random() * 0.01 - 0.01) as f32;
            }
            ShitItem {
                id: format!("shit_item-{}", random()),
                triangles: vec![Triangle {
                    coords: t,
                    color: [0.9; 4],
                    shader_id: ShaderId::Default,
                    wireframe: false,
                }],
                moving,
            }
        })
        .collect()
}

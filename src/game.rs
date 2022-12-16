use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use js_sys::Math::random;

use crate::{
    fish::{Fish, FishRace},
    level::Level,
    log,
    player::Player,
    sick_physics::Physics,
    types::{Entity, GameState, ShaderId, Triangle},
    user_input::{self, InputHandler},
};

pub struct Game {
    render_buffer: Vec<Triangle>,
    entities: HashMap<String, Rc<RefCell<dyn Entity>>>,
    input_handler: InputHandler,

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
        let mut entities: HashMap<String, Rc<RefCell<dyn Entity>>> = HashMap::new();

        let level = Level::load_from_svg_str(include_str!("../assets/map.svg"));
        let ground = level.ground();
        let player = Player::new(level.player_pos());
        entities.insert(level.id().clone(), Rc::new(RefCell::new(level)));
        entities.insert(player.id().clone(), Rc::new(RefCell::new(player)));

        let mut physics = Physics::new();

        physics.insert_ground(
            ground.0 + ground.2 / 2.0,
            ground.1 + ground.3 / 2.0,
            ground.2 / 2.0,
            ground.3 / 2.0,
        );
        // physics.insert_cube(16.1, 0.0, 2.0);
        entities.insert(physics.id().clone(), Rc::new(RefCell::new(physics)));

        let fish = Fish::new(FishRace::Goldfish);
        entities.insert(fish.id().clone(), Rc::new(RefCell::new(fish)));

        let mut input_handler = user_input::InputHandler::new();
        input_handler.attach();

        Game {
            render_buffer: vec![],
            entities,
            input_handler,
            frames_drawn: 0,
            last_fps_print: 0.0,
        }
    }

    pub fn tick(&mut self, time_passed: f32) {
        self.render_buffer.clear();
        // data
        let input = self.input_handler.current_state();
        let mut game_state = GameState {
            input: &input,
            entities: &self.entities,
            entity_ops: vec![],
        };
        for (id, item) in &self.entities {
            item.as_ref().borrow_mut().on_user_input(&input);
            item.as_ref()
                .borrow_mut()
                .update(time_passed, &mut game_state);
        }
        for op in game_state.entity_ops {
            op(&mut self.entities);
        }
        self.handle_fps();
    }

    pub fn entities(&self) -> impl Iterator<Item = Ref<dyn Entity>> {
        self.entities.values().map(|item| item.borrow())
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
                }],
                moving,
            }
        })
        .collect()
}

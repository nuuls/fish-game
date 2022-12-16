use js_sys::Math::random;

use crate::{
    level::Level,
    log,
    player::Player,
    types::{Entity, ShaderId, Triangle},
    user_input::{self, InputHandler},
};

pub struct Game {
    render_buffer: Vec<Triangle>,
    entities: Vec<Box<dyn Entity>>,
    input_handler: InputHandler,

    last_fps_print: f64,
    frames_drawn: usize,
}

struct ShitItem {
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
    fn update(&mut self, _time_passed: f32) {
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
        let mut entities: Vec<Box<dyn Entity>> = random_shit_items(3)
            .into_iter()
            .map(|si| Box::new(si) as Box<dyn Entity>)
            .collect();

        let level = Level::load_from_svg_str(include_str!("../assets/map.svg"));
        let player = Box::new(Player::new(level.player_pos()));
        entities.push(Box::new(level));
        entities.push(player);

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

    pub fn next_frame(&mut self, time_passed: f32) -> &Vec<Box<dyn Entity>> {
        self.render_buffer.clear();
        // data
        let input = self.input_handler.current_state();
        for item in &mut self.entities {
            item.on_user_input(&input);
            item.update(time_passed);
        }

        self.handle_fps();

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

fn random_shit_items(n: usize) -> Vec<ShitItem> {
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
                id: format!("shit_item-{}", n),
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

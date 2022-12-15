use js_sys::Math::random;

use crate::{
    level::Level,
    log,
    types::{Entity, Triangle},
};

pub struct Game {
    level: Level,
    render_buffer: Vec<Triangle>,
    entities: Vec<Box<dyn Entity>>,

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
    fn update(&mut self, time_passed: f32) {
        self.update();
    }
}

impl ShitItem {
    fn update(&mut self) {
        let tri = &mut self.triangles[0].coords;
        for n in 0..9 {
            let bounds = match n % 3 {
                0 => (23.0, 46.0),
                1 => (17.5, 18.0),
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
        Game {
            render_buffer: vec![],
            entities: random_shit_items(1000)
                .into_iter()
                .map(|si| Box::new(si) as Box<dyn Entity>)
                .collect(),
            level: Level::load_from_svg_str(include_str!("../assets/map.svg")),
            frames_drawn: 0,
            last_fps_print: 0.0,
        }
    }

    pub fn next_frame(&mut self) -> &Vec<Triangle> {
        self.render_buffer.clear();
        let level_triangles = self.level.triangles();
        for tri in level_triangles {
            self.render_buffer.push(tri.clone())
        }

        // data
        for item in &mut self.entities {
            item.update(0.0);
            for tri in item.triangles() {
                self.render_buffer.push(tri.clone());
            }
        }

        self.handle_fps();

        &self.render_buffer
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
            for i in 0..3 {
                t[i * 3] = (random() * 23.0 + 23.0) as f32;
                t[i * 3 + 1] = (random() * 1.0 + 10.5) as f32;
            }
            ShitItem {
                id: format!("shit_item-{}", n),
                triangles: vec![Triangle {
                    coords: t,
                    color: [0.9; 4],
                }],
                moving: [0.01; 9],
            }
        })
        .collect()
}

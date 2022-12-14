use js_sys::Math::random;

use crate::{level::Level, log};

pub struct Game {
    level: Level,
    render_buffer: Vec<Triangle>,
    game_items: Vec<ShitItem>,

    last_fps_print: f64,
    frames_drawn: usize,
}

pub trait GameItem {
    fn into_triangle(&self) -> Triangle;
}

#[derive(Clone)]
pub struct Triangle {
    pub coords: [f32; 9],
    pub color: [f32; 4],
}

struct ShitItem {
    triangle: Triangle,
    moving: [f32; 9],
}

impl GameItem for ShitItem {
    fn into_triangle(&self) -> Triangle {
        self.triangle.clone()
    }
}

impl ShitItem {
    fn update(&mut self) {
        let tri = &mut self.triangle.coords;
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
            game_items: random_shit_items(100),
            level: Level::load_from_svg_str(include_str!("../assets/map.svg")),
            frames_drawn: 0,
            last_fps_print: 0.0,
        }
    }

    pub fn next_frame(&mut self) -> Vec<Triangle> {
        self.render_buffer.clear();
        let level_triangles = self.level.triangles();
        for tri in level_triangles {
            self.render_buffer.push(tri.clone())
        }

        // data
        for item in &mut self.game_items {
            item.update();
            self.render_buffer.push(item.into_triangle());
        }

        self.handle_fps();

        self.render_buffer.clone()
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
                t[i * 3 + 1] = (random() * 1.0 + 17.5) as f32;
            }
            ShitItem {
                triangle: Triangle {
                    coords: t,
                    color: [0.9; 4],
                },
                moving: [0.01; 9],
            }
        })
        .collect()
}

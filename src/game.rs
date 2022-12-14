use js_sys::Math::random;

use crate::level;

pub struct Game {
    level: level::Level,
    render_buffer: Vec<Triangle>,
    game_items: Vec<ShitItem>,
}

pub trait GameItem {
    fn into_triangle(&self) -> Triangle;
}

pub type Triangle = [f32; 9];

struct ShitItem {
    triangle: Triangle,
    moving: [f32; 9],
}

impl GameItem for ShitItem {
    fn into_triangle(&self) -> Triangle {
        self.triangle
    }
}

impl ShitItem {
    fn update(&mut self) {
        let tri = &mut self.triangle;
        for n in 0..tri.len() {
            let hit_edge = tri[n] >= 1.0 || tri[n] <= -1.0;

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
            game_items: random_shit_items(5),
            level: level::Level::load_from_svg("xd"),
        }
    }

    pub fn next_frame(&mut self) -> Vec<Triangle> {
        self.render_buffer.clear();
        let level_triangles = self.level.triangles();
        for tri in level_triangles {
            self.render_buffer.push(tri)
        }

        // data

        for item in &mut self.game_items {
            item.update();
            self.render_buffer.push(item.into_triangle());
        }

        self.render_buffer.clone()
    }
}

fn random_triangles(n: usize) -> Vec<Triangle> {
    (0..n)
        .map(|_| {
            let mut t: Triangle = [0.0; 9];
            for i in 0..9 {
                t[i] = (random() * 2.0 - 1.0) as f32;
            }
            t
        })
        .collect()
}

fn random_shit_items(n: usize) -> Vec<ShitItem> {
    (0..n)
        .map(|_| {
            let mut t: Triangle = [0.0; 9];
            for i in 0..9 {
                t[i] = (random() * 2.0 - 1.0) as f32;
            }
            ShitItem {
                triangle: t,
                moving: [0.001; 9],
            }
        })
        .collect()
}

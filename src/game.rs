use js_sys::Math::random;

use crate::level::Level;

pub struct Game {
    level: Level,
    render_buffer: Vec<Triangle>,
    game_items: Vec<ShitItem>,
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
            level: Level::load_from_svg_str(include_str!("../assets/map.svg")),
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

        self.render_buffer.clone()
    }
}

fn random_shit_items(n: usize) -> Vec<ShitItem> {
    (0..n)
        .map(|_| {
            let mut t = [0.0; 9];
            for i in 0..9 {
                t[i] = (random() * 2.0 - 1.0) as f32;
            }
            ShitItem {
                triangle: Triangle {
                    coords: t,
                    color: [0.5; 4],
                },
                moving: [0.001; 9],
            }
        })
        .collect()
}

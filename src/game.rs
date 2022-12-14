use js_sys::Math::random;

use crate::level;

pub struct Game {
    level: level::Level,
    triangles: Vec<Triangle>,
}

pub type Triangle = [f32; 9];

impl Game {
    pub fn new() -> Game {
        Game {
            triangles: vec![],
            level: level::Level::load_from_svg("xd"),
        }
    }

    pub fn next_frame(&mut self) -> Vec<Triangle> {
        self.triangles.clear();
        let level_triangles = self.level.triangles();
        for tri in level_triangles {
            self.triangles.push(tri)
        }

        // data
        for tri in random_triangles(5) {
            self.triangles.push(tri);
        }

        self.triangles.clone()
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

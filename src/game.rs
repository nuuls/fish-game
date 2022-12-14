use js_sys::Math::random;

use crate::level;

pub struct Game {
    level: level::Level,
    render_buffer: Vec<Triangle>,
    triangles: Vec<Triangle>,
}

pub type Triangle = [f32; 9];

impl Game {
    pub fn new() -> Game {
        Game {
            render_buffer: vec![],
            triangles: random_triangles(5),
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

        for tri in &mut self.triangles {
            for n in 0..tri.len() {
                tri[n] = tri[n] + ((random() * 2.0 - 1.0) * 0.01).clamp(-1.0, 1.0) as f32;
            }
            self.render_buffer.push(tri.clone());
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

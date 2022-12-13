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
        let coordinates: [f32; _] = [
            -0.5, -0.5, 0.0, //
            -0.5, 0.5, 0.0, //
            0.5, -0.5, 0.0, //
        ];

        self.triangles.push(coordinates);

        self.triangles.clone()
    }
}

use crate::types::{Entity, Triangle};

pub struct Player {
    id: String,
    position: (f32, f32),
    triangles: Vec<Triangle>,
}

impl Entity for Player {
    fn id(&self) -> &String {
        &self.id
    }

    fn triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }

    fn update(&mut self, time_passed: f32) {
        self.position.0 += 0.00001;
        for i in 0..3 {
            self.triangles[0].coords[i * 3] += 0.001;
        }
    }

    fn position(&self) -> Option<(f32, f32)> {
        Some(self.position)
    }
}

impl Player {
    pub fn new(position: (f32, f32)) -> Player {
        Player {
            id: "player".to_string(),
            position,
            triangles: vec![Triangle {
                coords: [
                    position.0,
                    position.1,
                    1.0,
                    position.0 + 3.0,
                    position.1,
                    1.0,
                    position.0,
                    position.1 + 2.0,
                    1.0,
                ],
                color: [1.0, 0.0, 0.0, 1.0],
            }],
        }
    }
}

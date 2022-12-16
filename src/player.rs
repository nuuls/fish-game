

use crate::{
    types::{Entity, ShaderId, Triangle},
};

pub struct Player {
    id: String,
    position: (f32, f32),
    triangles: Vec<Triangle>,

    movement: f32,
}

impl Entity for Player {
    fn id(&self) -> &String {
        &self.id
    }

    fn triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }

    fn update(&mut self, time_passed: f32) {
        self.position.0 += time_passed * self.movement * 3.0;
    }

    fn position(&self) -> (f32, f32) {
        self.position
    }

    fn on_user_input(&mut self, input: &crate::user_input::UserInput) {
        if input.move_left {
            self.movement = -1.0;
        } else if input.move_right {
            self.movement = 1.0;
        } else {
            self.movement = 0.0;
        }
    }
}

impl Player {
    pub fn new(position: (f32, f32)) -> Player {
        Player {
            id: "player".to_string(),
            position,
            triangles: vec![Triangle {
                coords: [
                    0.0, 0.0, 1.0, //
                    3.0, 0.0, 1.0, //
                    0.0, 2.0, 1.0, //
                ],
                color: [1.0, 0.0, 0.0, 1.0],
                shader_id: ShaderId::Default,
            }],
            movement: 0.0,
        }
    }
}

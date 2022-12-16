use js_sys::Math::random;

use crate::types::{Entity, GameState, ShaderId, Triangle};

pub struct Fish {
    id: String,
    race: FishRace,
    triangles: Vec<Triangle>,
    position: (f32, f32),
}

pub enum FishRace {
    Goldfish,
    Eel,
    Whale,
}

impl Fish {
    pub fn new(race: FishRace) -> Self {
        Fish {
            id: "XD".to_string(),
            race,
            position: ((random() * 20.0) as f32, (random() * 20.0) as f32),
            triangles: vec![Triangle {
                coords: [
                    0.0, 0.0, 1.0, //
                    3.0, 0.0, 1.0, //
                    0.0, 2.0, 1.0, //
                ],
                color: [0.0, 1.0, 0.0, 1.0],
                ..Default::default()
            }],
        }
    }
}

impl Entity for Fish {
    fn id(&self) -> &String {
        &self.id
    }

    fn triangles(&self) -> &Vec<crate::types::Triangle> {
        &self.triangles
    }

    fn position(&self) -> (f32, f32) {
        self.position
    }

    fn update(&mut self, _time_passed: f32, _game_state: &mut GameState) {
        self.position.0 += (random() as f32 - 0.5) * 0.1;
        self.position.1 += (random() as f32 - 0.5) * 0.1;
    }
}

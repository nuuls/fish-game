use nphysics2d::object::{DefaultBodyHandle, DefaultColliderHandle};

use crate::{
    fishing_rod::FishingRod,
    sick_physics::Physics,
    types::{red, Entity, GameState, ShaderId, Triangle},
};

pub struct Player {
    id: String,
    position: (f32, f32),
    rotation: f32,
    triangles: Vec<Triangle>,

    body_handle: Option<DefaultBodyHandle>,
    collider_handle: Option<DefaultColliderHandle>,
}

impl Entity for Player {
    fn id(&self) -> &String {
        &self.id
    }

    fn triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }

    fn update(&mut self, _time_passed: f32, gs: &mut GameState) {
        let movement: f32 = if gs.input.move_left { -1.0 } else { 0.0 }
            + if gs.input.move_right { 1.0 } else { 0.0 };

        if gs.input.throw_rod {
            gs.entity_ops.insert(FishingRod::new(self.position));
        }

        // walk
        if let Some(body) = self.body_handle.and_then(|h| gs.physics.bodies.get_mut(h)) {
            body.apply_displacement(&[movement * 0.02, 0.0, 0.0]);
        }

        // update position
        if let Some(collider) = self
            .collider_handle
            .and_then(|h| gs.physics.colliders.get_mut(h))
        {
            let translation = collider.position().translation;
            self.position = (translation.x, translation.y);
            self.rotation = collider.position().rotation.angle();
        }
    }

    fn position(&self) -> (f32, f32) {
        self.position
    }

    fn rotation(&self) -> f32 {
        self.rotation
    }

    fn init_physics(
        &mut self,
        physics: &mut Physics,
    ) -> Option<(DefaultBodyHandle, DefaultColliderHandle)> {
        let (body_handle, collider_handle) = physics.insert_cuboid(
            self.position.0 + 0.5,
            self.position.1 + 1.0,
            1.0,
            2.0,
            physics.collision_groups.player,
        );

        self.body_handle = Some(body_handle);
        self.collider_handle = Some(collider_handle);

        Some((body_handle, collider_handle))
    }
}

impl Player {
    pub fn new(position: (f32, f32)) -> Player {
        let half_width = 0.5;
        let half_height = 1.0;

        Player {
            id: "player".to_string(),
            position,
            rotation: 0.0,
            triangles: vec![
                Triangle::from_points(
                    (-half_width, -half_height),
                    (half_width, -half_height),
                    (half_width, half_height),
                    red(),
                ),
                Triangle::from_points(
                    (-half_width, -half_height),
                    (half_width, half_height),
                    (-half_width, half_height),
                    red(),
                ),
            ],
            body_handle: None,
            collider_handle: None,
        }
    }
}

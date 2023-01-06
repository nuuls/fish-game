use nphysics2d::{
    algebra::Force2,
    object::{DefaultBodyHandle, DefaultColliderHandle},
};

use crate::{
    sick_physics::Physics,
    types::{cyan, green, red, Entity, GameState, ShaderId, Triangle},
    utils::next_id,
};

const HALF_WIDTH: f32 = 0.1;
const HALF_HEIGHT: f32 = 0.1;

pub struct FishingRod {
    id: String,
    position: (f32, f32),
    rotation: f32,
    triangles: Vec<Triangle>,
    once: bool,

    body_handle: Option<DefaultBodyHandle>,
    collider_handle: Option<DefaultColliderHandle>,
}

impl Entity for FishingRod {
    fn id(&self) -> &String {
        &self.id
    }

    fn triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }

    fn update(&mut self, _time_passed: f32, gs: &mut GameState) {
        // apply force
        if self.once {
            self.once = false;
            if let Some(body) = self.body_handle.and_then(|h| gs.physics.bodies.get_mut(h)) {
                body.apply_force(
                    0,
                    &Force2::from_slice(&[0.1, 0.0, 0.0]),
                    nphysics2d::math::ForceType::Impulse,
                    true,
                );
            }
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
            self.position.0,
            self.position.1,
            HALF_WIDTH,
            HALF_HEIGHT,
            physics.collision_groups.fishing_rod,
        );

        self.body_handle = Some(body_handle);
        self.collider_handle = Some(collider_handle);

        Some((body_handle, collider_handle))
    }
}

impl FishingRod {
    pub fn new(position: (f32, f32)) -> Self {
        Self {
            id: next_id() + &"fishing_hook",
            position,
            rotation: 0.0,
            once: true,
            triangles: vec![
                Triangle::from_points(
                    (-HALF_WIDTH, -HALF_HEIGHT),
                    (HALF_WIDTH, -HALF_HEIGHT),
                    (HALF_WIDTH, HALF_HEIGHT),
                    red(),
                ),
                Triangle::from_points(
                    (-HALF_WIDTH, -HALF_HEIGHT),
                    (HALF_WIDTH, HALF_HEIGHT),
                    (-HALF_WIDTH, HALF_HEIGHT),
                    red(),
                ),
            ],
            body_handle: None,
            collider_handle: None,
        }
    }
}

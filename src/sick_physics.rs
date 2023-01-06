use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::math::Isometry;
use nphysics2d::nalgebra::Vector2;
use nphysics2d::ncollide2d::pipeline::CollisionGroups;
use nphysics2d::ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderHandle,
    DefaultColliderSet, Ground, RigidBodyDesc,
};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

use crate::types::{red, Entity, GameState, Triangle};

type F = f32;

pub struct Collisions {
    pub player: CollisionGroups,
    pub fishing_rod: CollisionGroups,
}

impl Collisions {
    pub fn new() -> Self {
        Self {
            player: CollisionGroups::new(),
            fishing_rod: CollisionGroups::new(),
        }
    }
}

pub struct Physics {
    pub id: String,
    pub triangles: Vec<Triangle>,

    pub mechanical_world: DefaultMechanicalWorld<F>,
    pub geometrical_world: DefaultGeometricalWorld<F>,
    pub bodies: DefaultBodySet<F>,
    pub colliders: DefaultColliderSet<F>,
    pub joint_constraints: DefaultJointConstraintSet<F>,
    pub force_generators: DefaultForceGeneratorSet<F>,

    pub collisions: Collisions,
}

impl Physics {
    pub fn new() -> Self {
        let mut mechanical_world = DefaultMechanicalWorld::new(Vector2::new(0.0, 9.81));
        let geometrical_world = DefaultGeometricalWorld::<F>::new();
        let bodies = DefaultBodySet::new();
        let colliders = DefaultColliderSet::new();
        let joint_constraints = DefaultJointConstraintSet::<F>::new();
        let force_generators = DefaultForceGeneratorSet::<F>::new();
        mechanical_world.set_timestep(1.0 / 60.0);

        Self {
            id: "physics".to_string(),
            triangles: vec![],

            mechanical_world,
            geometrical_world,
            bodies,
            colliders,
            joint_constraints,
            force_generators,

            collisions: Collisions::new(),
        }
    }

    pub fn step(&mut self) {
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators,
        );
    }

    pub fn insert_ground(&mut self, x: f32, y: f32, half_width: f32, half_height: f32) {
        let shape = ShapeHandle::new(Cuboid::new(Vector2::new(half_width, half_height)));
        // let body = RigidBodyDesc::new().translation(Vector2::new(x, y)).build();
        let body_handle = self.bodies.insert(Ground::new());
        let co = ColliderDesc::new(shape)
            .translation(Vector2::new(x, y))
            .build(BodyPartHandle(body_handle, 0));
        self.colliders.insert(co);
    }

    pub fn insert_cuboid(
        &mut self,
        center_x: f32,
        center_y: f32,
        width: f32,
        height: f32,
    ) -> (DefaultBodyHandle, DefaultColliderHandle) {
        let shape = ShapeHandle::new(Cuboid::new(Vector2::new(width / 2.0, height / 2.0)));
        let body = RigidBodyDesc::new()
            .translation(Vector2::new(center_x, center_y))
            .build();
        let body_handle = self.bodies.insert(body);
        let co = ColliderDesc::new(shape)
            .density(1.0)
            .build(BodyPartHandle(body_handle, 0));
        let collider_handle = self.colliders.insert(co);
        (body_handle, collider_handle)
    }
}

// fn triangulate_cuboid(cuboid: &Cuboid<F>, position: &Isometry<f32>, triangles: &mut Vec<Triangle>) {
//     let half_width = cuboid.half_extents.x;
//     let half_height = cuboid.half_extents.y;

//     // TODO: rotation
//     let (x, y) = (position.translation.x, position.translation.y);

//     triangles.push(Triangle::from_points(
//         (x - half_width, y - half_height),
//         (x + half_width, y - half_height),
//         (x + half_width, y + half_height),
//         red(),
//     ));
//     triangles.push(Triangle::from_points(
//         (x - half_width, y - half_height),
//         (x + half_width, y + half_height),
//         (x - half_width, y + half_height),
//         red(),
//     ));
// }

// impl Entity for Physics {
//     fn update(&mut self, _delta_time: f32, _game_state: &mut GameState) {
//         // self.mechanical_world.set_timestep(delta_time / 1000.0);
//         self.step();

//         self.triangles.clear();
//         for (_, collider) in self.colliders.iter() {
//             triangulate_cuboid(
//                 collider.shape().as_shape::<Cuboid<F>>().unwrap(),
//                 collider.position(),
//                 &mut self.triangles,
//             );
//         }
//     }

//     fn id(&self) -> &String {
//         &self.id
//     }

//     fn triangles(&self) -> &Vec<crate::types::Triangle> {
//         &self.triangles
//     }
// }

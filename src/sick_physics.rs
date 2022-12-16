use nphysics2d::algebra::Force2;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::math::Isometry;
use nphysics2d::nalgebra::{Point2, RealField, Vector2};
use nphysics2d::ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::object::{
    Body, BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, Ground, RigidBodyDesc,
};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

use crate::types::{red, Color, Entity, Triangle};

type F = f32;

pub struct Physics {
    id: String,
    triangles: Vec<Triangle>,

    mechanical_world: DefaultMechanicalWorld<F>,
    geometrical_world: DefaultGeometricalWorld<F>,
    bodies: DefaultBodySet<F>,
    colliders: DefaultColliderSet<F>,
    joint_constraints: DefaultJointConstraintSet<F>,
    force_generators: DefaultForceGeneratorSet<F>,
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
        }
    }

    fn step(&mut self) {
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

    pub fn insert_cube(&mut self, x: f32, y: f32, size: f32) {
        let shape = ShapeHandle::new(Cuboid::new(Vector2::new(size / 2.0, size / 2.0)));
        let body = RigidBodyDesc::new().translation(Vector2::new(x, y)).build();
        let body_handle = self.bodies.insert(body);
        let co = ColliderDesc::new(shape)
            .density(1.0)
            .build(BodyPartHandle(body_handle, 0));
        self.colliders.insert(co);
    }
}

fn triangulate_cuboid(cuboid: &Cuboid<F>, position: &Isometry<f32>, triangles: &mut Vec<Triangle>) {
    let half_width = cuboid.half_extents.x;
    let half_height = cuboid.half_extents.y;

    // TODO: rotation
    let (x, y) = (position.translation.x, position.translation.y);

    triangles.push(Triangle::from_points(
        (x - half_width, y - half_height),
        (x + half_width, y - half_height),
        (x + half_width, y + half_height),
        red(),
    ));
    triangles.push(Triangle::from_points(
        (x - half_width, y - half_height),
        (x + half_width, y + half_height),
        (x - half_width, y + half_height),
        red(),
    ));
}

impl Entity for Physics {
    fn update(&mut self, delta_time: f32) {
        // self.mechanical_world.set_timestep(delta_time / 1000.0);
        self.step();

        self.triangles.clear();
        for (_, collider) in self.colliders.iter() {
            triangulate_cuboid(
                collider.shape().as_shape::<Cuboid<F>>().unwrap(),
                collider.position(),
                &mut self.triangles,
            );
        }
    }

    fn id(&self) -> &String {
        &self.id
    }

    fn triangles(&self) -> &Vec<crate::types::Triangle> {
        &self.triangles
    }
}

pub fn init_world() {
    // /*
    //  * World
    //  */
    // let mechanical_world = DefaultMechanicalWorld::new(Vector2::new(0.0, -9.81));
    // let geometrical_world = DefaultGeometricalWorld::<f32>::new();
    // let mut bodies = DefaultBodySet::new();
    // let mut colliders = DefaultColliderSet::new();
    // let joint_constraints = DefaultJointConstraintSet::<f32>::new();
    // let force_generators = DefaultForceGeneratorSet::<f32>::new();

    // /*
    //  * Ground
    //  */
    // let ground_size = 25.0;
    // let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(ground_size, 1.0)));

    // let ground_handle = bodies.insert(Ground::new());
    // let co = ColliderDesc::new(ground_shape)
    //     .translation(-Vector2::y())
    //     .build(BodyPartHandle(ground_handle, 0));
    // colliders.insert(co);

    /*
     * Create the boxes
     */
    //     let num = 10;
    //     let rad = 0.1;

    //     let cuboid = ShapeHandle::new(Cuboid::new(Vector2::repeat(rad)));

    //     let shift = (rad + ColliderDesc::<f32>::default_margin()) * (2.0);
    //     let centerx = shift * (num as f32) / 2.0;
    //     let centery = shift / (2.0);

    //     for i in 0usize..num {
    //         for j in 0..num {
    //             let x = (i as f32) * shift - centerx;
    //             let y = (j as f32) * shift + centery;

    //             // Build the rigid body.
    //             let rb = RigidBodyDesc::new().translation(Vector2::new(x, y)).build();
    //             let rb_handle = bodies.insert(rb);

    //             // Build the collider.
    //             let co = ColliderDesc::new(cuboid.clone())
    //                 .density(1.0)
    //                 .build(BodyPartHandle(rb_handle, 0));
    //             colliders.insert(co);
    //         }
    //     }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sick_physics() {
        let mut physics = Physics::new();

        physics.insert_ground(0.0, 0.0, 25.0, 1.0);
        physics.insert_cube(0.0, 10.0, 1.0);
        for _ in 0..20 {
            physics.step();

            // let mut it = physics.bodies.iter();
            // let first = it.next().unwrap();
            // let second = it.next().unwrap();
            // drop(it);

            // println!(
            //     "{:?}, {:?}",
            //     first.1.e
            //     second.1.position().translation.vector
            // );

            let mut it = physics.colliders.iter();

            let first = it.next().unwrap();
            let second = it.next().unwrap();
            drop(it);

            println!(
                "{:?}, {:?}",
                first.1.position().translation.vector,
                second.1.position().translation.vector
            );

            // physics.bodies.iter_mut().next().map(|b| {
            //     b.1.apply_force(
            //         0,
            //         &Force2::from_slice(&[1.0, 0.0]),
            //         nphysics2d::math::ForceType::Impulse,
            //         true,
            //     )
            // });
        }

        // let mut testbed = Testbed::<f32>::from_builders(0, vec![("Boxes", init_world)]);
        // testbed.run()
    }
}

// fn main() {
//     let testbed = Testbed::<f32>::from_builders(0, vec![("Boxes", init_world)]);
//     testbed.run()
// }

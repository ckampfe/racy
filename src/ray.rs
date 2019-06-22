use nalgebra::{Matrix4, Point3, Vector3};

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Self {
        Ray { origin, direction }
    }

    /// see https://www.nalgebra.org/points_and_transformations/
    pub fn position(&self, t: f32) -> Point3<f32> {
        Point3::from_homogeneous(
            self.origin.to_homogeneous() + (self.direction * t).to_homogeneous(),
        )
        .unwrap()
    }

    pub fn transform_mut(&mut self, m: Matrix4<f32>) -> () {
        let origin = m * self.origin.to_homogeneous();
        let direction = m * self.direction.to_homogeneous();

        self.origin = Point3::from_homogeneous(origin).unwrap();
        self.direction = Vector3::from_homogeneous(direction).unwrap();
    }
}

use nalgebra::{Point3, Vector3};

pub trait Normal {
    fn normal_at(&self, point: Point3<f32>) -> Vector3<f32>;
}

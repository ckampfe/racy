use nalgebra::{Point3, Vector3};

pub trait LocalNormal {
    fn local_normal_at(&self, point: Point3<f32>) -> Vector3<f32>;
}

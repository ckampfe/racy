use crate::intersection::Intersection;
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::{Matrix4, Point3, Projective3, Transform, Vector3};

pub trait Shape {
    fn material(&self) -> Material;
    fn transform(&self) -> Matrix4<f32>;
    fn normal_at(&self, point: Point3<f32>) -> Vector3<f32>;
    fn local_intersect(&self, ray: Ray) -> Vec<Box<Intersection>>;
    fn local_normal_at(&self, point: Point3<f32>) -> Vector3<f32>;

    fn intersect(&self, ray: &Ray) -> Vec<Box<Intersection>> {
        let projective_inverse: Projective3<f32> =
            Transform::from_matrix_unchecked(self.transform()).inverse();
        let local_ray = ray.transform(projective_inverse.to_homogeneous());

        self.local_intersect(local_ray)
    }
}

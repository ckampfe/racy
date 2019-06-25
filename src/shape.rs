use crate::intersection::Intersection;
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::{Point3, Vector3};

pub trait Shape {
    fn material(&self) -> Material;
    fn intersect(&self, ray: &Ray) -> Vec<Box<Intersection>>;
    fn normal_at(&self, point: Point3<f32>) -> Vector3<f32>;
}

use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::shape::Shape;

pub trait Intersect<T: Shape> {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection<T>>;
}

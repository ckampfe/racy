use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::shape::Shape;

pub trait LocalIntersect {
    fn local_intersect(&self, ray: Ray) -> Vec<Box<Intersection>>;
}

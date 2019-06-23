use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::shape::Shape;

pub trait LocalIntersect<T: Clone + Shape> {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection<T>>;
}

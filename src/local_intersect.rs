use crate::ray::Ray;
use crate::intersection::Intersection;

pub trait LocalIntersect<T: Clone> {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection<T>>;
}

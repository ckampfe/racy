use crate::intersection::Intersection;
use crate::ray::Ray;

pub trait LocalIntersect<T: Clone> {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection<T>>;
}

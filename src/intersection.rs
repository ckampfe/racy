use crate::ray::Ray;
use crate::shape::Shape;
use nalgebra::{Point3, Vector3};
use std::{cmp::Ordering, sync::Arc};

pub type PreparedComputations<'a, T> = (
    f32,
    T,
    Point3<f32>,
    Vector3<f32>,
    Vector3<f32>,
    bool,
    Point3<f32>,
);

pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub fn new<T: 'a + Shape>(t: f32, object: &'a T) -> Self {
        Intersection { t, object }
    }

    pub fn aggregate(intersections: Vec<Intersection>) -> Vec<Intersection> {
        intersections
    }

    pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
        let mut positive_intersections: Vec<Intersection> = intersections
            .into_iter()
            .filter(|i| i.t >= 0.0)
            .collect::<Vec<Intersection>>();

        positive_intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal));

        // TODO: fix this nonsense
        if !positive_intersections.is_empty() {
            Some(positive_intersections.swap_remove(0))
        } else {
            None
        }
    }

    pub fn prepare_computations(&self, ray: &Ray) -> PreparedComputations<&dyn Shape> {
        let point = ray.position(self.t);
        let eyev = ray.direction * -1.0;
        let normalv = (*self.object).normal_at(point);

        let (inside, normalv) = if normalv.dot(&eyev) < 0.0 {
            (true, normalv * -1.0)
        } else {
            (false, normalv)
        };

        let over_point = point + normalv * 0.00001;

        (
            self.t,
            self.object,
            point,
            eyev,
            normalv,
            inside,
            over_point,
        )
    }
}

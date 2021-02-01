use crate::ray::Ray;
use crate::shape::Shape;
use nalgebra::{Point3, Vector3};
use std::cmp::Ordering;

pub struct PreparedComputations<'p> {
    pub t: f32,
    pub object: &'p dyn Shape,
    pub point: Point3<f32>,
    pub eyev: Vector3<f32>,
    pub normalv: Vector3<f32>,
    pub inside: bool,
    pub over_point: Point3<f32>,
}

pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub fn new<T: 'a + Shape>(t: f32, object: &'a T) -> Self {
        Intersection { t, object }
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

    pub fn prepare_computations(&self, ray: &Ray) -> PreparedComputations {
        let point = ray.position(self.t);
        let eyev = ray.direction * -1.0;
        let normalv = self.object.normal_at(point);

        let (inside, normalv) = if normalv.dot(&eyev) < 0.0 {
            (true, normalv * -1.0)
        } else {
            (false, normalv)
        };

        let over_point = point + normalv * 0.00001;

        PreparedComputations {
            t: self.t,
            object: self.object,
            point,
            eyev,
            normalv,
            inside,
            over_point,
        }
    }
}

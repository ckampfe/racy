use crate::normal::Normal;
use crate::ray::Ray;
use crate::shape::Shape;
use nalgebra::{Point3, Vector3};
use std::cmp::Ordering;

pub type PreparedComputations<'a, T> = (
    f32,
    &'a T,
    Point3<f32>,
    Vector3<f32>,
    Vector3<f32>,
    bool,
    Point3<f32>,
);

#[derive(Clone, Debug)]
pub struct Intersection<T: Shape> {
    pub t: f32,
    pub object: T,
}

impl<T> Intersection<T>
where
    T: Clone + Normal + Shape,
{
    pub fn new(t: f32, object: T) -> Self {
        Intersection { t, object }
    }

    pub fn aggregate(intersections: Vec<Intersection<T>>) -> Vec<Intersection<T>> {
        intersections
    }

    pub fn hit(intersections: Vec<Intersection<T>>) -> Option<Intersection<T>> {
        let mut positive_intersections = intersections
            .iter()
            .filter(|i| i.t >= 0.0)
            .collect::<Vec<&Intersection<T>>>();

        positive_intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal));

        if positive_intersections.is_empty() {
            None
        } else {
            Some((*positive_intersections[0]).clone())
        }
    }

    pub fn prepare_computations(&self, ray: &Ray) -> PreparedComputations<T> {
        let point = ray.position(self.t);
        let eyev = ray.direction * -1.0;
        let normalv = self.object.normal_at(point);

        let (inside, normalv) = if normalv.dot(&eyev) < 0.0 {
            (true, normalv * -1.0)
        } else {
            (false, normalv)
        };

        let over_point = point + normalv * 0.00001;

        (
            self.t,
            &self.object,
            point,
            eyev,
            normalv,
            inside,
            over_point,
        )
    }
}

use std::sync::Arc;

use nalgebra::Point3;

use crate::{intersection::Intersection, shape::Shape, triangle};

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

impl AABB {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn all() -> Self {
        Self {
            min: Point3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY),
            max: Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
        }
    }

    pub fn merge(&self, other: AABB) -> AABB {
        let min = if self.min <= other.min {
            self.min
        } else {
            other.min
        };

        let max = if self.max >= other.max {
            self.max
        } else {
            other.max
        };

        AABB { min, max }
    }
}

impl BoundingBox for AABB {
    fn bounding_box(&self) -> AABB {
        *self
    }
}

impl Shape for AABB {
    fn material(&self) -> crate::material::Material {
        todo!()
    }

    fn transform(&self) -> nalgebra::Matrix4<f32> {
        todo!()
    }

    fn normal_at(&self, point: Point3<f32>) -> nalgebra::Vector3<f32> {
        todo!()
    }

    fn local_intersect(&self, ray: crate::ray::Ray) -> Vec<crate::intersection::Intersection> {
        let mut tmin = (self.min.x - ray.origin.x) / ray.direction.x;
        let mut tmax = (self.max.x - ray.origin.x) / ray.direction.x;

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (self.min.y - ray.origin.y) / ray.direction.y;
        let mut tymax = (self.max.y - ray.origin.y) / ray.direction.y;

        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if (tmin > tymax) || (tymin > tmax) {
            // return false;
            return vec![];
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (self.min.z - ray.origin.z) / ray.direction.z;
        let mut tzmax = (self.max.z - ray.origin.z) / ray.direction.z;

        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            // return false;
            return vec![];
        }

        // if tzmin > tmin {
        //     tmin = tzmin;
        // }

        // if tzmax < tmax {
        //     tmax = tzmax;
        // }

        // return true;
        vec![crate::intersection::Intersection::new(0.0, self)]
    }

    fn local_normal_at(&self, point: Point3<f32>) -> nalgebra::Vector3<f32> {
        todo!()
    }
}

pub trait BoundingBox {
    fn bounding_box(&self) -> AABB;
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            min: Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: Point3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY),
        }
    }
}

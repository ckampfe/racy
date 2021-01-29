use std::sync::Arc;

use crate::aabb::{BoundingBox, AABB};
use nalgebra::{Matrix4, Point3, Vector3};
// use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{material::Material, shape::Shape};

pub struct Group {
    shapes: Vec<Arc<dyn Shape + Send + Sync>>,
    bounding_box: AABB,
    material: Material,
    transform: Matrix4<f32>,
}

impl Group {
    pub fn new(shapes: Vec<Arc<dyn Shape + Send + Sync>>) -> Self {
        let transform = Matrix4::identity();
        let material = Material::default();

        let mut aabb = AABB::default();

        for shape in shapes.iter() {
            aabb = aabb.merge(shape.bounding_box());
        }

        Group {
            shapes,
            bounding_box: aabb,
            material,
            transform,
        }
    }
}

impl BoundingBox for Group {
    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}

impl Shape for Group {
    fn material(&self) -> crate::material::Material {
        self.material
    }

    fn transform(&self) -> nalgebra::Matrix4<f32> {
        self.transform
    }

    fn normal_at(&self, point: nalgebra::Point3<f32>) -> nalgebra::Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }

    fn local_intersect(&self, ray: crate::ray::Ray) -> Vec<crate::intersection::Intersection> {
        // if self.bounding_box.local_intersect(ray).len() > 0 {
        let intersections = self
            .shapes
            .iter()
            .map(|shape| shape.local_intersect(ray))
            .collect::<Vec<Vec<_>>>();

        intersections.into_iter().flatten().collect::<Vec<_>>()
        // } else {
        //     vec![]
        // }
    }

    fn local_normal_at(&self, point: nalgebra::Point3<f32>) -> nalgebra::Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

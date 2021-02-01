use crate::bounding_box::{BoundingBox, AABB};
use crate::cube::Cube;
use nalgebra::{Matrix4, Vector3};

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{material::Material, shape::Shape};

pub struct DynGroup<S: Shape + Sized> {
    shapes: Vec<S>,
    bounding_box: AABB,
    material: Material,
    transform: Matrix4<f32>,
}

impl<S: Shape + Sized> DynGroup<S> {
    pub fn new(shapes: Vec<S>) -> Self {
        let transform = Matrix4::identity();
        let material = Material::default();

        let mut aabb = AABB::default();

        for shape in shapes.iter() {
            aabb.merge_mut(shape.bounding_box());
        }

        DynGroup {
            shapes,
            bounding_box: aabb,
            material,
            transform,
        }
    }
}

impl<S: Shape + Sized> BoundingBox for DynGroup<S> {
    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}

impl<S: Shape + Sized> Shape for DynGroup<S> {
    fn material(&self) -> crate::material::Material {
        self.material
    }

    fn transform(&self) -> nalgebra::Matrix4<f32> {
        self.transform
    }

    fn normal_at(&self, _point: nalgebra::Point3<f32>) -> nalgebra::Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }

    fn local_intersect(&self, ray: crate::ray::Ray) -> Vec<crate::intersection::Intersection> {
        let cube: Cube = self.bounding_box.into();

        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        if !cube.local_intersect(ray).is_empty() {
            let intersections = self
                .shapes
                .par_iter()
                .map(|shape| shape.local_intersect(ray))
                .collect::<Vec<Vec<_>>>();

            intersections.into_iter().flatten().collect::<Vec<_>>()
        } else {
            vec![]
        }
        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        if !cube.local_intersect(ray).is_empty() {
            let intersections = self
                .shapes
                .iter()
                .map(|shape| shape.local_intersect(ray))
                .collect::<Vec<Vec<_>>>();

            intersections.into_iter().flatten().collect::<Vec<_>>()
        } else {
            vec![]
        }
    }

    fn local_normal_at(&self, _point: nalgebra::Point3<f32>) -> nalgebra::Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

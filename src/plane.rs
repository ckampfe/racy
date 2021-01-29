use crate::ray::Ray;
use crate::shape::Shape;
use crate::{aabb::BoundingBox, intersection::Intersection};
use crate::{aabb::AABB, material::Material};
use nalgebra::{Matrix4, Point3, Projective3, Transform, Vector3};

#[derive(Clone, Copy)]
pub struct Plane {
    transform: Matrix4<f32>,
    material: Material,
    bounding_box: AABB,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            transform: Matrix4::identity(),
            material: Material::default(),
            bounding_box: AABB::all(),
        }
    }
}

impl BoundingBox for Plane {
    fn bounding_box(&self) -> crate::aabb::AABB {
        self.bounding_box
    }
}

impl Shape for Plane {
    fn material(&self) -> Material {
        self.material
    }
    fn transform(&self) -> Matrix4<f32> {
        self.transform
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        if ray.direction.y.abs() < 0.00001 {
            vec![]
        } else {
            let t = -ray.origin.y / ray.direction.y;
            vec![Intersection::new(t, self)]
        }
    }

    fn normal_at(&self, point: Point3<f32>) -> Vector3<f32> {
        let transformed_transform: Projective3<f32> =
            Transform::from_matrix_unchecked(self.transform);
        let local_point: Point3<f32> = transformed_transform.inverse_transform_point(&point);
        let local_normal = self.local_normal_at(local_point);
        let transposed_transform: Projective3<f32> =
            Transform::from_matrix_unchecked(self.transform.transpose());
        let world_normal = transposed_transform.inverse_transform_vector(&local_normal);
        let mut world_normal_homogeneous = world_normal.to_homogeneous();

        world_normal_homogeneous.w = 0.0;

        Vector3::from_homogeneous(world_normal_homogeneous)
            .unwrap()
            .normalize()
    }

    fn local_normal_at(&self, _point: Point3<f32>) -> Vector3<f32> {
        Vector3::new(0.0, 1.0, 0.0)
    }
}

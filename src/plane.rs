// use crate::intersect::Intersect;
use crate::intersection::Intersection;
use crate::local_intersect::LocalIntersect;
use crate::local_normal::LocalNormal;
use crate::material::Material;
use crate::normal::Normal;
use crate::ray::Ray;
use crate::shape::Shape;
use nalgebra::{Matrix4, Point3, Projective3, Transform, Vector3};

#[derive(Clone, Copy)]
pub struct Plane {
    transform: Matrix4<f32>,
    material: Material,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            transform: Matrix4::identity(),
            material: Material::default(),
        }
    }
}

impl Shape for Plane {
    fn material(&self) -> Material {
        self.material
    }
    fn intersect(&self, ray: &Ray) -> Vec<Box<Intersection>> {
        let projective_inverse: Projective3<f32> =
            Transform::from_matrix_unchecked(self.transform).inverse();
        let local_ray = ray.transform(projective_inverse.to_homogeneous());

        self.local_intersect(local_ray)
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
}

impl LocalNormal for Plane {
    fn local_normal_at(&self, _point: Point3<f32>) -> Vector3<f32> {
        Vector3::new(0.0, 1.0, 0.0)
    }
}

impl LocalIntersect for Plane {
    fn local_intersect(&self, ray: Ray) -> Vec<Box<Intersection>> {
        if ray.direction.y.abs() < 0.00001 {
            vec![]
        } else {
            let t = -ray.origin.y / ray.direction.y;
            vec![Box::new(Intersection::new(t, *self))]
        }
    }
}

// impl Normal for Plane {
//     fn normal_at(&self, point: Point3<f32>) -> Vector3<f32> {
//         let transformed_transform: Projective3<f32> =
//             Transform::from_matrix_unchecked(self.transform);
//         let local_point: Point3<f32> = transformed_transform.inverse_transform_point(&point);
//         let local_normal = self.local_normal_at(local_point);
//         let transposed_transform: Projective3<f32> =
//             Transform::from_matrix_unchecked(self.transform.transpose());
//         let world_normal = transposed_transform.inverse_transform_vector(&local_normal);
//         let mut world_normal_homogeneous = world_normal.to_homogeneous();
// 
//         world_normal_homogeneous.w = 0.0;
// 
//         Vector3::from_homogeneous(world_normal_homogeneous)
//             .unwrap()
//             .normalize()
//     }
// }

// impl Intersect for Plane {
//     fn intersect<T: Shape>(&self, ray: &Ray) -> Vec<Intersection> {
//         let projective_inverse: Projective3<f32> =
//             Transform::from_matrix_unchecked(self.transform).inverse();
//         let local_ray = ray.transform(projective_inverse.to_homogeneous());
// 
//         self.local_intersect(local_ray)
//     }
// }

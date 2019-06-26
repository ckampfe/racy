use crate::intersection::Intersection;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Shape;
use nalgebra::{Matrix4, Point3, Projective3, Transform, Vector3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub origin: Point3<f32>,
    pub radius: f32,
    pub transform: Matrix4<f32>,
    pub material: Material,
}

impl Shape for Sphere {
    fn material(&self) -> Material {
        self.material
    }

    fn transform(&self) -> Matrix4<f32> {
        self.transform
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

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - self.origin;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let disc_sqrt = discriminant.sqrt();
            let t1 = -b - disc_sqrt / (2.0 * a);
            let t2 = -b + disc_sqrt / (2.0 * a);
            vec![Intersection::new(t1, self), Intersection::new(t2, self)]
        }
    }

    fn local_normal_at(&self, point: Point3<f32>) -> Vector3<f32> {
        point.coords
    }
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            origin: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Matrix4::identity(),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_spheres_default_transformation() {}

    #[test]
    fn changing_a_spheres_transformation() {}

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {}

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {}

    #[test]
    fn the_hit_should_offset_the_point() {}
}

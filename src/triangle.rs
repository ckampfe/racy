use crate::intersect::Intersect;
use crate::intersection::Intersection;
use crate::local_intersect::LocalIntersect;
use crate::local_normal::LocalNormal;
use crate::material::Material;
use crate::normal::Normal;
use crate::ray::Ray;
use crate::shape::Shape;
use nalgebra::{Matrix4, Point3, Projective3, Transform, Vector3};

#[derive(Clone, Copy)]
pub struct Triangle {
    pub p1: Point3<f32>,
    pub p2: Point3<f32>,
    pub p3: Point3<f32>,
    pub e1: Vector3<f32>,
    pub e2: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub transform: Matrix4<f32>,
    pub material: Material,
}

impl Triangle {
    pub fn new(p1: Point3<f32>, p2: Point3<f32>, p3: Point3<f32>) -> Triangle {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = e2.cross(&e1).normalize();
        let transform = Matrix4::identity();
        let material = Material::default();

        Triangle {
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
            transform,
            material,
        }
    }
}

impl Shape for Triangle {
    fn material(&self) -> Material {
        self.material
    }
}

impl Intersect<Triangle> for Triangle {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection<Self>> {
        let projective_inverse: Projective3<f32> =
            Transform::from_matrix_unchecked(self.transform).inverse();
        let local_ray = ray.transform(projective_inverse.to_homogeneous());

        self.local_intersect(local_ray)
    }
}

impl Normal for Triangle {
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

impl LocalNormal for Triangle {
    fn local_normal_at(&self, _point: Point3<f32>) -> Vector3<f32> {
        self.normal
    }
}

impl LocalIntersect<Triangle> for Triangle {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection<Triangle>> {
        let dir_cross_e2 = ray.direction.cross(&self.e2);
        let det = self.e1.dot(&dir_cross_e2);

        if det.abs() < 0.00001 {
            vec![]
        } else {
            let f = 1.0 / det;

            let p1_to_origin = ray.origin - self.p1;

            let u = f * p1_to_origin.dot(&dir_cross_e2);

            if u < 0.0 || u > 1.0 {
                vec![]
            } else {
                let origin_cross_e1 = p1_to_origin.cross(&self.e1);

                let v = f * ray.direction.dot(&origin_cross_e1);

                if v < 0.0 || u + v > 1.0 {
                    vec![]
                } else {
                    let t = f * self.e2.dot(&origin_cross_e1);
                    vec![Intersection::new(t, *self)]
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{Point3, Vector3};

    #[test]
    fn constructing_a_triangle() {
        let p1 = Point3::new(0.0, 1.0, 0.0);
        let p2 = Point3::new(-1.0, 0.0, 0.0);
        let p3 = Point3::new(1.0, 0.0, 0.0);
        let t = Triangle::new(p1, p2, p3);
        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p2);
        assert_eq!(t.p3, p3);
        assert_eq!(t.e1, Vector3::new(-1.0, -1.0, 0.0));
        assert_eq!(t.e2, Vector3::new(1.0, -1.0, 0.0));
        assert_eq!(t.normal, Vector3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn finding_the_normal_on_a_triangle() {
        let t = Triangle::new(
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        );

        let n1 = t.local_normal_at(Point3::new(0.0, 0.5, 0.0));
        let n2 = t.local_normal_at(Point3::new(-0.5, 0.75, 0.0));
        let n3 = t.local_normal_at(Point3::new(0.5, 0.25, 0.0));
        assert_eq!(n1, t.normal);
        assert_eq!(n2, t.normal);
        assert_eq!(n3, t.normal);
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let t = Triangle::new(
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        );

        let r = Ray::new(Point3::new(0.0, -1.0, -2.0), Vector3::new(0.0, 1.0, 0.0));
        let xs = t.local_intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_misses_the_p1_p3_edge() {
        let t = Triangle::new(
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        );
        let r = Ray::new(Point3::new(1.0, 1.0, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let xs = t.local_intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_misses_the_p1_p2_edge() {
        let t = Triangle::new(
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        );
        let r = Ray::new(Point3::new(-1.0, 1.0, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let xs = t.local_intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_misses_the_p2_p3_edge() {
        let t = Triangle::new(
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        );
        let r = Ray::new(Point3::new(0.0, -1.0, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let xs = t.local_intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let t = Triangle::new(
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(-1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        );

        let r = Ray::new(Point3::new(0.0, 0.5, -2.0), Vector3::new(0.0, 0.0, 1.0));
        let xs = t.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 2.0);
    }
}

use nalgebra::{Point3, Vector3};

use crate::{
    bounding_box::{BoundingBox, AABB},
    intersection::Intersection,
    ray::Ray,
    shape::Shape,
};

#[derive(Clone, Copy, Debug)]
pub struct Cube {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

impl Default for Cube {
    fn default() -> Self {
        Self {
            min: Point3::new(-1.0, -1.0, -1.0),
            max: Point3::new(1.0, 1.0, 1.0),
        }
    }
}

impl From<AABB> for Cube {
    fn from(aabb: AABB) -> Cube {
        Cube {
            min: aabb.min,
            max: aabb.max,
        }
    }
}

impl BoundingBox for Cube {
    fn bounding_box(&self) -> crate::bounding_box::AABB {
        let mut aabb = AABB::new();
        aabb.min = self.min;
        aabb.max = self.max;
        aabb
    }
}

impl Shape for Cube {
    fn material(&self) -> crate::material::Material {
        todo!()
    }

    fn transform(&self) -> nalgebra::Matrix4<f32> {
        todo!()
    }

    fn normal_at(&self, _point: Point3<f32>) -> nalgebra::Vector3<f32> {
        todo!()
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x, self.min.x, self.max.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y, self.min.y, self.max.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z, self.min.z, self.max.z);

        let tmin = [xtmin, ytmin, ztmin]
            .iter()
            .fold(-f32::INFINITY, |a, b| a.max(*b));
        let tmax = [xtmax, ytmax, ztmax]
            .iter()
            .fold(f32::INFINITY, |a, b| a.min(*b));

        if tmin > tmax {
            return vec![];
        }

        vec![Intersection::new(tmin, self), Intersection::new(tmax, self)]
    }

    fn local_normal_at(&self, point: Point3<f32>) -> Vector3<f32> {
        let maxc = [point.x.abs(), point.y.abs(), point.z.abs()]
            .iter()
            .fold(-f32::INFINITY, |a, b| a.max(*b));

        if (maxc - point.x.abs()).abs() < f32::EPSILON {
            Vector3::new(point.x, 0.0, 0.0)
        } else if (maxc - point.y.abs()).abs() < f32::EPSILON {
            Vector3::new(0.0, point.y, 0.0)
        } else {
            Vector3::new(0.0, 0.0, point.z)
        }
    }
}

fn check_axis(origin: f32, direction: f32, axis_min: f32, axis_max: f32) -> (f32, f32) {
    let tmin_numerator = axis_min - origin;
    let tmax_numerator = axis_max - origin;

    let (mut tmin, mut tmax) = if direction.abs() >= f32::EPSILON {
        (tmin_numerator / direction, tmax_numerator / direction)
    } else {
        (
            tmin_numerator * f32::INFINITY,
            tmax_numerator * f32::INFINITY,
        )
    };

    if tmin > tmax {
        std::mem::swap(&mut tmin, &mut tmax);
    }

    (tmin, tmax)
}

#[cfg(test)]
mod test {
    use nalgebra::Vector3;

    use crate::ray::Ray;

    use super::*;

    #[test]
    fn a_ray_intersects_a_cube() {
        let inputs = vec![
            (
                "+x",
                Point3::new(5.0, 0.5, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
                4.0,
                6.0,
            ),
            (
                "-x",
                Point3::new(-5.0, 0.5, 0.0),
                Vector3::new(1.0, 0.0, 0.0),
                4.0,
                6.0,
            ),
            (
                "+y",
                Point3::new(0.5, 5.0, 0.0),
                Vector3::new(0.0, -1.0, 0.0),
                4.0,
                6.0,
            ),
            (
                "-y",
                Point3::new(0.5, -5.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                4.0,
                6.0,
            ),
            (
                "+z",
                Point3::new(0.5, 0.0, 5.0),
                Vector3::new(0.0, 0.0, -1.0),
                4.0,
                6.0,
            ),
            (
                "-z",
                Point3::new(0.5, 0.0, -5.0),
                Vector3::new(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                "inside",
                Point3::new(0.0, 0.5, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                -1.0,
                1.0,
            ),
        ];

        let cube = Cube::default();

        for (_, origin, direction, t1, t2) in inputs {
            let ray = Ray::new(origin, direction);
            let xs = cube.local_intersect(ray);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, t1);
            assert_eq!(xs[1].t, t2);
        }
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let inputs = vec![
            (
                Point3::new(-2.0, 0.0, 0.0),
                Vector3::new(0.2673, 0.5345, 0.8018),
            ),
            (
                Point3::new(0.0, -2.0, 0.0),
                Vector3::new(0.8018, 0.2673, 0.5345),
            ),
            (
                Point3::new(0.0, 0.0, -2.0),
                Vector3::new(0.5345, 0.8018, 0.2673),
            ),
            (Point3::new(2.0, 0.0, 2.0), Vector3::new(0.0, 0.0, -1.0)),
            (Point3::new(0.0, 2.0, 2.0), Vector3::new(0.0, -1.0, 0.0)),
            (Point3::new(2.0, 2.0, 0.0), Vector3::new(-1.0, 0.0, 0.0)),
        ];

        let cube = Cube::default();

        for (origin, direction) in inputs {
            let ray = Ray::new(origin, direction);
            let xs = cube.local_intersect(ray);
            assert_eq!(xs.len(), 0)
        }
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let inputs = vec![
            (Point3::new(1.0, 0.5, -0.8), Vector3::new(1.0, 0.0, 0.0)),
            (Point3::new(-1.0, -0.2, 0.9), Vector3::new(-1.0, 0.0, 0.0)),
            (Point3::new(-0.4, 1.0, -0.1), Vector3::new(0.0, 1.0, 0.0)),
            (Point3::new(0.3, -1.0, -0.7), Vector3::new(0.0, -1.0, 0.0)),
            (Point3::new(-0.6, 0.3, 1.0), Vector3::new(0.0, 0.0, 1.0)),
            (Point3::new(0.4, 0.4, -1.0), Vector3::new(0.0, 0.0, -1.0)),
            (Point3::new(1.0, 1.0, 1.0), Vector3::new(1.0, 0.0, 0.0)),
            (Point3::new(-1.0, -1.0, -1.0), Vector3::new(-1.0, 0.0, 0.0)),
        ];
        let c = Cube::default();
        for (point, expected_normal) in inputs {
            let normal = c.local_normal_at(point);
            assert_eq!(normal, expected_normal)
        }
    }
}

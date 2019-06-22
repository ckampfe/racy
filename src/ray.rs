use nalgebra::{Matrix4, Point3, Vector3};

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Self {
        Ray { origin, direction }
    }

    /// see https://www.nalgebra.org/points_and_transformations/
    pub fn position(&self, t: f32) -> Point3<f32> {
        Point3::from_homogeneous(
            self.origin.to_homogeneous() + (self.direction * t).to_homogeneous(),
        )
        .unwrap()
    }

    pub fn transform_mut(&mut self, m: Matrix4<f32>) -> () {
        let origin = m * self.origin.to_homogeneous();
        let direction = m * self.direction.to_homogeneous();

        self.origin = Point3::from_homogeneous(origin).unwrap();
        self.direction = Vector3::from_homogeneous(direction).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vector3::new(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(Point3::new(2.0, 3.0, 4.0), Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), Point3::new(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Point3::new(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Point3::new(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Point3::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {}

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {}

    #[test]
    fn a_ray_misses_a_sphere() {}

    #[test]
    fn a_ray_originates_inside_a_sphere() {}

    #[test]
    fn a_sphere_is_behind_a_ray() {}

    #[test]
    fn an_intersection_encapsulates_t_and_object() {}

    #[test]
    fn aggregating_intersections() {}

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {}

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {}

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {}

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {}

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {}

    #[test]
    fn translating_a_ray() {}

    #[test]
    fn scaling_a_ray() {}

    /*
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
    */
}

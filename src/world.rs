use nalgebra::{Matrix4, Point3, Vector3};

use std::cmp::Ordering;

use crate::intersection::{Intersection, PreparedComputations};
use crate::light::Light;
use crate::material::Material;
use crate::plane::Plane;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::Sphere;

pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
    pub light: Light,
}

impl World {
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = self
            .objects
            .iter()
            .flat_map(|object| object.intersect(&ray))
            .collect();

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal));

        intersections
    }

    fn shade_hit(&self, comps: PreparedComputations) -> Vector3<f32> {
        Light::lighting(
            comps.object.material(),
            self.light,
            comps.over_point,
            comps.eyev,
            comps.normalv,
            self.is_shadowed(comps.over_point),
        )
    }

    pub fn color_at(&self, ray: Ray) -> Vector3<f32> {
        let intersections = self.intersect(ray);
        let intersection = Intersection::hit(intersections);

        if let Some(i) = intersection {
            let comps = i.prepare_computations(&ray);
            self.shade_hit(comps)
        } else {
            Vector3::new(0.0, 0.0, 0.0) // black
        }
    }

    fn is_shadowed(&self, point: Point3<f32>) -> bool {
        let v = self.light.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let ray = Ray::new(point, direction);
        let intersections = self.intersect(ray);
        let hit = Intersection::hit(intersections);

        if let Some(h) = hit {
            h.t < distance
        } else {
            false
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let light = Light::point_light(Point3::new(16.0, 10.0, 25.0), Vector3::new(1.0, 1.0, 1.0));
        let m = Material {
            color: Vector3::new(0.1, 1.0, 0.1),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        };

        let mut s1 = Sphere::new();

        s1.material = m;

        s1.transform = Matrix4::new_nonuniform_scaling(&Vector3::new(1.5, 1.5, 1.5))
            * Matrix4::new_translation(&Vector3::new(2.0, 0.8, -2.0));

        let m2 = Material {
            color: Vector3::new(0.2, 0.1, 0.8),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        };

        let mut s2 = Sphere::new();

        s2.material = m2;

        s2.transform = Matrix4::new_nonuniform_scaling(&Vector3::new(2.0, 2.0, 2.0))
            * Matrix4::new_translation(&Vector3::new(-1.3, 1.0, 0.0));

        // let mut s3_material = Material::default();
        // s3_material.color = Vector3::new(0.9, 0.1, 0.8);
        // let mut s3 = Sphere::new();
        // s3.material = s3_material;

        // s3.transform = Matrix4::new_translation(&Vector3::new(3.5, 0.5, -0.5))
        //     * Matrix4::new_nonuniform_scaling(&Vector3::new(2.5, 2.5, 2.5));

        let floor = Box::new(Plane::new());

        World {
            // objects: vec![Arc::new(s1), Arc::new(s2), Arc::new(floor)],
            // objects: vec![Arc::new(floor)],
            objects: vec![floor],
            light,
        }
    }
}

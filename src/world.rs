use nalgebra::{Matrix4, Point3, Translation, Vector3};

use std::cmp::Ordering;

use crate::intersect::Intersect;
use crate::intersection::{Intersection, PreparedComputations};
use crate::light::Light;
use crate::material::Material;
use crate::normal::Normal;
use crate::plane::Plane;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::triangle::Triangle;

#[derive(Clone, Debug)]
pub struct World<T: Shape + Clone + Sync + Send> {
    pub objects: Vec<T>,
    pub light: Light,
}

impl<T: Intersect<T> + Shape + Clone + Normal + Sync + Send> World<T> {
    pub fn new() -> Self {
        World {
            objects: vec![],
            light: Light::default(),
        }
    }

    // pub fn contains<T: Shape + PartialEq>(&self, object: Box<Shape>) -> bool {
    //     let xs = self.objects.iter().map(|x| *x.clone()).collect::<Vec<Box<Shape>>>();
    //     xs.contains(&object)
    // }

    fn intersect(&self, ray: Ray) -> Vec<Intersection<T>> {
        let mut intersections: Vec<Intersection<T>> = self
            .objects
            .iter()
            .flat_map(|object| (*object).intersect(&ray))
            .collect();

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal));

        intersections
    }

    fn shade_hit(&self, comps: PreparedComputations<T>) -> Vector3<f32> {
        Light::lighting(
            (*comps.1).material(),
            self.light,
            comps.6,
            comps.3,
            comps.4,
            self.is_shadowed(comps.6),
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

impl Default for World<Triangle> {
    fn default() -> Self {
        let light =
            Light::point_light(Point3::new(-10.0, -10.0, -5.0), Vector3::new(1.0, 1.0, 1.0));
        // let mut m = Material::default();
        // m.color = Vector3::new(0.5, 1.0, 0.1);
        // m.diffuse = 0.7;
        // m.specular = 0.2;

        // let mut s1 = Sphere::new();

        // s1.material = m;

        // let mut s2 = Sphere::new();

        // s2.transform = Matrix4::new_nonuniform_scaling(&Vector3::new(0.5, 0.5, 0.5))
        //     * Matrix4::new_translation(&Vector3::new(1.5, 0.5, -0.5));

        // let mut s3_material = Material::default();
        // s3_material.color = Vector3::new(0.9, 0.1, 0.8);
        // let mut s3 = Sphere::new();
        // s3.material = s3_material;

        // s3.transform = Matrix4::new_translation(&Vector3::new(3.5, 0.5, -0.5))
        //     * Matrix4::new_nonuniform_scaling(&Vector3::new(2.5, 2.5, 2.5));

        // let floor = Plane::new();

        World {
            objects: vec![],
            // objects: vec![s1, s2, s3],
            // objects: vec![floor],
            light,
        }
    }
}

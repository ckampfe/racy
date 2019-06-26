use nalgebra::{Matrix4, Point3, Vector3};

use std::cmp::Ordering;
use std::sync::Arc;

use crate::intersection::{Intersection, PreparedComputations};
use crate::light::Light;
use crate::material::Material;
use crate::plane::Plane;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::Sphere;

pub struct World {
    pub objects: Vec<Arc<dyn Shape + Send + Sync>>,
    pub light: Light,
}

impl World {
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

    fn intersect<T: Shape>(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = self
            .objects
            .iter()
            .flat_map(|object| (**object).intersect(&ray))
            .collect();

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal));

        // let boxed = intersections.iter().map(|i| Box::new(i)).collect();

        // boxed
        intersections
    }

    fn shade_hit<T: Shape>(&self, comps: PreparedComputations<&Box<dyn Shape>>) -> Vector3<f32> {
        Light::lighting(
            (*comps.1).material(),
            self.light,
            comps.6,
            comps.3,
            comps.4,
            self.is_shadowed::<T>(comps.6),
        )
    }

    pub fn color_at<T: Shape>(&self, ray: Ray) -> Vector3<f32> {
        let intersections = self.intersect::<T>(ray);
        let intersection = Intersection::hit(intersections);

        if let Some(i) = intersection.get(0) {
            let comps = i.prepare_computations(&ray);
            self.shade_hit::<T>(comps)
        } else {
            Vector3::new(0.0, 0.0, 0.0) // black
        }
    }

    fn is_shadowed<T: Shape>(&self, point: Point3<f32>) -> bool {
        let v = self.light.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let ray = Ray::new(point, direction);
        let intersections = self.intersect::<T>(ray);
        let hit = Intersection::hit(intersections);

        if let Some(h) = hit.get(0) {
            h.t < distance
        } else {
            false
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let light = Light::point_light(Point3::new(16.0, 10.0, 25.0), Vector3::new(1.0, 1.0, 1.0));
        let mut m = Material::default();
        m.color = Vector3::new(0.1, 1.0, 0.1);
        m.diffuse = 0.7;
        m.specular = 0.2;

        let mut s1 = Sphere::new();

        s1.material = m;

        s1.transform = Matrix4::new_nonuniform_scaling(&Vector3::new(1.5, 1.5, 1.5))
            * Matrix4::new_translation(&Vector3::new(2.0, 0.8, -2.0));

        let mut m2 = Material::default();

        m2.color = Vector3::new(0.2, 0.1, 0.8);
        m2.diffuse = 0.7;
        m2.specular = 0.2;

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

        let floor = Plane::new();

        World {
            objects: vec![Arc::new(s1), Arc::new(s2), Arc::new(floor)],
            light,
        }
    }
}

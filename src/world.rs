use nalgebra::{Matrix4, Point3, Vector3};

use std::cmp::Ordering;
use std::sync::Arc;

use crate::group::Group;
use crate::intersection::{Intersection, PreparedComputations};
use crate::light::Light;
use crate::material::Material;
use crate::plane::Plane;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::Sphere;
use std::sync::RwLock;

pub struct World {
    pub objects: Vec<Arc<RwLock<dyn Shape + Send + Sync>>>,
    pub light: Light,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: vec![],
            light: Light::default(),
        }
    }

    pub fn add_object(&mut self, shape: Arc<RwLock<dyn Shape + Send + Sync>>) -> usize {
        let obj_index = self.objects.len();
        shape.write().unwrap().set_obj_index(obj_index);

        self.objects.push(shape);

        obj_index
    }

    pub fn add_group(&mut self, group: &mut Group) {
        let indexes = group
            .children_objs
            .iter()
            .map(|obj| self.add_object(obj.clone()))
            .collect::<Vec<usize>>();

        group.children = indexes
    }

    // pub fn contains<T: Shape + PartialEq>(&self, object: Box<Shape>) -> bool {
    //     let xs = self.objects.iter().map(|x| *x.clone()).collect::<Vec<Box<Shape>>>();
    //     xs.contains(&object)
    // }

    fn intersect<T: Shape>(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = self
            .objects
            .iter()
            .enumerate()
            .flat_map(|(index, object)| object.read().unwrap().intersect(&ray, index))
            .collect();

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal));

        intersections
    }

    fn shade_hit<T: Shape>(&self, comps: PreparedComputations) -> Vector3<f32> {
        let obj = self.objects[comps.object_index].read().unwrap();
        Light::lighting(
            obj.material(),
            self.light,
            comps.point,
            comps.eyev,
            comps.normalv,
            self.is_shadowed::<T>(comps.over_point),
        )
    }

    pub fn color_at<T: Shape>(&self, ray: Ray) -> Vector3<f32> {
        let intersections = self.intersect::<T>(ray);
        let intersection = Intersection::hit(intersections);

        if let Some(i) = intersection {
            let comps = i.prepare_computations(&self, &ray);
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
            objects: vec![
                Arc::new(RwLock::new(s1)),
                Arc::new(RwLock::new(s2)),
                Arc::new(RwLock::new(floor)),
            ],
            light,
        }
    }
}

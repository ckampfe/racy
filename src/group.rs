// use crate::example_shape::ExampleShape;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Shape;
use nalgebra::{Matrix4, Point3, Vector3};
use std::sync::{Arc, RwLock};
use crate::world::World;
use std::cmp::Ordering;

pub struct Group<'a> {
    material: Material,
    transform: Matrix4<f32>,
    pub children_objs: Vec<Arc<RwLock<dyn Shape + Send + Sync>>>,
    pub children: Vec<usize>,
    pub parent: Option<&'a mut Group<'a>>,
    obj_index: usize,
}

impl<'a> Group<'a> {
    pub fn new() -> Self {
        Group {
            material: Material::default(),
            transform: Matrix4::identity(),
            children_objs: vec![],
            children: vec![],
            parent: None,
            obj_index: Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn add_children(&'a mut self, children: &'a [Arc<RwLock<Shape + Send + Sync>>]) {
        self.children_objs = children.to_vec();
    }

}

impl Shape for Group<'_> {
    fn get_obj_index(&self) -> usize {
        self.obj_index
    }

    fn set_obj_index(&mut self, obj_index: usize) {
        self.obj_index = obj_index
    }

    fn material(&self) -> Material {
        self.material
    }
    fn transform(&self) -> Matrix4<f32> {
        self.transform
    }
    fn normal_at(&self, point: Point3<f32>) -> Vector3<f32> {
        Vector3::zeros()
    }

    fn local_intersect(&'_ self, ray: Ray, object_index: usize) -> Vec<Intersection> {
        let mut intersections = self
            .children_objs
            .iter()
            .flat_map(|child_shape| {
                let shape = child_shape.read().unwrap();
                shape.intersect(&ray, object_index)
            })
            .collect::<Vec<Intersection>>();

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal));

        intersections
    }
    fn local_normal_at(&self, point: Point3<f32>) -> Vector3<f32> {
        Vector3::zeros()
    }
}

impl<'a> PartialEq<Group<'a>> for Group<'a> {
    fn eq(&self, other: &Group<'a>) -> bool {
            std::ptr::eq(
                self, other
                    )
                // self as &(dyn Shape + Send + Sync) as *const (dyn Shape + Send + Sync) as *const u8,
                // *other as *const (dyn Shape + Send + Sync) as *const u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::example_shape::ExampleShape;
    use crate::ray::Ray;
    use crate::sphere::Sphere;

    #[test]
    fn creating_a_new_group() {
        let g = Group::new();
        assert_eq!(g.transform, Matrix4::identity());
        assert_eq!(g.is_empty(), true)
    }

    #[test]
    fn a_shape_has_parent_attribute() {
        let s = ExampleShape::new();
        assert_eq!(s.parent.is_none(), true);
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let mut g = Group::new();
        let mut s = ExampleShape::new();
        g.add_child(&mut s);
        assert_eq!(g.is_empty(), false);
        assert!(s.parent().unwrap() == &g);
    }

    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let g = Group::new();
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let xs = g.local_intersect(r);
        assert_eq!(xs.is_empty(), true);
    }

    #[test]
    fn intersecting_a_ray_with_a_nonempty_group() {
        let mut g = Group::new();
        let s1 = Sphere::new();
        let mut s2 = Sphere::new();
        s2.transform = Matrix4::new_translation(&Vector3::new(0.0, 0.0, -3.0));
        let mut s3 = Sphere::new();
        s3.transform = Matrix4::new_translation(&Vector3::new(5.0, 0.0, 0.0));
        g.add_child(&mut s1);
        g.add_child(&mut s2);
        g.add_child(&mut s3);
        let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let xs = g.local_intersect(r);
        assert_eq!(xs.len(), 4);
        assert!(s2 == xs[0].object);
        assert!(s2 == xs[1].object);
        assert!(s1 == xs[2].object);
        assert!(s1 == xs[3].object);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let mut g = Group::new();
        g.transform = Matrix4::new_nonuniform_scaling(&Vector3::new(2.0, 2.0, 2.0));
        let mut s = Sphere::new();
        s.transform = Matrix4::new_translation(&Vector3::new(5.0, 0.0, 0.0));
        g.add_child(&mut s);
        let r = Ray::new(Point3::new(10.0, 0.0, -10.0), Vector3::new(0.0, 0.0, 1.0));
        let xs = g.intersect(&r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn converting_a_point_from_world_to_object_space() {
        let mut g1 = Group::new();
        g1.transform = Matrix4::<f32>::from_scaled_axis(
            Vector3::<f32>::y() * std::f64::consts::FRAC_2_PI as f32,
        );

        let mut g2 = Group::new();
        g2.transform = Matrix4::new_nonuniform_scaling(&Vector3::new(2.0, 2.0, 2.0));

        g1.add_child(&mut g2);

        let mut s = Sphere::new();
        s.transform = Matrix4::new_translation(&Vector3::new(5.0, 0.0, 0.0));

        g2.add_child(&mut s);

        let p = s.world_to_object(Point3::new(-2.0, 0.0, -10.0));

        assert_eq!(p, Point3::new(0.0, 0.0, -1.0))
    }
}

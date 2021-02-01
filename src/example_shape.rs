use crate::group::Group;
use crate::intersection::Intersection;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Shape;
use nalgebra::{Matrix4, Point3, Vector3};

pub struct ExampleShape<'a> {
    pub material: Material,
    pub transform: Matrix4<f32>,
    pub parent: Option<&'a mut Group<'a>>,
}

impl<'a> ExampleShape<'a> {
    pub fn new() -> Self {
        ExampleShape {
            material: Material::default(),
            transform: Matrix4::identity(),
            parent: None,
        }
    }
}

impl<'a> Shape<'a> for ExampleShape<'a> {
    fn kind(&self) -> &'static str {
        "example_shape"
    }

    fn parent(&self) -> Option<&'a mut Group<'a>> {
        self.parent
    }

    fn set_parent(&mut self, group: &'a mut Group<'a>) {
        self.parent = Some(group);
    }

    fn material(&self) -> Material {
        self.material
    }

    fn transform(&self) -> Matrix4<f32> {
        self.transform
    }

    fn normal_at(&self, point: Point3<f32>, intersection: &Intersection) -> Vector3<f32> {
        Vector3::zeros()
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection<'a>> {
        vec![]
    }

    fn local_normal_at(&self, point: Point3<f32>, _intersection: &Intersection) -> Vector3<f32> {
        Vector3::zeros()
    }
}

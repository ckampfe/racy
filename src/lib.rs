mod bounding_box;
mod camera;
mod canvas;
mod cube;
mod dyn_group;
mod group;
mod intersection;
mod light;
mod material;
mod normal;
mod plane;
mod ray;
mod shape;
mod sphere;
mod triangle;
mod world;

use camera::Camera;
use group::Group;
use material::Material;
use nalgebra::{Point3, Vector3};
use shape::Shape;
use triangle::Triangle;
use world::World;

pub struct Options {
    pub width_pixels: usize,
    pub height_pixels: usize,
    pub from: Point3<f32>,
    pub to: Point3<f32>,
    pub up: Vector3<f32>,
    pub fov_radians: f32,
    pub material_color: Vector3<f32>,
    pub image_format: image::ImageFormat,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            width_pixels: 400,
            height_pixels: 400,
            from: Point3::new(0.0, -2.5, -10.0),
            to: Point3::new(0.0, -5.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            fov_radians: std::f32::consts::FRAC_PI_2,
            material_color: Vector3::new(0.0196, 0.65, 0.874),
            image_format: image::ImageFormat::Png,
        }
    }
}

pub struct Scene {
    pub shapes: Vec<Box<dyn Shape>>,
}

pub fn render(mesh: &nom_stl::Mesh, options: &Options) -> Result<Vec<u8>, String> {
    let mut material = Material::new();

    material.color = options.material_color;

    let triangles = mesh
        .triangles()
        .iter()
        .map(|triangle| {
            let [v1i, v2i, v3i] = triangle.vertices();

            let mut triangle =
                Triangle::new(Point3::from(v1i), Point3::from(v2i), Point3::from(v3i));

            triangle.material = material;

            triangle
        })
        .collect::<Vec<_>>();

    let mut world = World::default();

    let mut camera = Camera::new(
        options.width_pixels,
        options.height_pixels,
        options.fov_radians,
    );

    let view_transforms = Camera::view_transforms(options.from, options.to, options.up);

    camera.transform = view_transforms;

    let group = Group::new(triangles);

    world.objects.push(Box::new(group));

    let canvas = camera.render(world);

    canvas.to_image(options.image_format)
}

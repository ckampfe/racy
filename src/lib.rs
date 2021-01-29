mod aabb;
mod camera;
mod canvas;
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

use std::sync::Arc;

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
        }
    }
}

pub fn render(file: &[u8], options: &Options) -> Result<Vec<u8>, String> {
    let mut bytes = std::io::Cursor::new(file);
    let mesh = nom_stl::parse_stl(&mut bytes).unwrap();

    let mut material = Material::new();

    material.color = options.material_color;

    let triangles: Vec<Arc<dyn Shape + Send + Sync>> = mesh
        .triangles()
        .iter()
        .map(|triangle| {
            let [v1i, v2i, v3i] = triangle.vertices();

            let mut triangle =
                Triangle::new(Point3::from(v1i), Point3::from(v2i), Point3::from(v3i));

            triangle.material = material;

            let shape: Arc<dyn Shape + Send + Sync> = Arc::new(triangle);

            shape
        })
        .collect::<Vec<Arc<dyn Shape + Send + Sync>>>();

    let mut world = World::default();

    let mut camera = Camera::new(
        options.width_pixels,
        options.height_pixels,
        options.fov_radians,
    );

    // let view_transforms = Camera::view_transforms(
    //     // Point3::new(0.0, -2.5, -5.0),
    //     Point3::new(-10.0, -20.5, -20.0),
    //     Point3::new(3.0, -50.0, -0.8),
    //     Vector3::new(0.0, 1.0, 0.0),
    // );

    let view_transforms = Camera::view_transforms(options.from, options.to, options.up);

    camera.transform = view_transforms;

    let group = Group::new(triangles);

    world.objects.push(Arc::new(group));

    let canvas = camera.render(world);

    // let ppm = canvas.to_ppm();
    canvas.to_image(image::ImageFormat::Jpeg)

    // let mut f = File::create("case.ppm")?;
    // let mut f = File::create("root_vase_floor.ppm")?;
    // let mut f = File::create("2021moon2.ppm")?;
    // let mut f = File::create("2021moon3.ppm")?;
    // let mut f = File::create("2021pen.ppm")?;
    // f.write_all(ppm.as_bytes())
}

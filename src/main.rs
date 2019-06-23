mod camera;
mod canvas;
mod intersect;
mod intersection;
mod light;
mod local_intersect;
mod local_normal;
mod material;
mod normal;
mod plane;
mod ray;
mod shape;
mod sphere;
mod triangle;
mod world;

use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::plane::Plane;
use crate::world::World;

use nalgebra::{Matrix4, Point3, Vector3, Vector4};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn clock() -> std::io::Result<()> {
    let white = Vector3::new(255.0, 255.0, 255.0);

    let mut canvas = Canvas::new(200, 200);

    let middle = Matrix4::<f32>::new_translation(&Vector3::new(100.0, 100.0, 0.0));

    let twelve = Matrix4::<f32>::new_translation(&Vector3::new(0.0, -75.0, 0.0))
        * Vector4::new(0.0, -1.0, 0.0, 1.0);

    let pi = std::f64::consts::PI;

    let mut numbers: Vec<f32> = vec![0.0];

    for p in 0..11 {
        numbers.push(numbers[p] + pi as f32 / 6.0);
    }

    let ns = numbers.iter().map(|radians| {
        middle * Matrix4::<f32>::from_scaled_axis(Vector3::<f32>::z() * *radians) * twelve
    });

    for n in ns {
        canvas.write_pixel(n.x.trunc() as usize, n.y.trunc() as usize, white);
    }

    let out = canvas.to_ppm();
    let mut f = File::create("clock.ppm")?;
    f.write_all(out.as_bytes())
}

fn stl() -> std::io::Result<()> {
    let floor = Plane::new();
    ///////////////////
    let mut world = World::default();

    // world.objects.push(floor);

    let mut camera = Camera::new(400, 400, std::f32::consts::PI / 2.0);

    let view_transforms = Camera::view_transforms(
        Point3::new(0.0, 1.5, -5.0),
        Point3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    camera.transform = view_transforms;

    let canvas = camera.render(world);

    let ppm = canvas.to_ppm();

    let mut f = File::create("x.ppm")?;
    f.write_all(ppm.as_bytes())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    // Ok(clock()?)
    Ok(stl()?)
}

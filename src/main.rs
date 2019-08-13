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

use crate::camera::Camera;
use crate::group::Group;
use crate::material::Material;
use crate::shape::Shape;
use crate::triangle::Triangle;
use crate::world::World;

use memmap::MmapOptions;
use nom_stl;
use rayon::prelude::*;

use nalgebra::{Point3, Vector3};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, RwLock};

/*
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
*/

/*
fn stl() -> std::io::Result<()> {
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
*/

fn stl2<T: Shape>() -> std::io::Result<()> {
    // options.stl_path
    let file = std::fs::File::open("/Users/clark/code/Moon.stl").unwrap();
    // let file = std::fs::File::open("/Users/clark/Downloads/rpi3-top_rev03.stl").unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    let (_, mesh) = nom_stl::parse_stl(&mmap).unwrap();

    let vertices = mesh.vertices;

    let mut material = Material::new();

    material.color = Vector3::new(0.0196, 0.65, 0.874);

    let mut triangles: Vec<Arc<RwLock<dyn Shape + Send + Sync>>> = mesh
        .triangles
        .par_iter()
        .map(|triangle| {
            let [v1i, v2i, v3i] = triangle.vertices;

            let mut triangle = Triangle::new(
                Point3::new(vertices[v1i][0], vertices[v1i][1], vertices[v1i][2]),
                Point3::new(vertices[v2i][0], vertices[v2i][1], vertices[v2i][2]),
                Point3::new(vertices[v3i][0], vertices[v3i][1], vertices[v3i][2]),
            );

            triangle.material = material;

            let shape: Arc<RwLock<dyn Shape + Send + Sync>> = Arc::new(RwLock::new(triangle));

            shape
        })
        .collect::<Vec<Arc<RwLock<dyn Shape + Send + Sync>>>>();

    let mut world = World::default();

    // let light = Light::point_light(Point3::new(70.0, 60.0, -5.0), Vector3::new(1.0, 1.0, 1.0));

    // world.light = light;

    let mut camera = Camera::new(800, 800, std::f32::consts::PI / 2.0);

    // let view_transforms = Camera::view_transforms(
    //     // Point3::new(0.0, -2.5, -5.0),
    //     Point3::new(-10.0, -20.5, -20.0),
    //     Point3::new(3.0, -50.0, -0.8),
    //     Vector3::new(0.0, 1.0, 0.0),
    // );
    let view_transforms = Camera::view_transforms(
        Point3::new(0.0, -2.5, -10.0),
        // Point3::new(-10.0, -20.5, -20.0),
        Point3::new(0.0, -5.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    camera.transform = view_transforms;

    // world.objects.append(&mut triangles);

    // for triangle in triangles {
    //     world.add_object(triangle);
    // }

    let s = sphere::Sphere::new();

    let g = Arc::new(RwLock::new(Group::new()));

    g.write().unwrap().children_objs = triangles;

    world.add_object(g);

    // add sphere
    world.add_object(Arc::new(RwLock::new(s)));

    /*
    1. Add interface to world objects (ie, "#add_child()" or similar)
    2. Have that interface stamp the object with its index in the object list
    3. A group is a list of indexes into this object list, rather than objects themselves
    4. All group work happens against this list, like the bounding box tests, etc.
    */

    /*
    1. create various shapes.
    2. add shapes to a group(s)
    at this point, the group owns the shapes
    3. add group to world.
    at this point, group transfers ownership of shapes to world
    and receives back indexes to those shapes so it can check them
    */

    let canvas = camera.render::<T>(world);

    let ppm = canvas.to_ppm();

    // let mut f = File::create("case.ppm")?;
    let mut f = File::create("moon_floor.ppm")?;
    f.write_all(ppm.as_bytes())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    // Ok(clock()?)
    let x = stl2::<sphere::Sphere>()?;
    Ok(x)
    // Ok(stl2()?)
}

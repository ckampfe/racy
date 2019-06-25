use crate::canvas::Canvas;
// use crate::intersect::Intersect;
use crate::normal::Normal;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::world::World;
use nalgebra::{Matrix4, Point3, Projective3, Vector3};
use rayon::prelude::*;

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f32,
    pub transform: Matrix4<f32>,
    pub half_width: f32,
    pub half_height: f32,
    pub pixel_size: f32,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f32 / vsize as f32;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = half_width * 2.0 / hsize as f32;

        let transform = Matrix4::identity();

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform,
            half_width,
            half_height,
            pixel_size,
        }
    }

    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let x_offset = (px as f32 + 0.5) * self.pixel_size;
        let y_offset = (py as f32 + 0.5) * self.pixel_size;
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let projective_inverse: Projective3<f32> =
            Projective3::from_matrix_unchecked(self.transform).inverse();

        let pixel = projective_inverse * Point3::new(world_x, world_y, -1.0);
        let origin = projective_inverse * Point3::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render<T: Shape>(&self, world: World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        // for y in 0..self.vsize {
        //     for x in 0..self.hsize {
        //         let ray = self.ray_for_pixel(x, y);
        //         let color = world.color_at(ray);
        //         image.write_pixel(x, y, color);
        //     }
        // }

        let xycs: Vec<(usize, usize, Vector3<f32>)> = (0..self.vsize)
            .into_par_iter()
            .flat_map(|y: usize| {
                (0..self.hsize)
                    .into_par_iter()
                    .map(|x| {
                        let ray = self.ray_for_pixel(x, y);
                        let color = world.color_at::<T>(ray);
                        (x, y, color)
                    })
                    .collect::<Vec<(usize, usize, Vector3<f32>)>>()
            })
            .collect();

        for (x, y, color) in xycs {
            image.write_pixel(x, y, color)
        }

        image
    }

    pub fn view_transforms(from: Point3<f32>, to: Point3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
        Matrix4::face_towards(&from, &to, &up)
    }
}

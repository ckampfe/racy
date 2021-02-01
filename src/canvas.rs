use std::convert::TryInto;

use image::{jpeg::JpegEncoder, png::PngEncoder, pnm::PnmEncoder};
use nalgebra::Vector3;

type Pixel = Vector3<f32>;

#[derive(Clone, Debug)]
pub struct Canvas {
    grid: Vec<Vec<Pixel>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut row = Vec::with_capacity(width);

        let black = Vector3::new(0.0, 0.0, 0.0);

        for _ in 0..width {
            row.push(black);
        }

        let mut rows = Vec::with_capacity(height);

        for _ in 0..height {
            rows.push(row.clone());
        }

        Canvas { grid: rows }
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    pub(crate) fn write_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.grid[y][x] = pixel;
    }

    fn pixel_at(&self, x: usize, y: usize) -> Pixel {
        self.grid[y][x]
    }

    pub(crate) fn to_image(&self, format: image::ImageFormat) -> Result<Vec<u8>, String> {
        let mut buf = Vec::new();
        let mut img = image::RgbImage::new(
            self.width().try_into().unwrap(),
            self.height().try_into().unwrap(),
        );
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pixel = self.pixel_at(x, y);
                let color = image::Rgb([
                    scale(clamp(pixel.x)).try_into().unwrap(),
                    scale(clamp(pixel.y)).try_into().unwrap(),
                    scale(clamp(pixel.z)).try_into().unwrap(),
                ]);

                img.put_pixel(x.try_into().unwrap(), y.try_into().unwrap(), color);
            }
        }

        match format {
            image::ImageFormat::Png => {
                let (x, y) = img.dimensions();

                let as_png = PngEncoder::new(&mut buf);

                let page_as_bytes = img.into_raw();

                as_png
                    .encode(&page_as_bytes, x, y, image::ColorType::Rgb8)
                    .map_err(|e| format!("{}", e))?;

                Ok(buf)
            }
            image::ImageFormat::Jpeg => {
                let mut encoder = JpegEncoder::new(&mut buf);
                encoder.encode_image(&img).map_err(|e| format!("{}", e))?;
                Ok(buf)
            }
            image::ImageFormat::Pnm => {
                let mut encoder = PnmEncoder::new(&mut buf);
                let (x, y) = img.dimensions();
                let as_bytes = img.into_raw();
                encoder
                    .encode(as_bytes.as_slice(), x, y, image::ColorType::Rgb8)
                    .map_err(|e| format!("{}", e))?;

                Ok(buf)
            }
            _ => Err(format!("{:?} not supported", format)), // image::ImageFormat::WebP => {}
                                                             // image::ImageFormat::Pnm => {}
                                                             // image::ImageFormat::Tiff => {}
                                                             // image::ImageFormat::Tga => {}
                                                             // image::ImageFormat::Dds => {}
                                                             // image::ImageFormat::Bmp => {}
                                                             // image::ImageFormat::Ico => {}
                                                             // image::ImageFormat::Hdr => {}
                                                             // image::ImageFormat::Farbfeld => {}
                                                             // image::ImageFormat::Avif => {}
                                                             // image::ImageFormat::__NonExhaustive(_) => {}
        }
    }
}

fn clamp(color_channel: f32) -> f32 {
    match color_channel {
        c if c > 1.0 => 1.0,
        c if c < 0.0 => 0.0,
        c => c,
    }
}

fn scale(color_channel: f32) -> usize {
    (color_channel * 255.0).ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width(), 10);
        assert_eq!(c.height(), 20);
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Vector3::<f32>::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }
}

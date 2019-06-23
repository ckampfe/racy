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

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.grid[y][x] = pixel;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Pixel {
        self.grid[y][x]
    }

    pub fn to_ppm(&self) -> String {
        let width = self.width();
        let height = self.height();

        let mut pixel_rows: Vec<Vec<String>> = Vec::with_capacity(height);

        for y in 0..height {
            let mut row: Vec<String> = Vec::with_capacity(width);

            for x in 0..width {
                let pixel = self.pixel_at(x, y);
                let r = format!("{}", scale(clamp(pixel.x)));
                let g = format!("{}", scale(clamp(pixel.y)));
                let b = format!("{}", scale(clamp(pixel.z)));
                row.push(r);
                row.push(g);
                row.push(b);
            }

            pixel_rows.push(row);
        }

        let mut ppm_pixels = "".to_string();

        let mut line = "".to_string();

        for row in pixel_rows {
            for color_channel_value in row {
                if line.len() + color_channel_value.len() + 1 >= 70 {
                    line.push_str(&"\n");
                    ppm_pixels.push_str(&line);
                    line = "".to_string();
                    line.push_str(&color_channel_value)
                } else if line == "" {
                    line.push_str(&color_channel_value);
                } else {
                    line.push_str(&" ");
                    line.push_str(&color_channel_value);
                }
            }

            if line != "" {
                line.push_str(&"\n");
                ppm_pixels.push_str(&line);
                // reset line
                line = "".to_string();
            }
        }

        format!("P3\n{} {}\n255\n{}", width, height, ppm_pixels)
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

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let v: Vec<&str> = ppm.split('\n').take(3).collect();
        let mut joined = v.join("\n");
        joined.push_str("\n");
        let test = "P3\n5 3\n255\n";
        assert_eq!(joined, test);
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Vector3::<f32>::new(1.5, 0.0, 0.0);
        let c2 = Vector3::<f32>::new(0.0, 0.5, 0.0);
        let c3 = Vector3::<f32>::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm = c.to_ppm();
        println!("c {:?}", c);
        println!("ppm {:?}", ppm);

        let v: Vec<&str> = ppm.split('\n').skip(3).take(3).collect();
        println!("{:?}", v);
        let mut data_lines = v.join("\n");
        data_lines.push_str("\n");

        let example_lines = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";
        assert_eq!(data_lines, example_lines);
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        let pixel = Vector3::new(1.0, 0.8, 0.6);
        for y in 0..c.height() {
            for x in 0..c.width() {
                c.write_pixel(x, y, pixel);
            }
        }

        let ppm = c.to_ppm();
        let v: Vec<&str> = ppm.split('\n').skip(3).take(4).collect();
        let mut data_lines = v.join("\n");
        data_lines.push_str("\n");

        let example_lines = "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n";

        assert_eq!(data_lines, example_lines);
    }

    #[test]
    fn ppm_files_are_termianted_by_a_newline_character() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        assert!(ppm.ends_with("\n"));
    }
}

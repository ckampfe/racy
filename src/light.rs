use crate::material::Material;
use nalgebra::{Point3, Vector3};

pub struct Light {
    pub position: Point3<f32>,
    pub intensity: Vector3<f32>,
}

impl Light {
    pub fn point_light(position: Point3<f32>, intensity: Vector3<f32>) -> Self {
        Light {
            position,
            intensity,
        }
    }

    pub fn lighting(
        material: Material,
        light: Light,
        point: Point3<f32>,
        eyev: Vector3<f32>,
        normalv: Vector3<f32>,
        in_shadow: bool,
    ) -> Vector3<f32> {
        let effective_color = material.color.component_mul(&light.intensity);
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * material.ambient;
        let light_dot_normal = lightv.dot(&normalv);
        let black = Vector3::new(0.0, 0.0, 0.0);

        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (black, black)
        } else {
            let diffuse: Vector3<f32> = effective_color * material.diffuse * light_dot_normal;
            let reflectv = reflect(lightv * -1.0, normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);

            if reflect_dot_eye <= 0.0 {
                (diffuse, black)
            } else {
                let factor = reflect_dot_eye.powf(material.shininess);
                let specular = light.intensity * material.specular * factor;

                (diffuse, specular)
            }
        };

        if in_shadow {
            ambient
        } else {
            ambient + diffuse + specular
        }
    }
}

fn reflect(in_vec: Vector3<f32>, normal_vec: Vector3<f32>) -> Vector3<f32> {
    in_vec - (normal_vec * 2.0) * (in_vec.dot(&normal_vec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_light_has_a_position_an_intensity() {
        let intensity = Vector3::new(1.0, 1.0, 1.0);
        let position = Point3::new(0.0, 0.0, 0.0);
        let light = Light::point_light(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::new();
        let position = Point3::new(0.0, 0.0, 0.0);
        let eyev = Vector3::new(0.0, 0.0, -1.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = Light::point_light(Point3::new(0.0, 0.0, -10.0), Vector3::new(1.0, 1.0, 1.0));
        let result = Light::lighting(m, light, position, eyev, normalv, false);
        assert_eq!(result, Vector3::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45_degrees() {
        let m = Material::new();
        let position = Point3::new(0.0, 0.0, 0.0);
        let eyev = Vector3::new(0.0, 2.0_f32.sqrt() / 2.0, -1.0 * 2.0_f32.sqrt() / 2.0);
        let normalv = Vector3::new(0.0, 0.0, -1.0);
        let light = Light::point_light(Point3::new(0.0, 0.0, -10.0), Vector3::new(1.0, 1.0, 1.0));
        let result = Light::lighting(m, light, position, eyev, normalv, false);
        assert_eq!(result, Vector3::new(1.0, 1.0, 1.0))
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_45_degrees() {}

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {}

    #[test]
    fn lighting_with_the_light_behind_the_surface() {}

    #[test]
    fn lighting_with_the_surface_in_shadow() {}
}

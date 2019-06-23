use crate::material::Material;

pub trait Shape {
    fn material(&self) -> Material;
}

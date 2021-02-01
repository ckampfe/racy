use nalgebra::Point3;

pub trait BoundingBox {
    fn bounding_box(&self) -> AABB;
}

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

impl AABB {
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn merge(&self, other: AABB) -> AABB {
    //     let min = if other.min < self.min {
    //         other.min
    //     } else {
    //         self.min
    //     };

    //     let max = if other.max > self.max {
    //         other.max
    //     } else {
    //         self.max
    //     };

    //     AABB { min, max }
    // }

    pub fn merge_mut(&mut self, other: AABB) {
        if other.min < self.min {
            self.min = other.min;
        };

        if other.max > self.max {
            self.max = other.max;
        }
    }
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            min: Point3::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY),
            max: Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
        }
    }
}

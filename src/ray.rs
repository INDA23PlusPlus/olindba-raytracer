use crate::util::vector::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction: direction.normalized()
        }
    }
}

#[derive(Clone)]
pub struct Interval {
    pub min: f32,
    pub max: f32
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Interval {
        Interval {
            min,
            max
        }
    }

    pub fn standard() -> Interval {
        Interval {
            min: 0.001,
            max: 1e9
        }
    }

    pub fn max() -> Interval {
        Interval {
            min: -1e9,
            max:  1e9
        }
    }

    pub fn rev() -> Interval {
        Interval {
            min:  1e9,
            max: -1e9
        }
    }

    pub fn expand(&mut self, delta: f32) {
        self.min = self.min - delta / 2.0;
        self.max = self.max + delta / 2.0;
    }
}
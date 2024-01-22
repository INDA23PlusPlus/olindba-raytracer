#[derive(Copy, Clone)]
pub struct OldVec3 {
    elements: [f64; 3]
}

impl OldVec3 {
    pub fn new(elements: [f64; 3]) -> OldVec3 {
        OldVec3 { elements }
    }

    pub fn zero() -> OldVec3 {
        OldVec3 {  elements: [0.0; 3] }
    }

    pub fn set(&mut self, pos: usize, value: f64) {
        self.elements[pos] = value;
    }

    pub fn get(&self, pos: usize) -> f64 {
        self.elements[pos]
    }

    pub fn length(&self) -> f64 {
        self.elements.iter().map(|e| e * e).sum::<f64>().sqrt()
    }

    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    pub fn normalized(&self) -> OldVec3 {
        *self * (1.0 / self.length())
    }

    pub fn cross(&self, vec: OldVec3) -> OldVec3 {
        let mut out = OldVec3::zero();
        out.set(0, self.get(1) * vec.get(2) - self.get(2) * vec.get(1));
        out.set(1, self.get(2) * vec.get(0) - self.get(0) * vec.get(2));
        out.set(2, self.get(0) * vec.get(1) - self.get(1) * vec.get(0));
        out
    }

    pub fn project(&self, vec: OldVec3) -> OldVec3 {
        vec * (vec * *self * (1.0 / (vec * vec)))
    }

    pub fn reflect(&self, vec: OldVec3) -> OldVec3 {
        *self - vec * ((*self * vec) * 2.0)
    }
}

impl std::ops::Add<OldVec3> for OldVec3 {
    type Output = OldVec3;

    fn add(self, rhs: OldVec3) -> OldVec3 {
        let mut out = OldVec3::zero();
        for i in 0..3 {
            out.set(i, self.get(i) + rhs.get(i));
        }
        out
    }
}

impl std::ops::Sub<OldVec3> for OldVec3 {
    type Output = OldVec3;

    fn sub(self, rhs: OldVec3) -> OldVec3 {
        let mut out = OldVec3::zero();
        for i in 0..3 {
            out.set(i, self.get(i) - rhs.get(i));
        }
        out
    }
}

impl std::ops::Mul<OldVec3> for OldVec3 {
    type Output = f64;

    fn mul(self, rhs: OldVec3) -> f64 {
        self.elements.iter().zip(rhs.elements.iter()).map(|(e1, e2)| e1 * e2).sum()
    }
}

impl std::ops::Mul<f64> for OldVec3 {
    type Output = OldVec3;

    fn mul(self, rhs: f64) -> OldVec3 {
        let mut out = OldVec3::zero();
        for i in 0..3 {
            out.set(i, self.get(i) * rhs);
        }
        out
    }
}

impl std::fmt::Display for OldVec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}

pub fn lerp(vec1: OldVec3, vec2: OldVec3, step: f64) -> OldVec3 {
    vec1 + (vec2 - vec1) * step
}
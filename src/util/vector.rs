use std::arch::x86_64::*;

#[derive(Clone, Copy)]
pub union Vec3 {
    components: [f32; 4],
    xmm: __m128
}

impl Vec3 {
    pub fn new(components: [f32; 3]) -> Vec3 {
        Vec3 { components: [
            components[0], 
            components[1], 
            components[2], 
            0.0
        ] }
    }

    pub fn from(xmm: __m128) -> Vec3 {
        Vec3 { xmm }
    }

    pub fn zero() -> Vec3 {
        unsafe { Vec3 { xmm: _mm_setzero_ps() }}
    }

    pub fn set(&mut self, pos: usize, value: f32) {
        unsafe { self.components[pos] = value; }
    }

    pub fn get(&self, pos: usize) -> f32 {
        unsafe { self.components[pos] }
    }

    pub fn length(&self) -> f32 {
        unsafe { self.components.iter().map(|e| e * e).sum::<f32>().sqrt() }
    }

    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    pub fn normalized(&self) -> Vec3 {
        *self * (1.0 / self.length())
    }

    pub fn cross(&self, vec: Vec3) -> Vec3 {
        unsafe {
            let tmp0 = _mm_shuffle_ps(self.xmm ,self.xmm, shuffle_mask(3,0,2,1));
            let tmp1 = _mm_shuffle_ps(vec.xmm, vec.xmm, shuffle_mask(3,1,0,2));
            let tmp2 = _mm_mul_ps(tmp0, vec.xmm);
            let tmp3 = _mm_mul_ps(tmp0, tmp1);
            let tmp4 = _mm_shuffle_ps(tmp2, tmp2, shuffle_mask(3,0,2,1));
            Vec3::from(_mm_sub_ps(tmp3, tmp4))
        }
    }

    pub fn project(&self, vec: Vec3) -> Vec3 {
        vec * (vec * *self * (1.0 / (vec * vec)))
    }

    pub fn reflect(&self, vec: Vec3) -> Vec3 {
        *self - vec * ((*self * vec) * 2.0)
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        unsafe { Vec3::from(_mm_add_ps(self.xmm, rhs.xmm)) }
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        unsafe { Vec3::from(_mm_sub_ps(self.xmm, rhs.xmm)) }
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Vec3) -> Self::Output {
        unsafe { self.components.iter().zip(rhs.components.iter()).map(|(e1, e2)| e1 * e2).sum::<f32>() }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        unsafe {
            let scalar = _mm_set1_ps(rhs);
            Vec3::from(_mm_mul_ps(self.xmm, scalar))
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe { write!(f, "{:?}", self.components) }
    }
}

pub const fn shuffle_mask(z: u32, y: u32, x: u32, w: u32) -> i32 {
    ((z << 6) | (y << 4) | (x << 2) | w) as i32
}

pub fn lerp(vec1: Vec3, vec2: Vec3, step: f32) -> Vec3 {
    vec1 + (vec2 - vec1) * step
}
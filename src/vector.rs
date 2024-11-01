#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2
{
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec2
{
    pub fn new(x: f32, y: f32) -> Vec2
    {
        Vec2 { x, y }
    }

    #[inline(always)]
    pub fn add(&self, other: &Vec2) -> Vec2
    {
        Vec2
        {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    #[inline(always)]
    pub fn sub(&self, other: &Vec2) -> Vec2
    {
        Vec2
        {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    #[inline(always)]
    pub fn mul_scale(&self, scale: f32) -> Vec2
    {
        Vec2
        {
            x: self.x * scale,
            y: self.y * scale,
        }
    }

    #[inline(always)]
    pub fn dot(&self, other: &Vec2) -> f32
    {
        self.x * other.x + self.y * other.y
    }

    #[inline(always)]
    pub fn length(&self) -> f32
    {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[inline(always)]
    pub fn unit(&self) -> Vec2
    {
        let len = self.length();
        if len > 0.0
        {
            self.mul_scale(1.0 / len)
        }
        else
        {
            *self
        }
    }

    #[inline(always)]
    pub fn distance(&self, other: &Vec2) -> f32
    {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    #[inline(always)]
    pub fn angle(&self, other: &Vec2) -> f32
    {
        (self.dot(other) / (self.length() * other.length())).acos()
    }

    #[inline(always)]
    pub fn project(&self, other: &Vec2) -> Vec2
    {
        let scale = self.dot(other) / other.dot(other);
        other.mul_scale(scale)
    }

    #[inline(always)]
    pub fn reflect(&self, normal: &Vec2) -> Vec2
    {
        let dot = self.dot(normal);
        Vec2
        {
            x: self.x - 2.0 * dot * normal.x,
            y: self.y - 2.0 * dot * normal.y,
        }
    }
}

impl Vec3
{
    pub fn new(x: f32, y: f32, z: f32) -> Vec3
    {
        Vec3 { x, y, z }
    }

    #[inline(always)]
    pub fn add(&self, other: &Vec3) -> Vec3
    {
        Vec3
        {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    #[inline(always)]
    pub fn sub(&self, other: &Vec3) -> Vec3
    {
        Vec3
        {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    #[inline(always)]
    pub fn mul_scale(&self, scale: f32) -> Vec3
    {
        Vec3
        {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }

    #[inline(always)]
    pub fn dot(&self, other: &Vec3) -> f32
    {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline(always)]
    pub fn cross(&self, other: &Vec3) -> Vec3
    {
        Vec3
        {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[inline(always)]
    pub fn length(&self) -> f32
    {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline(always)]
    pub fn normalize(&self) -> Vec3
    {
        let len: f32 = self.length();
        if len > 0.0
        {
            self.mul_scale(1.0 / len)
        }
        else
        {
            *self
        }
    }

    #[inline(always)]
    pub fn distance(&self, other: &Vec3) -> f32
    {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }

    #[inline(always)]
    pub fn angle(&self, other: &Vec3) -> f32
    {
        (self.dot(other) / (self.length() * other.length())).acos()
    }

    #[inline(always)]
    pub fn project(&self, other: &Vec3) -> Vec3
    {
        let scale = self.dot(other) / other.dot(other);
        other.mul_scale(scale)
    }

    #[inline(always)]
    pub fn reflect(&self, normal: &Vec3) -> Vec3
    {
        let dot = self.dot(normal);
        Vec3 {
            x: self.x - 2.0 * dot * normal.x,
            y: self.y - 2.0 * dot * normal.y,
            z: self.z - 2.0 * dot * normal.z,
        }
    }
}

impl Vec4
{
    #[inline(always)]
    pub fn as_ptr(&self) -> *const f32
    {
        &self.x as *const f32
    }

    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4
    {
        Vec4 { x, y, z, w }
    }

    #[inline(always)]
    pub fn add(&self, other: &Vec4) -> Vec4
    {
        Vec4
        {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    #[inline(always)]
    pub fn sub(&self, other: &Vec4) -> Vec4
    {
        Vec4
        {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }

    #[inline(always)]
    pub fn mul_scale(&self, scale: f32) -> Vec4
    {
        Vec4
        {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
            w: self.w * scale,
        }
    }

    #[inline(always)]
    pub fn dot(&self, other: &Vec4) -> f32
    {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    pub fn dot_simd(&self, other: &Vec4) -> f32
    {
        unsafe {
            let a = _mm_loadu_ps(self.as_ptr());
            let b = _mm_loadu_ps(other.as_ptr());
            let product = _mm_mul_ps(a, b);
            let sum1 = _mm_hadd_ps(product, product);
            let sum2 = _mm_hadd_ps(sum1, sum1);
            _mm_cvtss_f32(sum2)
        }
    }

    #[cfg(target_arch = "aarch64")]
    #[inline(always)]
    pub fn dot_simd(&self, other: &Vec4) -> f32
    {
        unsafe {
            let a = vld1q_f32(self.as_ptr());
            let b = vld1q_f32(other.as_ptr());
            let product = vmulq_f32(a, b);
            let sum1 = vaddq_f32(product, vextq_f32::<2>(product, product));
            let sum2 = vaddq_f32(sum1, vextq_f32::<1>(sum1, sum1));
            vgetq_lane_f32::<0>(sum2)
        }
    }

    #[inline(always)]
    pub fn length(&self) -> f32
    {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    #[inline(always)]
    pub fn unit(&self) -> Vec4
    {
        let len = self.length();
        if len > 0.0
        {
            self.mul_scale(1.0 / len)
        }
        else
        {
            *self
        }
    }

    #[inline(always)]
    pub fn distance(&self, other: &Vec4) -> f32
    {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2) + (self.w - other.w).powi(2)).sqrt()
    }

    #[inline(always)]
    pub fn angle(&self, other: &Vec4) -> f32
    {
        (self.dot(other) / (self.length() * other.length())).acos()
    }

    pub fn project(&self, other: &Vec4) -> Vec4
    {
        let scale = self.dot(other) / other.dot(other);
        other.mul_scale(scale)
    }

    pub fn reflect(&self, normal: &Vec4) -> Vec4
    {
        let dot = self.dot(normal);
        Vec4
        {
            x: self.x - 2.0 * dot * normal.x,
            y: self.y - 2.0 * dot * normal.y,
            z: self.z - 2.0 * dot * normal.z,
            w: self.w - 2.0 * dot * normal.w,
        }
    }
}
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

use crate::math::EPSILON;
#[derive(Debug, Clone, PartialEq)]
pub struct Mat4
{

    pub data: [[f32; 4]; 4],
}

impl Mat4
{
    pub fn new_identity() -> Mat4
    {
        Mat4
        {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn new_zero() -> Mat4
    {
        Mat4
        {
            data: [[0.0; 4]; 4],
        }
    }

    #[inline(always)]
    pub fn add(&self, other: &Mat4) -> Mat4
    {
        let mut result = Mat4::new_zero();
        for i in 0..4 {
            for j in 0..4
            {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }

    #[inline(always)]
    pub fn mul(&self, other: &Mat4) -> Mat4
    {
        let mut result = Mat4::new_zero();
        for i in 0..4
        {
            for j in 0..4
            {
                result.data[i][j] = 0.0;
                for k in 0..4
                {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }

    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    pub fn mul_simd(&self, other: &Mat4) -> Mat4
    {
        let mut result = Mat4::new_zero();
        for i in 0..4
        {
            unsafe {
                for j in 0..4
                {
                    let mut dot_product = _mm_set1_ps(0.0);
                    for k in 0..4
                    {
                        let a_element = _mm_set1_ps(self.data[i][k]);
                        let b = _mm_loadu_ps(other.data[k].as_ptr());
                        let product = _mm_mul_ps(a_element, b);
                        dot_product = _mm_add_ps(dot_product, product);
                    }
                    let mut temp = [0.0; 4];
                    _mm_storeu_ps(temp.as_mut_ptr(), dot_product);
                    result.data[i][j] = temp[j];
                }
            }
        }
        result
    }

    #[cfg(target_arch = "aarch64")]
    #[inline(always)]
    pub fn mul_neon(&self, other: &Mat4) -> Mat4
    {
        let mut result = Mat4::new_zero();
        for i in 0..4
        {
            unsafe {
                for j in 0..4
                {
                    let mut dot_product = vdupq_n_f32(0.0);
                    for k in 0..4
                    {
                        let a_element = vdupq_n_f32(self.data[i][k]);
                        let b = vld1q_f32(other.data[k].as_ptr());
                        let product = vmulq_f32(a_element, b);
                        dot_product = vaddq_f32(dot_product, product);
                    }
                    let mut temp = [0.0; 4];
                    vst1q_f32(temp.as_mut_ptr(), dot_product);
                    result.data[i][j] = temp[j];
                }
            }
        }
        result
    }

    #[inline(always)]
    pub fn mul_auto(&self, other: &Mat4) -> Mat4
    {
        #[cfg(target_arch = "x86_64")]
        {
            self.mul_simd(other)
        }
        #[cfg(target_arch = "aarch64")]
        {
            self.mul_neon(other)
        }
    }

    #[inline(always)]
    pub fn invert(&self) -> Option<Mat4>
    {
        let det = self.determinant();
        if det.abs() < EPSILON
        {
            return None;
        }

        let mut inv = Mat4::new_zero();
        for i in 0..4
        {
            for j in 0..4
            {
                // Minor for element (i, j)
                let minor = self.get_minor(i, j);
                inv.data[j][i] = minor.determinant() / det;
            }
        }
        Some(inv)
    }

    #[inline(always)]
    pub fn determinant(&self) -> f32
    {
        self.data[0][0] * self.get_minor(0, 0).determinant() -
            self.data[0][1] * self.get_minor(0, 1).determinant() +
            self.data[0][2] * self.get_minor(0, 2).determinant() -
            self.data[0][3] * self.get_minor(0, 3).determinant()
    }

    #[inline(always)]
    fn get_minor(&self, row: usize, col: usize) -> Mat3
    {
        let mut minor = Mat3::new_zero();
        let mut minor_row = 0;
        for i in 0..4
        {
            if i != row
            {
                let mut minor_col = 0;
                for j in 0..4
                {
                    if j != col
                    {
                        minor.data[minor_row][minor_col] = self.data[i][j];
                        minor_col += 1;
                    }
                }
                minor_row += 1;
            }
        }
        minor
    }

    #[inline(always)]
    pub fn scale(sx: f32, sy: f32, sz: f32) -> Mat4
    {
        Mat4
        {
            data: [
                [sx, 0.0, 0.0, 0.0],
                [0.0, sy, 0.0, 0.0],
                [0.0, 0.0, sz, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline(always)]
    pub fn rotate_x(angle: f32) -> Mat4
    {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();
        Mat4
        {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cos_theta, -sin_theta, 0.0],
                [0.0, sin_theta, cos_theta, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline(always)]
    pub fn rotate_y(angle: f32) -> Mat4
    {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();
        Mat4
        {
            data: [
                [cos_theta, 0.0, sin_theta, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-sin_theta, 0.0, cos_theta, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline(always)]
    pub fn rotate_z(angle: f32) -> Mat4
    {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();
        Mat4
        {
            data: [
                [cos_theta, -sin_theta, 0.0, 0.0],
                [sin_theta, cos_theta, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline(always)]
    pub fn translate(tx: f32, ty: f32, tz: f32) -> Mat4
    {
        Mat4
        {
            data: [
                [1.0, 0.0, 0.0, tx],
                [0.0, 1.0, 0.0, ty],
                [0.0, 0.0, 1.0, tz],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mat3
{
    pub data: [[f32; 3]; 3],
}

impl Mat3
{
    pub fn new_identity() -> Mat3
    {
        Mat3
        {
            data: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline(always)]
    pub fn new_zero() -> Mat3
    {
        Mat3
        {
            data: [[0.0; 3]; 3],
        }
    }

    #[inline(always)]
    pub fn add(&self, other: &Mat3) -> Mat3
    {
        let mut result = Mat3::new_zero();
        for i in 0..3
        {
            for j in 0..3
            {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }

    #[inline(always)]
    pub fn mul(&self, other: &Mat3) -> Mat3
    {
        let mut result = Mat3::new_zero();
        for i in 0..3
        {
            for j in 0..3
            {
                result.data[i][j] = 0.0;
                for k in 0..3
                {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }

    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    pub fn mul_simd(&self, other: &Mat3) -> Mat3
    {
        let mut result = Mat3::new_zero();
        for i in 0..3
        {
            unsafe {
                for j in 0..3
                {
                    let mut dot_product = _mm_set1_ps(0.0);
                    for k in 0..3
                    {
                        let a_element = _mm_set1_ps(self.data[i][k]);
                        let b = _mm_loadu_ps(other.data[k].as_ptr());
                        let product = _mm_mul_ps(a_element, b);
                        dot_product = _mm_add_ps(dot_product, product);
                    }
                    let mut temp = [0.0; 4];
                    _mm_storeu_ps(temp.as_mut_ptr(), dot_product);
                    result.data[i][j] = temp[j];
                }
            }
        }
        result
    }

    #[cfg(target_arch = "aarch64")]
    #[inline(always)]
    pub fn mul_neon(&self, other: &Mat3) -> Mat3
    {
        let mut result = Mat3::new_zero();
        for i in 0..3 {
            unsafe {
                for j in 0..3
                {
                    let mut dot_product = vdupq_n_f32(0.0);
                    for k in 0..3
                    {
                        let a_element = vdupq_n_f32(self.data[i][k]);
                        let b = vld1q_f32(other.data[k].as_ptr());
                        let product = vmulq_f32(a_element, b);
                        dot_product = vaddq_f32(dot_product, product);
                    }
                    let mut temp = [0.0; 4];
                    vst1q_f32(temp.as_mut_ptr(), dot_product);
                    result.data[i][j] = temp[j];
                }
            }
        }
        result
    }

    #[inline(always)]
    pub fn mul_auto(&self, other: &Mat3) -> Mat3
    {
        #[cfg(target_arch = "x86_64")]
        {
            self.mul_simd(other)
        }
        #[cfg(target_arch = "aarch64")]
        {
            self.mul_neon(other)
        }
    }

    #[inline(always)]
    pub fn invert(&self) -> Option<Mat3>
    {
        let det = self.determinant();
        if det.abs() < EPSILON
        {
            return None;
        }

        let mut inv = Mat3::new_zero();
        inv.data[0][0] = self.data[1][1] * self.data[2][2] - self.data[1][2] * self.data[2][1];
        inv.data[0][1] = self.data[0][2] * self.data[2][1] - self.data[0][1] * self.data[2][2];
        inv.data[0][2] = self.data[0][1] * self.data[1][2] - self.data[0][2] * self.data[1][1];

        inv.data[1][0] = self.data[1][2] * self.data[2][0] - self.data[1][0] * self.data[2][2];
        inv.data[1][1] = self.data[0][0] * self.data[2][2] - self.data[0][2] * self.data[2][0];
        inv.data[1][2] = self.data[0][2] * self.data[1][0] - self.data[0][0] * self.data[1][2];

        inv.data[2][0] = self.data[1][0] * self.data[2][1] - self.data[1][1] * self.data[2][0];
        inv.data[2][1] = self.data[0][1] * self.data[2][0] - self.data[0][0] * self.data[2][1];
        inv.data[2][2] = self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0];

        for i in 0..3
        {
            for j in 0..3
            {
                inv.data[i][j] /= det;
            }
        }

        Some(inv)
    }

    #[inline(always)]
    pub fn determinant(&self) -> f32
    {
        self.data[0][0] * (self.data[1][1] * self.data[2][2] - self.data[1][2] * self.data[2][1]) -
            self.data[0][1] * (self.data[1][0] * self.data[2][2] - self.data[1][2] * self.data[2][0]) +
            self.data[0][2] * (self.data[1][0] * self.data[2][1] - self.data[1][1] * self.data[2][0])
    }
}
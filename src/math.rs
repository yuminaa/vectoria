#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate libm;

pub const PI: f32 = std::f32::consts::PI;
pub const TAU: f32 = 2.0 * PI;
pub const E: f32 = std::f32::consts::E;
pub const SQRT_2: f32 = std::f32::consts::SQRT_2;
pub const LN_2: f32 = std::f32::consts::LN_2;
pub const DEG_TO_RAD: f32 = PI / 180.0;
pub const RAD_TO_DEG: f32 = 180.0 / PI;
pub const EPSILON: f32 = 1e-6;

#[inline(always)]
pub fn min(a: f32, b: f32) -> f32
{
    a.min(b)
}

#[inline(always)]
pub fn max(a: f32, b: f32) -> f32
{
    a.max(b)
}

#[inline(always)]
pub fn clamp(value: f32, min: f32, max: f32) -> f32
{
    value.max(min).min(max)
}

#[inline(always)]
pub fn lerp(start: f32, end: f32, t: f32) -> f32
{
    start + t * (end - start)
}

#[inline(always)]
pub fn approx_equal(a: f32, b: f32, epsilon: f32) -> bool
{
    (a - b).abs() < epsilon
}

#[inline(always)]
pub fn approx_zero(a: f32, epsilon: f32) -> bool
{
    a.abs() < epsilon
}

#[inline(always)]
pub fn degrees_to_radians(degrees: f32) -> f32
{
    degrees * DEG_TO_RAD
}

#[inline(always)]
pub fn radians_to_degrees(radians: f32) -> f32
{
    radians * RAD_TO_DEG
}

#[cfg(feature = "std")]
#[inline(always)]
pub fn sin(x: f32) -> f32
{
    x.sin()
}

#[cfg(not(feature = "std"))]
#[inline(always)]
pub fn sin(x: f32) -> f32
{
    libm::sinf(x)
}

#[cfg(feature = "std")]
#[inline(always)]
pub fn cos(x: f32) -> f32
{
    x.cos()
}

#[cfg(not(feature = "std"))]
#[inline(always)]
pub fn cos(x: f32) -> f32
{
    libm::cosf(x)
}

#[cfg(feature = "std")]
#[inline(always)]
pub fn tan(x: f32) -> f32
{
    x.tan()
}

#[cfg(not(feature = "std"))]
#[inline(always)]
pub fn tan(x: f32) -> f32
{
    libm::tanf(x)
}


use vectoria::vector::*;
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_vec2_add()
    {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        let result = v1.add(&v2);
        assert_eq!(result, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_vec2_sub()
    {
        let v1 = Vec2::new(5.0, 6.0);
        let v2 = Vec2::new(3.0, 4.0);
        let result = v1.sub(&v2);
        assert_eq!(result, Vec2::new(2.0, 2.0));
    }

    #[test]
    fn test_vec2_mul_scale()
    {
        let v = Vec2::new(1.0, 2.0);
        let result = v.mul_scale(2.0);
        assert_eq!(result, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn test_vec2_dot()
    {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        let result = v1.dot(&v2);
        assert_eq!(result, 11.0);
    }

    #[test]
    fn test_vec2_length()
    {
        let v = Vec2::new(3.0, 4.0);
        let result = v.length();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_vec2_unit()
    {
        let v = Vec2::new(3.0, 4.0);
        let result = v.unit();
        assert_eq!(result, Vec2::new(0.6, 0.8));
    }

    #[test]
    fn test_vec2_project()
    {
        let v1 = Vec2::new(2.0, 3.0);
        let v2 = Vec2::new(1.0, 0.0);
        let result = v1.project(&v2);
        assert_eq!(result, Vec2::new(2.0, 0.0));
    }

    #[test]
    fn test_vec2_reflect()
    {
        let v = Vec2::new(1.0, -1.0);
        let normal = Vec2::new(0.0, 1.0);
        let result = v.reflect(&normal);
        assert_eq!(result, Vec2::new(1.0, 1.0));
    }

    #[test]
    fn test_vec3_add()
    {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1.add(&v2);
        assert_eq!(result, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vec3_sub()
    {
        let v1 = Vec3::new(7.0, 8.0, 9.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1.sub(&v2);
        assert_eq!(result, Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_vec3_mul_scale()
    {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = v.mul_scale(2.0);
        assert_eq!(result, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_vec3_dot()
    {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1.dot(&v2);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_vec3_cross()
    {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1.cross(&v2);
        assert_eq!(result, Vec3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_vec3_length()
    {
        let v = Vec3::new(1.0, 2.0, 2.0);
        let result = v.length();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_vec3_normalize()
    {
        let v = Vec3::new(1.0, 2.0, 2.0);
        let result = v.normalize();
        assert_eq!(result, Vec3::new(1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0));
    }

    #[test]
    fn test_vec3_project()
    {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        let v2 = Vec3::new(1.0, 0.0, 0.0);
        let result = v1.project(&v2);
        assert_eq!(result, Vec3::new(2.0, 0.0, 0.0));
    }

    #[test]
    fn test_vec3_reflect()
    {
        let v = Vec3::new(1.0, -1.0, 0.0);
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let result = v.reflect(&normal);
        assert_eq!(result, Vec3::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_vec4_add()
    {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(5.0, 6.0, 7.0, 8.0);
        let result = v1.add(&v2);
        assert_eq!(result, Vec4::new(6.0, 8.0, 10.0, 12.0));
    }

    #[test]
    fn test_vec4_sub()
    {
        let v1 = Vec4::new(9.0, 8.0, 7.0, 6.0);
        let v2 = Vec4::new(5.0, 4.0, 3.0, 2.0);
        let result = v1.sub(&v2);
        assert_eq!(result, Vec4::new(4.0, 4.0, 4.0, 4.0));
    }

    #[test]
    fn test_vec4_mul_scale()
    {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let result = v.mul_scale(2.0);
        assert_eq!(result, Vec4::new(2.0, 4.0, 6.0, 8.0));
    }

    #[test]
    fn test_vec4_dot()
    {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(5.0, 6.0, 7.0, 8.0);
        let result = v1.dot(&v2);
        assert_eq!(result, 70.0);
    }

    #[test]
    fn test_vec4_length()
    {
        let v = Vec4::new(1.0, 2.0, 2.0, 1.0);
        let result = v.length();
        assert!((result - 3.1622777).abs() < 1e-5, "Expected approximately 3.1622777, got {:?}", result);
    }

    #[test]
    fn test_vec4_unit()
    {
        let v = Vec4::new(1.0, 2.0, 2.0, 1.0);
        let result = v.unit();
        assert!((result.x - 0.31622776).abs() < 1e-5, "Expected x: 0.31622776, got {:?}", result.x);
        assert!((result.y - 0.6324555).abs() < 1e-5, "Expected y: 0.6324555, got {:?}", result.y);
        assert!((result.z - 0.6324555).abs() < 1e-5, "Expected z: 0.6324555, got {:?}", result.z);
        assert!((result.w - 0.31622776).abs() < 1e-5, "Expected w: 0.31622776, got {:?}", result.w);
    }

    #[test]
    fn test_vec4_project()
    {
        let v1 = Vec4::new(2.0, 3.0, 4.0, 5.0);
        let v2 = Vec4::new(1.0, 0.0, 0.0, 0.0);
        let result = v1.project(&v2);
        assert_eq!(result, Vec4::new(2.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn test_vec4_reflect()
    {
        let v = Vec4::new(1.0, -1.0, 0.0, 0.0);
        let normal = Vec4::new(0.0, 1.0, 0.0, 0.0);
        let result = v.reflect(&normal);
        assert_eq!(result, Vec4::new(1.0, 1.0, 0.0, 0.0));
    }
}

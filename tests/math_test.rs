use vectoria::math::*;
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_min()
    {
        assert_eq!(min(1.0, 2.0), 1.0);
        assert_eq!(min(2.0, 1.0), 1.0);
        assert_eq!(min(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_max()
    {
        assert_eq!(max(1.0, 2.0), 2.0);
        assert_eq!(max(2.0, 1.0), 2.0);
        assert_eq!(max(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_clamp()
    {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(clamp(-5.0, 0.0, 10.0), 0.0);
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn test_lerp()
    {
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
    }

    #[test]
    fn test_approx_equal()
    {
        assert!(approx_equal(1.0, 1.0 + EPSILON / 2.0, EPSILON));
        assert!(!approx_equal(1.0, 1.0 + EPSILON * 2.0, EPSILON));
    }

    #[test]
    fn test_degrees_to_radians()
    {
        assert!((degrees_to_radians(180.0) - std::f32::consts::PI).abs() < EPSILON);
        assert!((degrees_to_radians(360.0) - 2.0 * std::f32::consts::PI).abs() < EPSILON);
    }

    #[test]
    fn test_radians_to_degrees()
    {
        assert!((radians_to_degrees(std::f32::consts::PI) - 180.0).abs() < EPSILON);
        assert!((radians_to_degrees(2.0 * std::f32::consts::PI) - 360.0).abs() < EPSILON);
    }

    #[test]
    fn test_sin()
    {
        assert!((sin(0.0) - 0.0).abs() < EPSILON);
        assert!((sin(std::f32::consts::PI / 2.0) - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_cos()
    {
        assert!((cos(0.0) - 1.0).abs() < EPSILON);
        assert!((cos(std::f32::consts::PI) + 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_tan()
    {
        assert!((tan(0.0) - 0.0).abs() < EPSILON);
        assert!((tan(std::f32::consts::PI / 4.0) - 1.0).abs() < EPSILON);
    }
}

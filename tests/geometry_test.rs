use vectoria::geometry::*;
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_point_distance()
    {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        assert_eq!(p1.distance(&p2), 5.0);
    }

    #[test]
    fn test_line_length()
    {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        let line = Line::new(p1, p2);
        assert_eq!(line.length(), 5.0);
    }

    #[test]
    fn test_circle_area()
    {
        let center = Point::new(0.0, 0.0);
        let circle = Circle::new(center, 1.0);
        assert_eq!(circle.area(), std::f64::consts::PI);
    }

    #[test]
    fn test_rectangle_area()
    {
        let top_left = Point::new(0.0, 0.0);
        let rectangle = Rectangle::new(top_left, 3.0, 4.0);
        assert_eq!(rectangle.area(), 12.0);
    }

    #[test]
    fn test_triangle_area()
    {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(3.0, 0.0);
        let c = Point::new(3.0, 4.0);
        let triangle = Triangle::new(a, b, c);
        assert_eq!(triangle.area(), 6.0);
    }

    #[test]
    fn test_sphere_volume()
    {
        let center = Point3D::new(0.0, 0.0, 0.0);
        let sphere = Sphere::new(center, 1.0);
        assert_eq!(sphere.volume(), (4.0 / 3.0) * std::f64::consts::PI);
    }

    #[test]
    fn test_cube_volume()
    {
        let center = Point3D::new(0.0, 0.0, 0.0);
        let cube = Cube::new(center, 2.0);
        assert_eq!(cube.volume(), 8.0);
    }

    #[test]
    fn test_circle_circumference()
    {
        let center = Point::new(0.0, 0.0);
        let circle = Circle::new(center, 1.0);
        assert_eq!(circle.circumference(), 2.0 * std::f64::consts::PI);
    }

    #[test]
    fn test_rectangle_perimeter()
    {
        let top_left = Point::new(0.0, 0.0);
        let rectangle = Rectangle::new(top_left, 3.0, 4.0);
        assert_eq!(rectangle.perimeter(), 14.0);
    }

    #[test]
    fn test_triangle_perimeter()
    {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(3.0, 0.0);
        let c = Point::new(3.0, 4.0);
        let triangle = Triangle::new(a, b, c);
        assert_eq!(triangle.perimeter(), 12.0);
    }

    #[test]
    fn test_sphere_surface_area()
    {
        let center = Point3D::new(0.0, 0.0, 0.0);
        let sphere = Sphere::new(center, 1.0);
        assert_eq!(sphere.surface_area(), 4.0 * std::f64::consts::PI);
    }

    #[test]
    fn test_cube_surface_area()
    {
        let center = Point3D::new(0.0, 0.0, 0.0);
        let cube = Cube::new(center, 2.0);
        assert_eq!(cube.surface_area(), 24.0);
    }
}
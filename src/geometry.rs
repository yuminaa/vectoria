#[derive(Debug, Clone, Copy)]
pub struct Point
{
    pub x: f64,
    pub y: f64,
}

impl Point
{
    pub fn new(x: f64, y: f64) -> Self
    {
        Self { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64
    {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn translate(&mut self, dx: f64, dy: f64)
    {
        self.x += dx;
        self.y += dy;
    }
}

#[derive(Debug, Clone)]
pub struct Line
{
    pub start: Point,
    pub end: Point,
}

impl Line
{
    pub fn new(start: Point, end: Point) -> Self
    {
        Self { start, end }
    }

    pub fn length(&self) -> f64
    {
        self.start.distance(&self.end)
    }

    pub fn midpoint(&self) -> Point
    {
        Point::new(
            (self.start.x + self.end.x) / 2.0,
            (self.start.y + self.end.y) / 2.0,
        )
    }
}

#[derive(Debug, Clone)]
pub struct Circle
{
    pub center: Point,
    pub radius: f64,
}

impl Circle
{
    pub fn new(center: Point, radius: f64) -> Self
    {
        Self { center, radius }
    }

    pub fn area(&self) -> f64
    {
        std::f64::consts::PI * self.radius.powi(2)
    }

    pub fn circumference(&self) -> f64
    {
        2.0 * std::f64::consts::PI * self.radius
    }

    pub fn contains(&self, point: &Point) -> bool
    {
        self.center.distance(point) <= self.radius
    }
}

#[derive(Debug, Clone)]
pub struct Rectangle
{
    pub top_left: Point,
    pub width: f64,
    pub height: f64,
}

impl Rectangle
{
    pub fn new(top_left: Point, width: f64, height: f64) -> Self
    {
        Self { top_left, width, height }
    }

    pub fn area(&self) -> f64
    {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f64
    {
        2.0 * (self.width + self.height)
    }

    pub fn contains(&self, point: &Point) -> bool
    {
        point.x >= self.top_left.x
            && point.x <= self.top_left.x + self.width
            && point.y >= self.top_left.y
            && point.y <= self.top_left.y + self.height
    }
}

#[derive(Debug, Clone)]
pub struct Triangle
{
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle
{
    pub fn new(a: Point, b: Point, c: Point) -> Self
    {
        Self { a, b, c }
    }

    pub fn area(&self) -> f64
    {
        let s = self.perimeter() / 2.0;
        (s * (s - self.a.distance(&self.b)) * (s - self.b.distance(&self.c)) * (s - self.c.distance(&self.a))).sqrt()
    }

    pub fn perimeter(&self) -> f64 {
        self.a.distance(&self.b) + self.b.distance(&self.c) + self.c.distance(&self.a)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point3D
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D
{
    pub fn new(x: f64, y: f64, z: f64) -> Self
    {
        Self { x, y, z }
    }

    pub fn distance(&self, other: &Point3D) -> f64
    {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone)]
pub struct Sphere
{
    pub center: Point3D,
    pub radius: f64,
}

impl Sphere
{
    pub fn new(center: Point3D, radius: f64) -> Self
    {
        Self { center, radius }
    }

    pub fn volume(&self) -> f64
    {
        (4.0 / 3.0) * std::f64::consts::PI * self.radius.powi(3)
    }

    pub fn surface_area(&self) -> f64
    {
        4.0 * std::f64::consts::PI * self.radius.powi(2)
    }

    pub fn contains(&self, point: &Point3D) -> bool
    {
        self.center.distance(point) <= self.radius
    }
}

#[derive(Debug, Clone)]
pub struct Cube
{
    pub center: Point3D,
    pub side_length: f64,
}

impl Cube
{
    pub fn new(center: Point3D, side_length: f64) -> Self
    {
        Self { center, side_length }
    }

    pub fn volume(&self) -> f64
    {
        self.side_length.powi(3)
    }

    pub fn surface_area(&self) -> f64
    {
        6.0 * self.side_length.powi(2)
    }

    pub fn contains(&self, point: &Point3D) -> bool
    {
        let half_side = self.side_length / 2.0;
        point.x >= self.center.x - half_side && point.x <= self.center.x + half_side &&
            point.y >= self.center.y - half_side && point.y <= self.center.y + half_side &&
            point.z >= self.center.z - half_side && point.z <= self.center.z + half_side
    }
}

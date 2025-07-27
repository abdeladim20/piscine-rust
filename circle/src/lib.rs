#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center: Point,
	pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Circle {
            center: Point(x, y),
            radius: radius,
        }
    }
    pub fn diameter(self) -> f64 {
        self.radius * 2.0
    }
    pub fn area(self) -> f64 {
        std::f64::consts::PI * self.radius*self.radius
    }
    pub fn intersect(self, c: Circle) -> bool {
        self.radius + c.radius >= self.center.distance(c.center)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(self, p: Point) -> f64 {
        (((p.0-self.0)*(p.0-self.0)) + ((p.1-self.1)*(p.1-self.1))).sqrt()
    }
}
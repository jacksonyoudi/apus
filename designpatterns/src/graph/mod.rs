pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

pub struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    pub fn new() -> CircleBuilder {
        CircleBuilder {
            x: 0.0,
            y: 0.0,
            radius: 0.0,
        }
    }
}


impl CircleBuilder {
    pub fn x(&mut self, coordinates: f64) -> &mut CircleBuilder {
        self.x = coordinates;
        self
    }

    pub fn y(&mut self, coordinates: f64) -> &mut CircleBuilder {
        self.y = coordinates;
        self
    }
    pub fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }

    pub fn build(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius,
        }
    }
}



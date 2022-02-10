#[derive(Debug)]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
pub struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Rectangle {
    pub fn rect_area(&self) -> f32 {
        let Rectangle {
            p1: Point { x: x1, y: y1 },
            p2: Point { x: x2, y: y2 },
        } = self;
        return (y2 - y1) * (x2 - x1);
    }
}

pub fn square(p1: Point, wh: f32) -> Rectangle {
    let Point { x: x1, y: y1 } = p1;
    let p2 = Point {
        x: x1 + wh,
        y: y1 + wh,
    };
    return Rectangle { p1, p2 };
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rectangle() {
        let p1 = Point::new(0.0, 0.0);
        let sq = square(p1, 10.0);
        assert_eq!(100.0, sq.rect_area());
    }
}

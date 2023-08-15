use crate::structs::point::Point;

#[derive(Debug)]
pub struct Polyline {
    pub points: Vec<Point>,
}

impl Polyline {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }
}

impl Clone for Polyline {
    fn clone(&self) -> Self {
        Self {
            points: self.points.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::point::Point;
    use crate::structs::polyline::Polyline;

    #[test]
    fn test_polyline_new() {
        let p = Polyline::new(vec![Point::new(1, 2), Point::new(3, 4)]);
        assert_eq!(p.points[0].x, 1);
        assert_eq!(p.points[0].y, 2);
        assert_eq!(p.points[1].x, 3);
        assert_eq!(p.points[1].y, 4);
    }

    #[test]
    fn test_polyline_clone() {
        let p = Polyline::new(vec![Point::new(1, 2), Point::new(3, 4)]);
        let p2 = p.clone();

        assert_eq!(p2.points[0].x, 1);
        assert_eq!(p2.points[0].y, 2);
        assert_eq!(p2.points[1].x, 3);
        assert_eq!(p2.points[1].y, 4);

        assert_eq!(p.points[0].x, 1);
        assert_eq!(p.points[0].y, 2);
        assert_eq!(p.points[1].x, 3);
        assert_eq!(p.points[1].y, 4);
    }
}

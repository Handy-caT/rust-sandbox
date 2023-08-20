
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

impl Copy for Point {
}


#[cfg(test)]
mod tests {
    use crate::point::point::Point;

    #[test]
    fn test_point_new() {
        let p = Point::new(1, 2);
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);
    }

    #[test]
    fn test_point_clone() {
        let p = Point::new(1, 2);
        let p2 = p.clone();
        assert_eq!(p2.x, 1);
        assert_eq!(p2.y, 2);
    }

    #[test]
    fn test_point_copy() {
        let p = Point::new(1, 2);
        let p2 = p;
        assert_eq!(p2.x, 1);
        assert_eq!(p2.y, 2);

        assert_eq!(p.x, 1);
    }

    #[test]
    fn test_point_default() {
        let p = Point::default();
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);
    }
}
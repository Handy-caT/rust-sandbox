mod structs;
use structs::point::Point;
use structs::polyline::Polyline;


fn main() {
    let p = Point::new(1, 2);
    let p2 = p;

    println!("{:?} is a copy of {:?}", p2, p);
    println!("The default value of Point is {:?}", Point::default());

    let polyline = Polyline::new(vec![Point::new(1, 2), Point::new(3, 4)]);
    println!("{:?} has no copy and default value", polyline);

    let polyline2 = polyline.clone();
    println!("Only a clone {:?}", polyline2);
}

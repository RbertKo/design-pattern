struct Coordinate(u8, u8);

struct Circle {
    center: Coordinate,
}

impl Circle {
    fn new(center: Coordinate) -> Self {
        Self {
            center
        }
    }
}

fn main() {
    let my_circle = Circle::new(Coordinate(0, 0));

    println!("my center Cooldinate X : {}   Y : {}", my_circle.center.0, my_circle.center.1);
}

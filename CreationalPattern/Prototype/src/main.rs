#[derive(Clone)]
struct Coordinate(u8, u8);

#[derive(Clone)]
struct Shape {
    name: String
}


struct Circle {

}

impl Circle {

}

fn main() {
    let my_circle = Circle::new(Coordinate(0, 0));

    println!("my center Cooldinate X : {}   Y : {}", my_circle.center.0, my_circle.center.1);
}

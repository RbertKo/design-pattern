struct Coordination(u8, u8);

struct Circle {
    center: Coordination,
}

impl Circle {
    fn new(center: Coordination) -> Self {
        Self {
            center
        }
    }
}

fn main() {
    let my_circle = Circle::new(Coordination(0, 0));

    println!("my center coordination X : {}   Y : {}", my_circle.center.0, my_circle.center.1);
}

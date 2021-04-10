trait TrafficTime {
    fn getTime(&self) -> u32;
}

enum Light {
    Red,
    Yellow,
    Green,
}

impl TrafficTime for Light {
    fn getTime(&self) -> u32 {
        match *self {
            Light::Red => 17,
            Light::Yellow => 5,
            Light::Green => 15,
        }
    }
}

// fn main() {
//     let time = Light::Red.getTime();
//     print!("time: {}", time);
// }

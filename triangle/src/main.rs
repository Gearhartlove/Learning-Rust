fn main() {
    let t: ScuffedTriangle = ScuffedTriangle::tri(2, 3);
    t.print_tri();
    println!("{}", t.area());
    println!("{}", t.perimeter());
}

//c value is not correct to official triangle standards lol
#[derive(Debug)]
struct ScuffedTriangle {
    a: u32,
    b: u32,
    c: u32, //longest leg
}

impl ScuffedTriangle {
    fn tri(a_size: u32, b_size: u32) -> ScuffedTriangle {
        ScuffedTriangle {
            a: a_size,
            b: b_size,
            c: (a_size * a_size) + (b_size * b_size), //don't understand square root library yet
        }
    }
    fn perimeter(&self) -> u32 {
        self.a + self.b + self.c
    }

    //also scuffed
    fn area(&self) -> u32 {
        (self.a * self.b) / 2
    }

    fn print_tri(&self) {
        println!("{} {} {}", self.a, self.b, self.c);
    }
}

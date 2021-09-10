use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();

    let rand_color = Color(
        x: thread_rng.gen_range(0..250),
        thread_rng.gen_range(0..250),
        thread_rng.gen_range(0..250),
    );
}

struct Color {
    x: i32,
    y: i32,
    z: i32,
}

fn generate_color() -> i32 {
    32
}

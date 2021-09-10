use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();

    let rand_color = Color(
        generate_color(&mut rng),
        generate_color(&mut rng),
        generate_color(&mut rng),
    );

    println!("{:?}", rand_color);
}

#[derive(Debug)]
struct Color(i32, i32, i32);

fn generate_color<R: Rng>(rng: &mut R) -> i32 {
    rng.gen_range(0..256)
}

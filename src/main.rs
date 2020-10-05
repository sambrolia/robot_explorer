mod robot;

fn main() {
    let size = robot::Robot{ ..Default::default() }.explore_area_size();

    println!("{}", size);
}

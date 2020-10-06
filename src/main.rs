mod robot;

fn main() {
    let size = robot::Robot {
        starting_x: 0, starting_y: 0,
        ..Default::default()
    }
    .explore_area_size();

    println!("{}", size);
}

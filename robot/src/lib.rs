pub mod loc;
pub mod robot;

/// Take a number and return the sum of its digits
/// # Examples
///
/// ```
/// assert_eq!(592597, robot::get_total_explorable_size());
/// ```
pub fn get_total_explorable_size() -> i64 {
    let size = robot::Robot {
        starting_x: 0,
        starting_y: 0,
        ..Default::default()
    }
    .explore_area_size();

    return size;
}

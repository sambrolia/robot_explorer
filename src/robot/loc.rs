const OBSTICLE_VALUE: i64 = 23;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Loc {
    pub x: i64,
    pub y: i64,
}

impl Loc {
    // Check each direction and add if safe.
    pub fn find_all_routes(self) -> Vec<Loc> {
        let mut open_routes = Vec::with_capacity(4);

        if self.right().is_safe() {
            open_routes.push(self.right());
        }
        if self.left().is_safe() {
            open_routes.push(self.left());
        }
        if self.up().is_safe() {
            open_routes.push(self.up());
        }
        if self.down().is_safe() {
            open_routes.push(self.down());
        }

        return open_routes;
    }

    // Return the location 1 to the right
    fn right(&self) -> Loc {
        return Loc {
            x: self.x + 1,
            y: self.y,
        };
    }

    // Return the location 1 to the left
    fn left(&self) -> Loc {
        return Loc {
            x: self.x - 1,
            y: self.y,
        };
    }

    // Return the location 1 up
    fn up(&self) -> Loc {
        return Loc {
            x: self.x,
            y: self.y + 1,
        };
    }

    // Return the location 1 down
    fn down(&self) -> Loc {
        return Loc {
            x: self.x,
            y: self.y - 1,
        };
    }

    // Compare the sum of the coordinates digits to
    // the OBSTICLE_VALUE which denotes dangerous mines
    fn is_safe(&self) -> bool {
        let mut sum = 0;
        sum += sum_num_digits(self.x.abs());
        sum += sum_num_digits(self.y.abs());
        return sum < OBSTICLE_VALUE;
    }
}

// Take a number and return the sum of its digits
fn sum_num_digits(mut num: i64) -> i64 {
    let mut sum = 0;
    while num > 0 {
        let m = num % 10;
        sum = sum + m;
        num = num / 10;
    }
    return sum;
}

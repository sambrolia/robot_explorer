const OBSTICLE_VALUE: i64 = 23;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Loc {
    pub x: i64,
    pub y: i64,
}

impl Loc {
    pub fn find_all_routes(self) -> Vec<Loc> {
        let mut open_routes = Vec::with_capacity(4);

        // If we are at least 2 away from an absticle, 
        // add all directions and return.
        if self.is_clear() {
            open_routes.push(self.right());
            open_routes.push(self.left());
            open_routes.push(self.up());
            open_routes.push(self.down());

            return open_routes;
        }

        // If we didn't return, we are only one space
        // away from an obsticle, check each one.
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


    fn right(&self) -> Loc {
        return Loc{
            x: self.x + 1, 
            y: self.y
        };
    }
    fn left(&self) -> Loc {
        return Loc{
            x: self.x - 1, 
            y: self.y
        };
    }
    fn up(&self) -> Loc {
        return Loc{
            x: self.x, 
            y: self.y + 1
        };
    }
    fn down(&self) -> Loc {
        return Loc{
            x: self.x, 
            y: self.y - 1
        };
    }
    
    // // If we are 
    fn is_clear(&self) -> bool {
        let mut sum = 0;
        sum += sum_num_digits(self.x.abs());
        sum += sum_num_digits(self.y.abs());
        return sum < OBSTICLE_VALUE - 2;
    }

    fn is_safe(&self) -> bool {
        let mut sum = 0;
        sum += sum_num_digits(self.x.abs());
        sum += sum_num_digits(self.y.abs());
        return sum < OBSTICLE_VALUE;
    }
}

fn sum_num_digits(mut num: i64) -> i64 {
    let mut sum = 0;
    while num > 0 {    
        let m = num % 10;    
        sum = sum + m;    
        num = num / 10;    
    }
    return sum;
}
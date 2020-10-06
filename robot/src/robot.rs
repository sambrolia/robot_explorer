use std::collections::HashMap;

use crate::loc::*;

pub struct Robot {
    // Create a HashMap to ensure duplicates
    // aren't processed twice.
    pub loc_visited: HashMap<loc::Loc, bool>,

    // Use a stack because simple recursion would
    // break the call stack max size.
    pub loc_stack: Vec<loc::Loc>,

    // Keep track of how many locations we visit.
    pub count: i64,

    // starting_loc
    pub starting_x: i64,
    pub starting_y: i64,
}

impl Robot {
    pub fn explore_area_size(&mut self) -> i64 {
        // Initialise starting point
        let loc = loc::Loc {
            x: self.starting_x,
            y: self.starting_y,
        };

        // Push the starting point onto the stack
        self.loc_stack.push(loc);
        // Pop each element in stack to process.
        while let Some(top) = self.loc_stack.pop() {
            // Find all valid routes from that location
            let routes = top.find_all_routes();
            for loc in routes {
                // If its not in the map, its a new location
                // so add it to the map and the stack,
                // then increment the counter
                if !self.loc_visited.contains_key(&loc) {
                    self.loc_visited.insert(loc, true);
                    self.loc_stack.push(loc);
                    self.count += 1;
                }
            }
        }
        return self.count;
    }
}

impl Default for Robot {
    fn default() -> Robot {
        Robot {
            loc_visited: HashMap::new(),
            loc_stack: Vec::new(),
            count: 0,
            starting_x: 0,
            starting_y: 0,
        }
    }
}

pub fn get_total_explorable_size() -> i64 {
    let size = Robot {
        starting_x: 0,
        starting_y: 0,
        ..Default::default()
    }
    .explore_area_size();

    return size;
}

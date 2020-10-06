
pub mod loc {
    const OBSTICLE_VALUE: i64 = 23;

    #[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
    pub struct Loc {
        pub x: i64,
        pub y: i64,
    }

    impl Loc {
    
        /// Take a number and return the sum of its digits
        /// # Examples
        ///
        /// ```
        /// # use robot::loc::loc::Loc;
        /// let loc = Loc{x: 4, y: 5};
        /// let result = vec![
        ///     Loc{x: 5, y: 5},
        ///     Loc{x: 3, y: 5},
        ///     Loc{x: 4, y: 6},
        ///     Loc{x: 4, y: 4},
        /// ];
        /// assert_eq!(result, loc.find_all_routes());
        /// 
        /// let loc = Loc{x: 56, y: 57};
        /// let result = vec![
        ///     Loc{x: 55, y: 57},
        ///     Loc{x: 56, y: 56},
        /// ];
        /// assert_eq!(result, loc.find_all_routes());
        /// ```
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

        /// Compare the sum of the coordinates digits to
        /// the OBSTICLE_VALUE which denotes dangerous mines. 
        /// # Examples
        ///
        /// ```
        /// # use robot::loc::loc::Loc;
        /// let loc = Loc{x: 3, y: 3};
        /// assert_eq!(true, loc.is_safe());
        /// 
        /// let loc = Loc{x: 57, y: 82};
        /// assert_eq!(true, loc.is_safe());
        /// 
        /// let loc = Loc{x: 57, y: 88};
        /// assert_eq!(false, loc.is_safe());
        /// 
        /// ```
        pub fn is_safe(&self) -> bool {
            let mut sum = 0;
            sum += sum_num_digits(self.x.abs());
            sum += sum_num_digits(self.y.abs());
            return sum <= OBSTICLE_VALUE;
        }
    }

    /// Take a number and return the sum of its digits
    /// # Examples
    ///
    /// ```
    /// # use robot::loc::loc::sum_num_digits;
    /// assert_eq!(9, robot::loc::loc::sum_num_digits(54));
    /// assert_eq!(43, robot::loc::loc::sum_num_digits(32523523554112));
    /// ```
    pub fn sum_num_digits(mut num: i64) -> i64 {
        let mut sum = 0;
        while num > 0 {
            let m = num % 10;
            sum = sum + m;
            num = num / 10;
        }
        return sum;
    }
}
use gcd::Gcd;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

impl BucketStats {
    pub fn new(moves: u8, goal_bucket: Bucket, other_bucket: u8) -> Self {
        Self {
            moves,
            goal_bucket,
            other_bucket,
        }
    }
}

pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if goal > capacity_1.max(capacity_2) || goal % capacity_1.gcd(capacity_2) != 0 {
        None
    } else {
        let ((from_bucket, to_bucket, from_capacity, to_capacity, mut from), mut to, mut actions) = (
            match start_bucket {
                Bucket::One => (Bucket::One, Bucket::Two, capacity_1, capacity_2, capacity_1),
                Bucket::Two => (Bucket::Two, Bucket::One, capacity_2, capacity_1, capacity_2),
            },
            0,
            1,
        );

        if goal == from_capacity {
            return Some(BucketStats::new(1, from_bucket, 0));
        }

        if goal == to_capacity {
            return Some(BucketStats::new(2, to_bucket, from_capacity));
        }

        loop {
            if from == goal {
                return Some(BucketStats::new(actions, from_bucket, to));
            }

            if to == goal {
                return Some(BucketStats::new(actions, to_bucket, from));
            }

            if from == 0 {
                from = from_capacity;
                actions += 1;
            }

            if to == to_capacity {
                to = 0;
                actions += 1;
            }

            let pour = from.min(to_capacity - to);
            from -= pour;
            to += pour;
            actions += 1;
        }
    }
}

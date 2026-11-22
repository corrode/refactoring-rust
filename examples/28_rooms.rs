// ANCHOR: listing
//! Group operations that share a collection of room occupancies.
//!
//! These iterator functions are already clear. Explore whether a `Rooms`
//! type gives the operations a useful home. You may change the public API,
//! but keep the calculation results and adapt the callers and tests.
//! Keep the free functions if the extra structure does not earn its place.

pub struct RoomOccupancy {
    pub adults: i32,
    pub children: Vec<i32>,
}

pub fn adults_total(rooms: &[RoomOccupancy]) -> i32 {
    rooms.iter().map(|r| r.adults).sum()
}

pub fn children_total(rooms: &[RoomOccupancy]) -> i32 {
    rooms.iter().map(|r| r.children.len() as i32).sum()
}

// Zero is the empty default, not a lower bound. This does not validate counts.
pub fn max_adults_in_a_room(rooms: &[RoomOccupancy]) -> i32 {
    rooms.iter().map(|r| r.adults).max().unwrap_or(0)
}

pub fn child_ages(rooms: &[RoomOccupancy]) -> Vec<i32> {
    rooms
        .iter()
        .flat_map(|r| r.children.iter().copied())
        .collect()
}
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_rooms_use_the_existing_defaults() {
        let rooms = [];
        assert_eq!(adults_total(&rooms), 0);
        assert_eq!(children_total(&rooms), 0);
        assert_eq!(max_adults_in_a_room(&rooms), 0);
        assert!(child_ages(&rooms).is_empty());
    }

    #[test]
    fn aggregates_mixed_counts_without_validation() {
        let rooms = vec![
            RoomOccupancy {
                adults: -2,
                children: vec![3, -1],
            },
            RoomOccupancy {
                adults: 0,
                children: vec![],
            },
            RoomOccupancy {
                adults: 5,
                children: vec![6],
            },
        ];
        assert_eq!(adults_total(&rooms), 3);
        assert_eq!(children_total(&rooms), 3);
        assert_eq!(max_adults_in_a_room(&rooms), 5);
        assert_eq!(child_ages(&rooms), vec![3, -1, 6]);
    }

    #[test]
    fn negative_maximum_is_not_clamped_to_zero() {
        let rooms = [
            RoomOccupancy {
                adults: -5,
                children: vec![],
            },
            RoomOccupancy {
                adults: -2,
                children: vec![],
            },
        ];
        assert_eq!(adults_total(&rooms), -7);
        assert_eq!(max_adults_in_a_room(&rooms), -2);
    }

    #[test]
    fn child_ages_preserve_room_order_and_duplicates() {
        let rooms = [
            RoomOccupancy {
                adults: 1,
                children: vec![8, 3, 8],
            },
            RoomOccupancy {
                adults: 2,
                children: vec![],
            },
            RoomOccupancy {
                adults: 1,
                children: vec![3, 1],
            },
        ];
        assert_eq!(children_total(&rooms), 5);
        assert_eq!(child_ages(&rooms), vec![8, 3, 8, 3, 1]);
    }
}
// ANCHOR_END: tests

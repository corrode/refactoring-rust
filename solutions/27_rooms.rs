// ANCHOR: listing
//! Expose an owned collection as the domain API instead of free functions.
//!
//! This changes how callers construct and query rooms. Choose it only when
//! owning the collection and grouping its operations is useful to callers.

#[derive(Debug, PartialEq, Eq)]
pub struct RoomOccupancy {
    pub adults: i32,
    pub children: Vec<i32>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Rooms(Vec<RoomOccupancy>);

impl Rooms {
    // Ownership is not validation: negative counts and ages remain accepted.
    pub fn new(rooms: Vec<RoomOccupancy>) -> Self {
        Self(rooms)
    }

    pub fn adults_total(&self) -> i32 {
        self.0.iter().map(|room| room.adults).sum()
    }

    pub fn children_total(&self) -> i32 {
        self.0.iter().map(|room| room.children.len() as i32).sum()
    }

    pub fn max_adults_in_a_room(&self) -> i32 {
        // Zero is the empty default, not a lower bound.
        self.0.iter().map(|room| room.adults).max().unwrap_or(0)
    }

    pub fn child_ages(&self) -> Vec<i32> {
        self.0
            .iter()
            .flat_map(|room| room.children.iter().copied())
            .collect()
    }
}
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_rooms_use_the_existing_defaults() {
        for rooms in [Rooms::new(vec![]), Rooms::default()] {
            assert_eq!(rooms.adults_total(), 0);
            assert_eq!(rooms.children_total(), 0);
            assert_eq!(rooms.max_adults_in_a_room(), 0);
            assert!(rooms.child_ages().is_empty());
        }
    }

    #[test]
    fn aggregates_mixed_counts_without_validation() {
        let rooms = Rooms::new(vec![
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
        ]);
        assert_eq!(rooms.adults_total(), 3);
        assert_eq!(rooms.children_total(), 3);
        assert_eq!(rooms.max_adults_in_a_room(), 5);
        assert_eq!(rooms.child_ages(), vec![3, -1, 6]);
    }

    #[test]
    fn negative_maximum_is_not_clamped_to_zero() {
        let rooms = Rooms::new(vec![
            RoomOccupancy {
                adults: -5,
                children: vec![],
            },
            RoomOccupancy {
                adults: -2,
                children: vec![],
            },
        ]);
        assert_eq!(rooms.adults_total(), -7);
        assert_eq!(rooms.max_adults_in_a_room(), -2);
    }

    #[test]
    fn child_ages_preserve_room_order_and_duplicates() {
        let rooms = Rooms::new(vec![
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
        ]);
        assert_eq!(rooms.children_total(), 5);
        assert_eq!(rooms.child_ages(), vec![8, 3, 8, 3, 1]);
    }

    #[test]
    fn constructor_owns_the_collection_beyond_the_source_scope() {
        let rooms = {
            let source = vec![RoomOccupancy {
                adults: 3,
                children: vec![3, 6],
            }];
            Rooms::new(source)
        };
        assert_eq!(rooms.adults_total(), 3);
        assert_eq!(rooms.children_total(), 2);
        assert_eq!(rooms.max_adults_in_a_room(), 3);
        assert_eq!(rooms.child_ages(), vec![3, 6]);
    }
}
// ANCHOR_END: tests

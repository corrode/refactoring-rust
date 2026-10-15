//! Put room-wide aggregates on a domain type.

#[derive(Debug, PartialEq, Eq)]
pub struct RoomOccupancy {
    pub adults: i32,
    pub children: Vec<i32>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Rooms(Vec<RoomOccupancy>);

impl Rooms {
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
        self.0.iter().map(|room| room.adults).max().unwrap_or(0)
    }

    pub fn child_ages(&self) -> Vec<i32> {
        self.0
            .iter()
            .flat_map(|room| room.children.iter().copied())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Rooms {
        Rooms::new(vec![
            RoomOccupancy {
                adults: 3,
                children: vec![3, 6],
            },
            RoomOccupancy {
                adults: 1,
                children: vec![],
            },
        ])
    }

    #[test]
    fn computes_room_aggregates() {
        let rooms = sample();
        assert_eq!(rooms.adults_total(), 4);
        assert_eq!(rooms.children_total(), 2);
        assert_eq!(rooms.max_adults_in_a_room(), 3);
        assert_eq!(rooms.child_ages(), vec![3, 6]);
    }

    #[test]
    fn empty_rooms_use_the_existing_defaults() {
        let rooms = Rooms::default();
        assert_eq!(rooms.adults_total(), 0);
        assert_eq!(rooms.children_total(), 0);
        assert_eq!(rooms.max_adults_in_a_room(), 0);
        assert!(rooms.child_ages().is_empty());
    }
}

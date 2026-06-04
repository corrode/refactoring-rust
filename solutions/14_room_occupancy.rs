//! Compute totals across a list of room occupancies.
//!
//! Solution highlights:
//! - Each helper collapses to one iterator chain - the shape is visible at a glance.
//! - `max` returns `Option`, so we restore the original "0 if empty" with `unwrap_or(0)`.
//! - `flat_map` + `copied` flattens the nested `&Vec<i32>` into owned values.

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

pub fn max_adults_in_a_room(rooms: &[RoomOccupancy]) -> i32 {
    rooms.iter().map(|r| r.adults).max().unwrap_or(0)
}

pub fn child_ages(rooms: &[RoomOccupancy]) -> Vec<i32> {
    rooms
        .iter()
        .flat_map(|r| r.children.iter().copied())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<RoomOccupancy> {
        vec![
            RoomOccupancy {
                adults: 3,
                children: vec![3, 6],
            },
            RoomOccupancy {
                adults: 1,
                children: vec![],
            },
        ]
    }

    #[test]
    fn sums_adults() {
        assert_eq!(adults_total(&sample()), 4);
    }

    #[test]
    fn sums_children() {
        assert_eq!(children_total(&sample()), 2);
    }

    #[test]
    fn finds_max_adults() {
        assert_eq!(max_adults_in_a_room(&sample()), 3);
    }

    #[test]
    fn flattens_ages() {
        assert_eq!(child_ages(&sample()), vec![3, 6]);
    }
}

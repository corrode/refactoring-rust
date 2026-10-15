//! Compute totals across a list of room occupancies. Notice the
//! repetition between the helpers - every one of them follows the same
//! shape.

pub struct RoomOccupancy {
    pub adults: i32,
    pub children: Vec<i32>,
}

pub fn adults_total(rooms: &[RoomOccupancy]) -> i32 {
    let mut total = 0;
    for room in rooms {
        total += room.adults;
    }
    total
}

pub fn children_total(rooms: &[RoomOccupancy]) -> i32 {
    let mut total = 0;
    for room in rooms {
        total += room.children.len() as i32;
    }
    total
}

pub fn max_adults_in_a_room(rooms: &[RoomOccupancy]) -> i32 {
    let mut max = 0;
    for room in rooms {
        if room.adults > max {
            max = room.adults;
        }
    }
    max
}

pub fn child_ages(rooms: &[RoomOccupancy]) -> Vec<i32> {
    let mut ages = Vec::new();
    for room in rooms {
        for age in &room.children {
            ages.push(*age);
        }
    }
    ages
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

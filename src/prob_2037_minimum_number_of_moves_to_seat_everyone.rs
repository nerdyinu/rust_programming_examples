#[allow(dead_code)]
pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
    let mut seats = seats;
    let mut students = students;
    seats.sort_unstable();
    students.sort_unstable();
    let mut moves = 0;
    seats
        .iter()
        .zip(students.iter())
        .for_each(|(seat, student)| {
            moves += (seat - student).abs();
        });
    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let seats = vec![3, 1, 5];
        let students = vec![2, 7, 4];
        assert_eq!(min_moves_to_seat(seats, students), 4);
    }

    #[test]
    fn test_example_2() {
        let seats = vec![4, 1, 5, 9];
        let students = vec![1, 3, 2, 6];
        assert_eq!(min_moves_to_seat(seats, students), 7);
    }

    #[test]
    fn test_example_3() {
        let seats = vec![2, 2, 6, 6];
        let students = vec![1, 3, 2, 6];
        assert_eq!(min_moves_to_seat(seats, students), 4);
    }
}

// pub struct Natural {
//     current_value: u128,
// }

// pub trait Set<T> {
//     fn is_valid(value: T) -> bool;
// }

// impl Iterator for Natural {
//     type Item = u128;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(self.current_value)
//     }
// }

// impl Natural {
//     fn new() -> Natural {
//         Natural { current_value: 0 }
//     }
// }

#[test]
fn test_set() {
    let natural = Natural::new();
    let even = Even::new();
    let odd = Odd::new();
    let one_digit_nums = Set::new([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let natural_even = Set::union(natural, even);
}

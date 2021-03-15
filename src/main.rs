mod two_sum;
mod add_two_numbers;

fn main() {
    let r = two_sum::two_sum(vec![3, 2, 4], 6);
    print!("{:?}", r);
}

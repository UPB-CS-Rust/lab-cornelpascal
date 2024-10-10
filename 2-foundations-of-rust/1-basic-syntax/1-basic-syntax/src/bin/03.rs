fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut max :i32 = 0;
    let mut min :i32 = 1000;
    let mut i :i32;

    for i in 0..input.len() {
        if input[i] > max {
            max = input[i];
        }
        if input[i] < min {
            min = input[i];
        }
    }

    println!("{} is largest and {} is smallest", max, min);
}

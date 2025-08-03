
fn filter_even<F>(nums: Vec<i32>, cb: F)
where
    F: Fn(i32),
{
    for n in nums {
        if n % 2 == 0 {
            cb(n);
        }
    }
}

fn main() {
    filter_even(vec![1, 2, 3, 4], |n| {
        println!("Even: {}", n);
    });
}

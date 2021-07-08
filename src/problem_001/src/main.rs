fn main() {
    let mut vec: Vec<i32> = (0..1000).collect();

    vec.retain(|&x| x % 3 == 0 || x % 5 == 0);

    let sum: i32 = vec.iter().sum();

    println!("{:?}", sum);
}


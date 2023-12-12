use rand::Rng;

// create a function that takes a vector of numbers and returns the sum of the squares of those numbers
pub fn sum_of_squares(vec: Vec<i32>) -> i32 {
    let mut sum = 0;
    for x in vec {
        sum += x * x;
    }
    sum
}

// create a function to return random numbers vector
pub fn random_numbers_vector() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for _ in 0..10 {
        vec.push(rng.gen_range(1..101));
    }
    vec
}

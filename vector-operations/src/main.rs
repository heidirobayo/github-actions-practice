fn main() {
    // instantiate  a  new vector
    let mut vec = Vec::new();
    // push elements  into  the  vector
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("print the vector using for loop");
    // iterate the vector with for loop
    for x in &vec {
        println!("{}", x);
    }

    println!("\n print the vector using while loop");
    // iterate the vector with while loop
    while let Some(x) = vec.pop() {
        println!("{}", x);
    }

    print!("print the sum of all squares of the vector: ");

    let sum = vector_operations::sum_of_squares(vec![1, 2, 3]);
    println!("sum all = {}", sum);

    println!("Get the vector of random numbers: \n");
    let vec = vector_operations::random_numbers_vector();
    println!("The vector of random numbers is => {:?}", vec);

    println!("Get slices from a vector: \n");
    let vec = vector_operations::get_slices(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    println!("The slices of the vector are => {:?}", vec);
}

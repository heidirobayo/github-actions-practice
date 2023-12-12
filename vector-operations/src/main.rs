fn main() {
    // instantiate  a  new vector
    let mut vec = Vec::new();
    // push elements  into  the  vector
    vec.push(1);
    vec.push(2);
    vec.push(3);

    
    // iterate the vector with for loop
    for x in &vec {
        println!("{}", x);
    }

    // iterate the vector with while loop
    while let Some(x) = vec.pop() {
        println!("{}", x);
    }
}

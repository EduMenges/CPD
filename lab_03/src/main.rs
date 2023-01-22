use radix_sort::*;

fn main() {
    let mut test: Vec<String> = vec!["dab".to_string(), "add".to_string(), "cab".to_string(), "fad".to_string(), "fee".to_string(), "bad".to_string(), "dad".to_string()];

    radix_sort(&mut test);
    
    println!("Hello, world!");
}

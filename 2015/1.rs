fn main() {
    let floor = "Hello world!";
    let vector = Vec<char> = floor.chars().collect();
    for i in vector {
        println!("{}", i);
    }
}

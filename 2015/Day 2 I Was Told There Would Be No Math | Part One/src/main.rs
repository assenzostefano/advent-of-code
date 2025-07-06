use std::fs::read_to_string;

fn calculation(l: u32, w: u32, h: u32) -> u32{
    let front_retro_box = l * w;
    let right_left_box = w * h;
    let above_over_box = h * l;

    let total_surface_area = 2 * front_retro_box + 2 * right_left_box + 2 * above_over_box;
    
    let list = [front_retro_box, right_left_box, above_over_box];
    let min_number = list.iter().min().unwrap(); 
    let result = total_surface_area + min_number;

    result
}

fn main() {
    let file_path = "src/text.txt";
    let mut calcolo_elfi: u32 = 0;
    let contents: Vec<_> = read_to_string(file_path)
        .expect("Should have been able to read the file")
        .trim()
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect();  // gather them together into a vector

    println!("{:?}", contents);

    for i in &contents {
        println!("{:?}", i);
        
        let dimensioni: Vec<u32> = i
            .split('x')
            .map(|x| x.parse().unwrap())
            .collect();
        let l = dimensioni[0];
        let w = dimensioni[1];
        let h = dimensioni[2];
                 
        println!("Lunghezza: {}, Width: {}, Height: {}", l, w, h);
        
        let result = calculation(l, w, h);
        calcolo_elfi = calcolo_elfi + result;
        println!("{:?}", result);
    };

    println!("{}", calcolo_elfi);
}

fn main() {
    /*
    Adding comments to your code is a good practice.
    Comments are ignored by the compiler and are not executed. 
    */
    let l = 8;
    let w = 5;
    let h = 3.5;
    let area = l as f32 * w as f32;
    let volume = area * h as f32;

    println!("The area of the rectangle is {}", area);
    println!("The Volume of the rectangle is {}", volume);
}
use std::io;
fn main() {
    let mut status = String::from("");
    let mut count = 1;
    loop {
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        count += 1;
        io::stdin()
            .read_line(&mut status)
            .expect("Failed to read line");
        let trimed = status.trim();
        if trimed == "The letter e" {
            println!("Number of trials: {}", count);
            break;
        } else {
            continue;
        }
    }
}

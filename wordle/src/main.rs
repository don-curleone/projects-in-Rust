use std::io;
use std::process;

fn check(og_word: &str, your_word: &str) -> (Vec<u8>, Vec<u8>) {

    let og_word = og_word.to_lowercase();
    let your_word = your_word.to_lowercase();

    let mut full_indices: Vec<u8> = Vec::new();
    let mut partial_indices: Vec<u8> = Vec::new();


    if og_word == your_word{
        println!("you win!");
        process::exit(0);
    }

    for i in 0..5{
        if og_word.as_bytes()[i] == your_word.as_bytes()[i]{
            full_indices.push(i as u8);
        } else if og_word.contains(your_word.chars().nth(i).unwrap()){
            partial_indices.push(i as u8);
        }
    }

    (full_indices, partial_indices)

}


fn main() {

    let word: String = "scale".to_string();
    // let grid: [char; 5] = ['_'; 5];


    for round in 0..6{

        println!("{}:", round + 1);

        // take input
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let input = input.trim();

        // validate input
        if input.chars().count() != 5{
            print!("enter a 5 lettered word\n");
            continue;
        }
         
        print!("\n");

        let (full, partial) = check(&word, &input);

        // println!("{:?}, {:?}", full, partial);

        for letter in input.chars(){
            print!("{letter} ");
        }
        print!("\n");

        for i in 0..5{
            // print!("{letter} ");
            if full.contains(&(i as u8)){
                // grid[i] = full.nth(i);
                print!("{} ", input.chars().nth(i).unwrap());
            } else if partial.contains(&(i as u8)){
                print!("* ");
            } else {
                print!("_ ",);
            }
        }

        print!("\n\n");

    }

    println!("Better luck next time!");
    
}

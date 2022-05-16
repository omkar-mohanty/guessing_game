use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    let secret_number:u32 = rand::thread_rng().gen_range(1..101);
    println!("Guessing Game");
   

    loop{
        let mut guess = String::new();

        println!("Ether a number");
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");


        let guess:u32 =match guess.trim().parse::<u32>(){
            Ok(num)=>num,
            Err(_) => continue,
        };

        println!("You Guessed {}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less=>println!("Less THan the number"),
            Ordering::Greater=>println!("Greater"),
            Ordering::Equal=>{
                println!("Correct!!");
                break;
            }
        }
    }
}

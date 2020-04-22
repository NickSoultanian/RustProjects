extern crate rand;
use rand::Rng;
use std::io;
use std::process::exit;

fn main() {
    let rock = &String::from("Rock");
    let paper = &String::from("Paper");
    let scissor = &String::from("Scissor");

    let mut input = String::new();
    println!("Type Rock, Paper, or Scissor");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Success! you said : {}", input);
        },
        Err(e) => println!("whoops: {}", e)

    }
    let mut userInput = 0;

    if input.trim() == rock {

         userInput = 0;
    }
    else if input.trim() == paper  {

         userInput = 1;
    }
    else if input.trim() == scissor {

         userInput = 2;
    }

    //cpu
    let mut rng = rand::thread_rng();
    let cpuChoice = rng.gen_range(0, 3);
    println!("Cpu chose this: {}", cpuChoice);
    println!("Player Chose: {}", userInput);

    if userInput == 0 {
        if cpuChoice == 0 {
            //draw
        }
        else if cpuChoice == 1 {
            //cpu win
            println!("You Lost!");
            exit(0);
        }
        else if cpuChoice == 2{
            //player win
            println!("Yay you won!");
            exit(0);
        }
    }
    if userInput == 1 {
        if cpuChoice == 0 {
            //player win
            println!("Yay you won!");
            exit(0);
        }
        else if cpuChoice == 1 {
            //draw
        }
        else{
            //cpu win
            println!("You Lost!");
            exit(0);
        }
    }
    if userInput == 2 {
        if cpuChoice == 0 {
            //cpu win
            println!("You Lost!");
            exit(0);
        }
        else if cpuChoice == 1 {
            //player win
            println!("Yay you won!");
            exit(0);
        }
        else{
            //draw
        }
    }

}

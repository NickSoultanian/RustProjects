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
    let mut user_input = 0;

    if input.trim() == rock {

         user_input = 0;
    }
    else if input.trim() == paper  {

         user_input = 1;
    }
    else if input.trim() == scissor {

         user_input = 2;
    }

    //cpu
    let mut rng = rand::thread_rng();
    let cpu_choice = rng.gen_range(0, 3);
    let mut cpu_string = String::from("");
    if cpu_choice == 0{
        cpu_string = String::from("Rock")
    }
    else if cpu_choice == 1{
        cpu_string = String::from("Paper")
    }
    else if cpu_choice == 2{
        cpu_string = String::from("Scissor")
    }
    println!("Cpu chose this: {}", cpu_string);
    println!();

    if user_input == 0 {
        if cpu_choice == 0 {
            //draw
            println!("It's a draw, Try Again!");
            main();
        }
        else if cpu_choice == 1 {
            //cpu win
            println!("You Lost!");
            exit(0);
        }
        else if cpu_choice == 2{
            //player win
            println!("Yay you won!");
            exit(0);
        }
    }
    if user_input == 1 {
        if cpu_choice == 0 {
            //player win
            println!("Yay you won!");
            exit(0);
        }
        else if cpu_choice == 1 {
            //draw
            println!("It's a draw, Try Again!");
            main();
        }
        else{
            //cpu win
            println!("You Lost!");
            exit(0);
        }
    }
    if user_input == 2 {
        if cpu_choice == 0 {
            //cpu win
            println!("You Lost!");
            exit(0);
        }
        else if cpu_choice == 1 {
            //player win
            println!("Yay you won!");
            exit(0);
        }
        else{
            //draw
            println!("It's a draw, Try Again!");
            main();
        }
    }

}

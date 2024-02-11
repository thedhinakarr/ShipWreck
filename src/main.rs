//Ship wreck multiplayer game. Let's see how the development varies in rust.

use rpassword;
use std::io::{self};

fn main() {
    let boat1: i32;
    let boat2: i32;
    let mut countera: i32 = 0;
    let mut counterb: i32 = 0;

    let mut grid: [i32; 50] = [0; 50];

    //Take player 1's input to the grid.
    println!("Player 1, enter a number between 0 and 47:");
    let input1 = rpassword::read_password().unwrap();
    let input_str = String::from_utf8(input1.into()).unwrap();
    boat1 = input_str
        .trim()
        .parse()
        .expect("Please enter a valid number");

    //Take player 2's input to the grid
    println!("Player 2, enter a number between 0 and 47:");
    let input2 = rpassword::read_password().unwrap();
    let input_str = String::from_utf8(input2.into()).unwrap();
    boat2 = input_str
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let mut b: usize;
    for i in boat1..(boat1 + 3) {
        b = i as usize;
        grid[b] = 1;
    }

    for i in boat2..(boat2 + 3) {
        b = i as usize;
        grid[b] = 1;
    }

    //Game Logic
    let mut option: i32 = 1;
    let mut hb1: [i32; 3] = [0; 3];
    let mut hb2: [i32; 3] = [0; 3];

    loop {
        if option == 1 {
            countera = countera + 1;
            println!("Player 1 attack, enter index: ");
            let mut attack1 = String::new();
            io::stdin()
                .read_line(&mut attack1)
                .expect("Failed to read line");
            let attack1: i32 = attack1.trim().parse().expect("Please enter a valid number");

            if attack1 == boat2 {
                if hb2[0] == 0 {
                    println!("Good job! boat 2 hit.🚢");
                    hb2[0] = 1;
                } else {
                    println!("already hit");
                }
            } else if attack1 == (boat2 + 1) {
                if hb2[1] == 0 {
                    println!("Good job! boat 2 hit🚢");
                    hb2[1] = 1;
                } else {
                    println!("You've already hit this, you dip shit.");
                }
            } else if attack1 == (boat2 + 2) {
                if hb2[2] == 0 {
                    println!("Good job! boat 2 hit.🚢");
                    hb2[2] = 1;
                } else {
                    println!("You've already hit this, you dip shit.");
                }
            } else {
                println!("You suck, boat not hit.");
            }
            if hb2[0] == 1 && hb2[1] == 1 && hb2[2] == 1 {
                println!("You've won Player 1. You've destroyed player 2's boat in {:?} attempts. Congrats!",countera);
                break;
            }

            option = 2;
        } else {
            counterb = counterb + 1;
            println!("Player 2 attack, enter index: ");
            let mut attack2 = String::new();
            io::stdin()
                .read_line(&mut attack2)
                .expect("Failed to read line");
            let attack2: i32 = attack2.trim().parse().expect("Please enter a valid number");

            if attack2 == boat1 {
                if hb1[0] == 0 {
                    println!("Good job! boat 1 hit.");
                    hb1[0] = 1;
                } else {
                    println!("YOu've already hit this you dip shit.");
                }
            } else if attack2 == (boat1 + 1) {
                if hb1[1] == 0 {
                    println!("Good job! boat 1 hit.🚢");
                    hb1[1] = 1;
                } else {
                    println!("You've already hit this you dip shit.");
                }
            } else if attack2 == (boat1 + 2) {
                if hb1[2] == 0 {
                    println!("Good job! boat 1 hit.🚢");
                    hb1[2] = 1;
                } else {
                    println!("You've already hit this you dip shit. Keep wasting your attempts and lose.")
                }
            } else {
                println!("You suck at this. Boat not hit.")
            }
            if hb2[0] == 1 && hb2[1] == 1 && hb2[2] == 1 {
                println!("You've won Player 1. You've destroyed player 2's boat in {:?} attempts. Congrats!",countera);
                break;
            }
            option = 1;
        }
    }
}

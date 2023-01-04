use std::io;

fn main() {
    loop {
        println!("Is there a bug? (Y or N)");
        let boolean1: bool = get_player_input();
        if boolean1 {
            do_you_know_the_problem();
        } else {
            println!("You're lying.");
            do_you_know_the_problem();
        }
        println!("Did it create more bugs? (Y or N)");
        let boolean2: bool = get_player_input();
        if boolean2 {
            do_you_know_the_problem();
        } else {
            println!("Are you tired of debugging? (Y or N)");
            let boolean3: bool = get_player_input();
            if !boolean3 {
                println!("Try and break something");
            } else {
                println!("Your code is still buggy");
                break;
            }
        }
    }

        let mut _final_say: String = String::new();
        io::stdin().read_line(&mut _final_say).expect("Well we are done anyway...");
}

fn do_you_know_the_problem() {
    println!("Do you know the problem? (Y or N)");
    let first_val: bool = get_player_input();
    if first_val {
        println!("Fix it.");
    } else {
        println!("Try something random.");
    }
    println!("Did it work? (Y or N)");
    let second_val: bool = get_player_input();
    if second_val {
        return;
    } else {
        do_you_know_the_problem();
    }
}

fn get_player_input() -> bool {
    let mut player_input: String = String::new();
    io::stdin().read_line(&mut player_input).expect("Failed to read line");
    let return_value: &String = &player_input.trim().to_lowercase();
    if return_value == "y"{
        return true;
    } else if return_value == "n" {
        return false;
    } else {
        return false;
    }
}
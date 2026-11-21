use std::io;

fn main() {
    println!("You see two doors. Choose one! One at the right, the other at the left.");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    if choice.trim() == "left"{
        println!("there was a pit hole. You fell.")
    } else if choice.trim() == "right"{
        println!("There is a dragon in front of you. What do you do?");
        let mut secondchoice = String::new();
        io::stdin().read_line(&mut secondchoice).unwrap();
        if secondchoice.trim() == "attack" {
            println!(" You don t have any weapon and your stuck.")
        }
    }else if choice.trim() == "back"{
        println!("you found a sword! You came back in front of the doors.What do you do now?");
        let mut choice3 = String::new();
        io::stdin().read_line(&mut choice3).unwrap();
        if choice3.trim() == "left"{
            println!("there was a pit hole. You fell.");
        } else if choice3.trim() == "right"{
            println!("There is a dragon in front of you. What do you do?");
            let mut choice4 = String::new();
            io::stdin().read_line(&mut choice4).unwrap();
            if choice4.trim() == "attack" {
                println!(" You struck him with your sword. He got scared and flew.\n you won!");
            }else{
                println!("Game over");
            }
        }
        } else{println!("Game over")}
        
}
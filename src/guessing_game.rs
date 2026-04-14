use std::io;
use std::cmp::Ordering; //ordering is a enum (its a type that can be  muliple types)
use rand::Rng;

pub fn guess(){   //pub -> made function public
    println!("Welcome to Guess a number game!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop{
        let mut guess = String::new(); //needs to be local
        println!("Guess a number:");

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
        let guess:u32 = guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

    match guess.cmp(&secret_number) {  //match -> compares patterns
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
        println!("You win!");
        break;
    },
    }  
    }

}

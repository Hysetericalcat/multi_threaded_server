mod trait_shapes;

fn main() {
  let mut shape = trait_shapes::::new(String::from("Akshat"), 2000);
  user1.deposit(300);
  match user1.withdraw(1300){
    Ok(result) => println!("Balance: {}", result),
    Err(e) => println!("Error: {}", e)
  };
  user1.display();
}

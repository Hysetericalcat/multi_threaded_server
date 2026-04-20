pub struct Account {
   pub name:String,
   pub amount:u64,
}

impl Account {
    pub fn new(name:String, amount: u64) -> Account {
        let user = Account{name,amount:amount};
        user
    }
    

    pub fn deposit(&mut self,deposit:u64){ //&self -> for referencing
        self.amount = self.amount + deposit;
    }

    pub fn withdraw(&mut self,withdraw:u64)->Result<u64, String>{
      if self.amount < withdraw {
        Err(String::from("Insuffcient balance!"))
      } else{
            self.amount = self.amount - withdraw;
            Ok(self.amount)
      } 
    }
    pub fn display(&self){
        println!("Amount:{}",self.amount);
     }
}


pub struct Account {
   pub name:String,
   pub amount:u64,
}

impl Account {
    pub fn new(name: &str, amount: u64) -> Account {
        Account {
            name: String::from(name),
            amount,
        }
    }

    pub fn deposit(&mut self,deposit:u64){ //&self -> for referencing
        self.amount = self.amount + deposit;
    }

    pub fn withdraw(&mut self,withdraw:u64){
        if self.amount<withdraw{
            println!("insifficient balance!!");
        }
        else {
        self.amount = self.amount - withdraw;
        }
    }
    pub fn display(&self){
        println!("Amount:{}",self.amount);
     }
}


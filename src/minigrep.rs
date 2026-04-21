
pub struct Config<'a> {
    pub filename :&'a String,
    pub query:&'a String,
    pub content :&'a String
}

impl Config<'_>  {
      pub fn search(&mut self) -> Vec<& str> {
        //removing & as when the function ends ...content wipes....and vector is still referncing it
        let mut results = Vec::new();
        for line in self.content.lines() {
            if line.contains(&*self.query) {
                results.push(line);
            }
        }
        results
    }
}


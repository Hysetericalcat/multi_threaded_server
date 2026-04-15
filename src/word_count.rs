use std::collections::HashMap;

pub fn word_count(sentence:&String){
    let mut v = vec![];
    let mut indexes = vec![];
    let mut j = 0;
    let mut map = HashMap::new();
    let bytes = sentence.as_bytes();
    let len_ = (&bytes).len();
    let mut count = 0;

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            v.push(&sentence[j..i]);
            j = i;
        }
    };
    v.push(&sentence[j..len_]);
    for (i,&word) in v.iter().enumerate(){
       if indexes.contains(&i){
            continue;
        }
        for (j,&word_) in v.iter().enumerate(){
            if word == word_{
               count=count+1;
               indexes.push(i);
               map.insert(word,count);
            }
        } 
        count = 0
    };   
    println!("{:?}",map) //map implements debug  
}
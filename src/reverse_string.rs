
pub fn reverse_string(sentence:&String){ //so string doesn't goes out of scope
    let mut v = vec![];
    let mut j = 0;
    let mut  count = 0;
    let mut count_ = 0;
    let bytes = sentence.as_bytes();
    let len_ = (&bytes).len();
    println!("{}",len_);

    for (i, &item) in bytes.iter().enumerate(){ //&item -> to maintain the ownership
        if item == b' ' {
         count = count +1;
        }
    };
    for (i, &item) in bytes.iter().enumerate(){ //&item -> to maintain the ownership
        if item == b' ' {
           count_ = count_ +1;
           v.push(&sentence[j..i]);
           j = i;
           if count_==count {
            v.push(&sentence[j..len_])
           }
        }
    };
    v.reverse();
    let mut rev_str = String::from("");
    for word in v.iter(){
        rev_str.push_str(word);
        rev_str.push_str(" ");
    }
    println!( "{}",rev_str);
}
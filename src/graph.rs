use std::rc::Rc;
use std::cell::RefCell;



pub struct Node {
    pub value: i32,
    pub children: Vec<Rc<RefCell<Node>>>
}

pub fn graph(){
    let child = Rc::new(RefCell::new(Node { value: 10, children: Vec::new() }));
    let parent1 = Rc::new(RefCell::new(Node { value: 1, children: Vec::new() }));
    let parent2 = Rc::new(RefCell::new(Node { value: 2, children: Vec::new() }));
    parent1.borrow_mut().children.push(Rc::clone(&child));
    parent2.borrow_mut().children.push(Rc::clone(&child));  // you are not modifying parent1 itself 
    parent1.borrow().children[0].borrow_mut().value = 99;
    println!("{}", parent2.borrow().children[0].borrow().value);
}
    

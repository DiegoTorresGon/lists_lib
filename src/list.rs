use crate::LinkedList;
use crate::DoublyLinkedList;
use crate::List;
use crate::Node;
use crate::DoubleNode;
use crate::Reversible;

use std::fmt;
use std::string::String;
use std::ops::AddAssign;
use std::ptr;
use std::cmp;
use std::cell::RefCell;
use std::rc::Rc;

impl<'a, T : Clone + fmt::Display + std::convert::From<T>> LinkedList<T> {

    pub fn new() -> LinkedList<T> {
        LinkedList {
        head : None,
        tail : ptr::null_mut(),
        n : 0,
        }
    } 

    fn at_as_mut(&mut self, index : usize) -> &mut Box<Node<T>> {
        if index >= self.n {panic!("Invalid index")};

        let mut current = &mut self.head;
        if index == self.n -1 {
            current = unsafe {
                if self.tail.is_null() {
                    panic!("null ptr at fn at_as_mut, index = {}", index);
                }
                &mut (*self.tail) 
                
            }
        } else {
            for _ in 0..(index){
                match current.as_mut() {
                    Some(box_node) => current = &mut box_node.next,
                    None => panic!("Wrong structure at function at.")
                }
            }
        }
            
        match current.as_mut() {
            Some(asked_node) => asked_node,
            None => panic!("Error in llist at function at_as_mut.")
        }
    }

    fn at_as_ref(&self, index : usize) -> &Box<Node<T>> {
        if index >= self.n {panic!("Invalid index")};

        let mut current = &self.head;
        if index == self.n - 1 {
            current = unsafe { 
                if self.tail.is_null() {
                    panic!("null ptr at fn at_as_mut, index = {}", index);
                }
                &(*self.tail) 
            }
        } else {
            for _ in 0..(index){
                match current {
                    Some(box_node) => current = &box_node.next,
                    None => panic!("Wrong structure at function at.")
                }
            }
        }
            
        match current {
            Some(asked_node) => asked_node,
            None => panic!("Error in llist at function at_as_mut.")
        }
    }

    fn at(&mut self, index : usize) -> Box<Node<T>> {
        if index >= self.n {panic!("Invalid index")};
            
        let mut current = &self.head;
        if index == self.n - 1 {
            current = unsafe { 
                if self.tail.is_null() {
                    panic!("null ptr at fn at_as_mut, index = {}", index);
                }
                &(*self.tail)  
            }
        } else {
            for _ in 0..(index){
                if current.is_some() {
                    current = &current.as_ref().unwrap().next;
                }
            }
        }

        let asked_node = current.clone();

        if let Some(asked_value) = asked_node {
            asked_value
        } else {
            panic!("Something is wrong at function at.");
        }
    }
        
}

impl<T : Clone + fmt::Display + std::convert::From<T>> Node<T> {
    fn new(value : T) -> Node<T> { 
        Node {
            value, next : None, 
        }
    }
}

impl<T>  fmt::Display for LinkedList<T> 
    where T : fmt::Display + Clone + std::convert::From<T>
{ 
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("[");
        if self.size() != 0 {
            result.add_assign(self.value_at(0).to_string().as_str());

            for i in 1..(self.size()) {
                result.add_assign(",");
                result.add_assign(self.value_at(i).to_string().as_str());
            };
        }
        result.add_assign("]");
        write!(f, "{}", result.as_str())
    }
}

impl<'a, T> List<T> for LinkedList<T> 
    where T : Clone + fmt::Display + std::convert::From<T>
{

    fn append(&mut self, value : T) -> u8 {
        let new_node = Box::from(Node::new(value));

        if self.n == 0 {
            self.head = Option::from(new_node);
            self.tail = &mut self.head;
            self.n += 1;
        } else {
            unsafe {
                if self.tail.is_null() {panic!("Null ptr at append")};
                (*self.tail).as_mut().unwrap().next = Option::from(new_node);
                self.tail = &mut (*self.tail).as_mut().unwrap().next;
            }
            self.n += 1;
        }
        
        0
    } 

    //the inserted value will be at index.
    fn insert_at(&mut self, value : T, index : usize) -> u8 {
        if index == self.n {
            self.append(value);
            return 0;
        } else if index >= self.n {
            panic!("Index out of bounds in function insert_at.");
        }

        let mut new_node = Node::new(value);

        new_node.next = Option::from(self.at(index)); 
         
        if index != 0 { 
            let prev_node = &mut self.at_as_mut(index - 1);
            prev_node.next = Option::from(Box::from(new_node));
        } else {
            self.head = Option::from(Box::from(new_node));
        }

        self.n += 1;
            
        if self.n == 1 {self.tail = &mut self.head}
        else {
        self.tail = &mut self.at_as_mut(self.n - 2).next;
        }

        0
    }
    
    fn remove_at(&mut self, index : usize) -> u8 {
        if self.n == 0 { return 1; }
        if index >= self.n {
            return 2;
        }

        let next_node : Option<Box<Node<T>>>;

        if index != self.n - 1 {
            next_node = Option::from(self.at(index + 1));
        } else {
            next_node = None;                
        }

        if index == 0 {
            self.head = next_node; 
        } else {
            let mut prev_node = self.at_as_mut(index - 1);
            prev_node.next = next_node; 
        }

        self.n -= 1;

        if self.n == 1 {self.tail = &mut self.head}
        else if self.n == 0 {self.tail = ptr::null_mut()}
        else {
        self.tail = &mut self.at_as_mut(self.n - 2).next;
        }

        0
    }

    fn value_at(&self, index: usize) -> &T {
        return &self.at_as_ref(index).value;
    }

    fn size(&self) -> usize {
        self.n
    }
}

impl<T> Reversible for LinkedList<T> 
    where T : Clone + fmt::Display + std::convert::From<T> 
{

    fn reverse(&mut self) -> &mut Self {
        if self.n <= 1 {return self;}
        if self.n == 2 {
            let mut new_head = self.at(1); 
            new_head.next = Option::from(self.at(0));
            self.head = Option::from(new_head);

            self.tail = &mut self.at_as_mut(0).next;
            return self;
        }

        //List is at least of length 3.
        let mut after = self.at(2);
        let mut middle = self.at(1);
        let mut before = self.at(0);
        
        for _ in 1..(self.n - 2) {
            middle.next = Option::from(before);
            before = middle;
            middle = after;
            after = match middle.next {
                Some(next) => next,
                None => panic!("Incorrect at reverse funcction.")
            } 
        }

        middle.next = Option::from(before);
        after.next = Option::from(middle);
        self.head = Option::from(after);
    
        self.tail = &mut self.at_as_mut(self.n - 2).next;

        self
    }
}

impl<T : Clone + fmt::Display + std::convert::From<T>> DoubleNode<T> {

    fn new(value : T) -> DoubleNode<T> { 
        DoubleNode {
            value, next : None, prev : None,
        }
    }
}

impl<T : Clone + fmt::Display + std::convert::From<T>> DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            head : None, n : 0,
        }
    }

    pub fn at(&mut self, index : usize) -> Rc<RefCell<DoubleNode<T>>> {
        if index >= self.n {panic!("Invalid index")};
            
        let mut current = &mut self.head;
        if index >= self.n / 2 {
            for _ in (index)..(self.n) {
                current = unsafe {&mut (*current.as_mut().unwrap().as_ptr()).prev}; 
            }
        } else {
            for _ in 0..(index){
                current = unsafe {&mut (*current.as_mut().unwrap().as_ptr()).next};
            }
        }

        current.as_ref().unwrap().clone()
    }

    pub fn at_as_ref(&self, index : usize) -> &Rc<RefCell<DoubleNode<T>>> {
        if index >= self.n {panic!("Invalid index")};
            
        let mut current = &self.head;
        if index >= self.n / 2 {
            for _ in (index)..(self.n) {
                current = unsafe {&mut (*current.as_ref().unwrap().as_ptr()).prev}; 
            }
        } else {
            for _ in 0..(index){
                current = unsafe {&mut (*current.as_ref().unwrap().as_ptr()).next};
            }
        }

        &current.as_ref().unwrap()
    }
}

impl<'a, T> List<T> for DoublyLinkedList<T> 
    where T : Clone + fmt::Display + std::convert::From<T>
{

    fn append(&mut self, value : T) -> u8 {
        let mut new_node = DoubleNode::new(value); 
             
        if self.n == 0 {
            let node = Rc::from(RefCell::from(new_node)); 
            node.borrow_mut().prev = Option::from(node.clone());
            node.borrow_mut().next = Option::from(node.clone());
            self.head = Option::from(node);
            self.n += 1;
        } else {
            new_node.prev = Option::from(self.at(self.n - 1));
            new_node.next = Option::from(self.at(0));
            let cell_node = Rc::from(RefCell::from(new_node));
            self.at(&self.n-1).borrow_mut().next = Option::from(cell_node.clone());
            self.n = self.n + 1;
        
            self.head.as_mut().unwrap().borrow_mut().prev = Option::from(cell_node);
        }
        0
    } 

    //the inserted value will be at index.
    fn insert_at(&mut self, value : T, index : usize) -> u8 {
        if index == self.n {
            self.append(value);
            return 0;
        } else if index >= self.n {
            panic!("Index out of bounds in function insert_at.");
        }

        let new_node = DoubleNode::new(value);

        let (mut prev_node, mut next_node, inserting_node) : (Rc<RefCell<DoubleNode<T>>>, 
            Rc<RefCell<DoubleNode<T>>>, Rc<RefCell<DoubleNode<T>>>); 

        //Just to initialize values.
        prev_node = self.at(0);
        next_node = self.at(0);
        
        inserting_node = Rc::from(RefCell::from(new_node));

        let prev_index;
        let next_index;

        if index == 0 {prev_index = self.n - 1; next_index = 0;}
        else {prev_index = index - 1; next_index = index; }

        let max_index = cmp::max(prev_index, next_index);

        let mut current = &mut self.head;
        for i in 0..(max_index + 1) {
            if i == prev_index { prev_node = current.as_mut().unwrap().clone(); } 
            if i == next_index { next_node = current.as_mut().unwrap().clone(); } 

            if i != max_index { 
                current = unsafe {&mut (*current.as_mut().unwrap().as_ptr()).next};
            }
        }

        inserting_node.borrow_mut().prev = Option::from(prev_node.clone());
        prev_node.borrow_mut().next = Option::from(inserting_node.clone());

        inserting_node.borrow_mut().next = Option::from(next_node.clone());
        next_node.borrow_mut().prev = Option::from(inserting_node.clone());
        
        if index == 0 {self.head = Option::from(inserting_node);}
        self.n += 1;

        0
    }
    
    fn remove_at(&mut self, index : usize) -> u8 {
        if self.n == 0 { return 1; }
        if index >= self.n {
            return 2;
        }

        if self.n == 1 {
            self.head = None;
            self.n = 0;
            return 0;
        }

        let to_be_deleted = self.at(index); 
        
        to_be_deleted.borrow().prev.as_ref().unwrap().borrow_mut().next = 
            Option::from(to_be_deleted.borrow().next.as_ref().unwrap().clone());

        to_be_deleted.borrow().next.as_ref().unwrap().borrow_mut().prev = 
            Option::from(to_be_deleted.borrow().prev.as_ref().unwrap().clone());

        if index == 0 { 
            self.head = Option::from(to_be_deleted.borrow().next.as_ref().unwrap().clone());
        }
        self.n -= 1;
        0
    }

    fn value_at(&self, index: usize) -> &T {
        return unsafe {&(*self.at_as_ref(index).as_ptr()).value}
    }

    fn size(&self) -> usize {
        self.n
    }
}

impl<T>  fmt::Display for DoublyLinkedList<T> 
    where T : fmt::Display + Clone + std::convert::From<T>
{ 
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("[");
        if self.size() != 0 {
            result.add_assign(self.value_at(0).to_string().as_str());

            for i in 1..(self.size()) {
                result.add_assign(",");
                result.add_assign(self.value_at(i).to_string().as_str());
            };
        }
        result.add_assign("]");
        write!(f, "{}", result.as_str())
    }
}

impl<T> Reversible for DoublyLinkedList<T> 
    where T : Clone + fmt::Display + std::convert::From<T> 
{

    fn reverse(&mut self) -> &mut Self {
        if self.n <= 1 {return self;}

        let mut current_node = self.head.as_ref().unwrap().clone();

        for i in 0..(self.n) {
            let prev_node = current_node.borrow().prev.as_ref().unwrap().clone();
            let next_node = current_node.borrow().next.as_ref().unwrap().clone();

            current_node.borrow_mut().prev = Option::from(next_node.clone());
            current_node.borrow_mut().next = Option::from(prev_node);
            
            if i != self.n - 1 { current_node = next_node; }
        }
        
        self.head = Option::from(current_node.clone());

        self
    }
}
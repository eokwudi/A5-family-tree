use std::io;
#[derive(Debug)]
#[derive(PartialEq)]
struct Node {
    value: String,
    mother: String,
    father: String, 
    madre: String, 
    padre: String,
    target: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
     fn add(&mut self, name: &String, relation1: &String, relation2: &String, con: &String ) {

        if &self.value == name {
            println!("Name already exists"); 
            return;
        }
        else if &self.value == con {
          if !self.left.is_none() && relation1 == "mother"{
            println!("Relationship already exists");
            return;
          }
          else if !self.right.is_none() && relation2 == "father"{
            println!("Relationship already exists");
            return;
          }
        }
        
        match self.left {
            Some(ref mut n) => { 
               n.add(name, relation1, relation2, con);
            },
            None => {
                if &self.value == con && relation1 == "mother" {
                        self.mother = relation1.to_string();
                        self.madre = name.to_string();
                        let new_node = Node {value: name.to_string(), mother: "".to_string(), father: "".to_string(), madre: "".to_string(), padre: "".to_string(), target: con.to_string() , left: None, right: None};
                        self.left = Some(Box::new(new_node));
                }
            },
        }
      
         
        match self.right {
            Some(ref mut n) => {
                n.add(name, relation1, relation2, con)
            },
            None => {
                if &self.value == con && relation2 == "father" {
                        self.father = relation2.to_string();
                        self.padre = name.to_string();
                        let new_node = Node {value: name.to_string(), mother: "".to_string(), father: "".to_string(), madre: "".to_string(), padre: "".to_string(), target: con.to_string(), left: None, right: None};
                        self.right = Some(Box::new(new_node));
                    }
            },
        }
     }

    fn search(&mut self, mac: &String) -> bool {
      if &self.value == mac {
        return true;
      }
        fn lookup<'a> (point: &mut Option<Box<Node>>, value: &String) -> bool {
        match point {
            Some(ref mut n) =>{ 
                if &n.value == value{
                    true
                }
                else {
                    n.search(value)
                    }
                }, 
            None => false,
        }
        }
        
        if lookup(&mut self.left, mac) == true || lookup(&mut self.right, mac) == true {
            true }
        else {
            return false;
        }
    }
    
    fn delete(&mut self, mac: &String) {
    if &self.madre == mac {
        self.left = None; 
        self.mother = "".to_string(); 
        self.madre = "".to_string();
        return;
    }
    else if &self.padre == mac {
        self.right = None;
        self.father = "".to_string();
        self.padre = "".to_string();
        return;
    }
    else {
        match self.left {
            Some(ref mut n) => {
                n.delete(mac);
            },
            None => (),
        }
        
        match self.right {
            Some(ref mut n) => {
                n.delete(mac);
            },
            None => (),
      }
      }
      }
      
 fn print_pre_order (&mut self, space:&mut String) {
        let rock = space.to_owned();
        let paper = self.value.to_owned();
        let combined = rock + &paper;
        println!("{}", combined); 
        space.push_str("	");
        let mut slash = space.clone();
        match self.left {
            Some(ref mut n) => n.print_pre_order(space), 
            None => (),
        }
        
        match self.right {
            Some(ref mut n) => n.print_pre_order(&mut slash), 
            None => (),
        }
    }
}


fn main() {
 let mut root = String::new();
 println!("Please enter your name");
 
 io::stdin().read_line(&mut root).expect("Failed to read line");
 let clean = root.trim();
 let bug = clean.clone();
 let mut head = Node {value: bug.to_string(), mother: "".to_string(), father: "".to_string(), madre: "".to_string(), padre: "".to_string(),  target: "".to_string(), left: None, right: None};
 loop {
    let mut points = String::new();
    println!("Next action please"); 
    io::stdin().read_line(&mut points).expect("Failed to read line");
    let mut fresh = Vec::new();

    for i in points.trim().split_whitespace() {
        fresh.push(String::from(i));
    }
    
    if points.starts_with(" ") {
        println!("Invalid command");
    } 
    else if fresh[0] == "add".to_string() {

     if head.search(&fresh[3]) == false {
        println!("Name not found");
      }
      else if head.search(&fresh[1]) == true {
         println!("Name already exists"); 
       }
      else if fresh[2] == "father" {
      head.add(&fresh[1], &"".to_string(), &fresh[2], &fresh[3]);
      }
      else if fresh[2] == "mother" {
        head.add(&fresh[1], &fresh[2], &"".to_string(), &fresh[3]);
      }
      else if fresh[2] != "mother" && fresh[3] == bug || fresh[2] != "father" && fresh[3] == bug {
        println!("Invalid relationship");
      }
      else {
        println!("Invalid relationship");
      }
    }
    else if fresh[0] == "delete".to_string() {
      if fresh[1] == bug {
        println!("Deletion failed");
      }
      else if head.search(&fresh[1]) == false {
        println!("Name not found");

      }
      else {
        head.delete(&fresh[1]);
        println!("Delete completed");

      }
    }
    else if fresh[0] == "print".to_string() {
       head.print_pre_order(&mut "".to_string());
    }
    else if fresh[0] == "quit".to_string() { 
      println!("Good Bye");
      break; 
    }
    else {
      println!("Invalid command");
    }
 }
}

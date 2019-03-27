use std::io;
fn main() {

  //enter the child
  println!("Please enter your name");
  let mut firstPerson = String::new();
  io::stdin().read_line(&mut firstPerson)
    .expect("Couldn't read line");
firstPerson = firstPerson.trim().to_string(); //get rid of newLine


let mut root = Person{
  Name: firstPerson, mother: None, father: None};

  loop{
    println!("Next action please");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .expect("Couldn't read line");

    let mut inputSplit = input.split_whitespace();
    let firstWord = inputSplit.next();

    //QUIT
    if firstWord == Some("quit") {
      println!("Good Bye");
      break;
    }

    //ADD
    else if firstWord == Some("add") {
       if let Some(parent) = inputSplit.next(){
        if let Some(relation) = inputSplit.next(){
          if let Some(child) = inputSplit.next(){
            if let Some(invalid) = inputSplit.next(){
              println!("Invalid command");
            }
            else{Person::add(&mut root, parent, relation, child);}
          }
          else{println!("Invalid command")}
        }
        else{println!("Invalid command")}
      }
      else{println!("Invalid command")}
    }

  //PRINT
   else if firstWord == Some("print") {
      Person::print(&root, 0);
    }

  //DELETE
    else if firstWord == Some("delete"){
      if let Some(exCom) = inputSplit.next(){
        if let Some(invalid) = inputSplit.next(){
          println!("Invalid command")
        }
        else{Person::delete(&mut root, exCom);}
      }
      else{println!("Invalid command")}
    }

    else{println!("Invalid command")}
  }
}

struct Person {
  Name: String,
  mother: Option<Box<Person>>,
  father: Option<Box<Person>>,
}

impl Person{
  fn new(newName: &str)-> Person{
    Person{Name: newName.to_string(), mother:None, father:None}
  }

  fn changeMother(&mut self, parent: &str){
    if let Some(ref _myMother) = self.mother{
      println!("Relationship already exists");
    }
    else {self.mother = Some(Box::new(Person::new(parent)))}
  }

 fn changeFather(&mut self, parent: &str){
    if let Some(ref _myFather) = self.father{
      println!("Relationship already exists");
    } 
    else {self.father = Some(Box::new(Person::new(parent)))}
  }

  fn print(&self, tabs: u32){
 
    if tabs == 1 {print!("\t");}
    if tabs == 2 {print!("\t\t");}
    
    println!("{}", self.Name);
    if let Some(ref myMother) = self.mother{
      Person::print(myMother, tabs+1)
    }
    if let Some(ref myFather) = self.father{
      Person::print(myFather, tabs+1)
    }
  }

  fn contains(&self, searchName: &str) -> bool{
    if let Some(ref myMother) = self.mother{
      if Person::contains(myMother, searchName){
        return true;
      }
    }
    if let Some(ref myFather) = self.father{
      if Person::contains(myFather, searchName){
        return true;
      }
    }
    if self.Name == searchName{return true;}
    else{return false;}
  }

  fn add(&mut self, parent: &str, relation: &str, child: &str){
    if Person::contains(self, parent) {println!("Name already exists");}

    else if Person::contains(self, child){
      if child == self.Name {
        if relation == "mother"{Person::changeMother(self, parent);}
      else if relation == "father" {
        Person::changeFather(self, parent);}
        else {println!("Invalid relationship")}
      }
      if let Some(ref mut myMother) = self.mother{if child == myMother.Name{
        Person::add(myMother, parent, relation, child);
      }
      }
      if let Some(ref mut myFather) = self.father{if child == myFather.Name{
        Person::add(myFather, parent, relation, child);
      }
    }
    }
    else{println!("Name not found");}
  }

  fn delete(&mut self, exCom: &str){
    if Person::contains(self, exCom){
      if exCom == self.Name {println!("Deletion failed");}
      else {

        //figure out if we need to delete father
        let mut deleteFather = false;
        if let Some(ref mut myFather) = self.father{
        if exCom == myFather.Name{
          deleteFather = true;}
        else if Person::contains(myFather, exCom){Person::delete(myFather, exCom);}
      }
      if deleteFather == true {
        self.father = None;
      println!("Delete completed");}

      //figure out if we need to delte mother
        let mut deleteMother = false;
        if let Some(ref mut myMother) = self.mother{
        if exCom == myMother.Name{deleteMother = true;}
        else if Person::contains(myMother, exCom){Person::delete(myMother, exCom);}
      }
      if deleteMother == true {
        self.mother = None;
        println!("Delete completed");}
    }
    }
    else{println!("Name not found");}
  }
  
}

// Conditionals - Used to check the condition of something and act on the result

pub fn run() {
    let age: u8 = 22;
    let check_id: bool = true;
    let knows_person_of_age = true;
    let maior_idade = age >= 21;
    // If/Else
    if maior_idade && check_id || knows_person_of_age {
      println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
      println!("Bartender: Sorry, you have to leave");
    } else {
      println!("Bartender: I'll need to see your ID");
    }
    
    // Shorthand If
    let is_of_age = if maior_idade { true } else { false };
    println!("Is Of Age: {}", is_of_age)
  }
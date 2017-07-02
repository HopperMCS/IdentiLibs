struct Gender {
    name: String,
    found: bool,
}

impl Gender {
    fn gender_name(&self) {
        println!("{}", self.name);
    }
}

trait Details {
    fn is_gender(&self);
}

impl Details for Gender {
    fn is_gender(&self) {
        if self.found == true {
          println!("Either male or female");
        } else {
          println!("Gender not found");
        }
    }
}

fn main() {
    let agender = Gender { name: String::from("Agender"), found: false };
    agender.gender_name();
    agender.is_gender();
}

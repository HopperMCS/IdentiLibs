enum Gender {
  Agender,
}

struct Details {
  name: String,
  description: String,
  sentence: String,
}

fn gender_name(name: Gender) -> String {
  match name {
    Gender::Agender => String::from("Agender"),
  }
}

fn gender_desc(description: Gender) -> String {
  match description {
    Gender::Agender => String::from("A satirical parody here"),
  }
}

fn gender_sentence(sentence: Gender) -> String {
  match sentence {
    Gender::Agender => String::from("A name and parody of this type will be up for grabs here soon, maybe?"),
  }
}

fn gender_struct() {
    let agender = Details {
                    name: gender_name(Gender::Agender),
                    description: gender_desc(Gender::Agender),
                    sentence: gender_sentence(Gender::Agender),
                  };

    println!("Name: {}\nDescription: {}\nSentence: {}", 
             agender.name, 
             agender.description, 
             agender.sentence
            );
}

fn main() {    
    gender_struct();
}

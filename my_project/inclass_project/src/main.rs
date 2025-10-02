//new struct
struct Student {
    name: String,
    major: String,
}

impl Student {
    // constructor
    fn new(name: &str, major: &str) -> Self {
        Student {
            name: name.to_string(),
            major: major.to_string(),
        }
    }

    // Setter
    fn set_major(&mut self, new_major: &str) {
        self.major = new_major.to_string();
    }

    // Getter
    fn get_major(&self) -> &str {
        &self.major
    }
}

fn main() {
    // create a new student
    let mut s1 = Student::new("alex", "Computer Engineering");

    println!("Student Name: {}", s1.name);
    println!("Student Major: {}", s1.get_major());

    // update major
    s1.set_major("Mathematics");
    println!("Updated Major: {}", s1.get_major());
}

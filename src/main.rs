#[allow(unused_variables, dead_code)]
use chrono::NaiveDateTime;


#[derive(Debug)]
struct Person {
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    dob: NaiveDateTime,
}
//implmenting the Person struct
impl Person {
    /// Creates a new [`Person`].
    fn new(
        first_name: String,
        middle_name: Option<String>,
        last_name: String,
        dob: NaiveDateTime,
    ) -> Person {
        Person {
            first_name,
            middle_name: match middle_name {
                Some(middle_name) => Some(middle_name),
                None => None,
            },
            last_name,
            dob,
        }
    }
    /// .
    fn update(
        &mut self,
        first_name: Option<String>,
        middle_name: Option<String>,
        last_name: Option<String>,
        dob: Option<NaiveDateTime>,
    ) {
        if let Some(first_name) = first_name {
            self.first_name = first_name;
        }
        if let Some(middle_name) = middle_name {
            self.middle_name = Some(middle_name);
        }
        if let Some(last_name) = last_name {
            self.last_name = last_name;
        }
        if let Some(dob) = dob {
            self.dob = dob;
        }
    }

    fn full_name(&self) -> String {
        match &self.middle_name {
            Some(middle_name) => format!("{} {} {}", self.first_name, middle_name, self.last_name),
            None => format!("{} {}", self.first_name, self.last_name),
        }
    }
    fn age(&self) -> i64 {
        let now = chrono::Local::now().naive_local();
        now.signed_duration_since(self.dob).num_days() / 365
    }
}
#[derive(Debug)]
struct Pepole {
    people: Vec<Person>,
}
impl Pepole {
    fn new() -> Pepole {
        Pepole { people: Vec::new() }
    }
    fn add(&mut self, person: Person) {
        self.people.push(person);
    }
    fn remove(&mut self, index: usize) {
        self.people.remove(index);
    }
    fn update(&mut self, index: usize, person: Person) {
        self.people[index] = person;
    }
    fn get(&self, index: usize) -> &Person {
        &self.people[index]
    }
    fn get_mut(&mut self, index: usize) -> &mut Person {
        &mut self.people[index]
    }
    fn len(&self) -> usize {
        self.people.len()
    }
}
    


fn main() {
    let p: Person = Person::new(
        "John".to_string(),
        Some("R".to_string()),
        "Doe".to_string(),
        NaiveDateTime::parse_from_str("1990-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    );
    println!("{:?}", &p);
    println!("Full name: {}", &p.full_name());
    let mut k = Pepole::new();
    k.add(p);
    k.add(Person::new(
        "Jane".to_string(),
        None,
        "Doe".to_string(),
        NaiveDateTime::parse_from_str("1990-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    ));
    k.add(Person::new(
        "John".to_string(),
        Some("R".to_string()),
        "Doe".to_string(),
        NaiveDateTime::parse_from_str("1940-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    ));
    // print list of people
    for (i, p) in k.people.iter().enumerate() {
        println!("{}: {} is {}", i, p.full_name(), p.age());
    }
}

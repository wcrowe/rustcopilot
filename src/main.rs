use chrono::NaiveDateTime;

#[derive(Debug)]
struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    dob: NaiveDateTime,
}
//implmenting the Person struct
#[allow(dead_code)]
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
            middle_name: Some(middle_name).map(|x| x.unwrap_or_default()).unwrap_or_default(),
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
            self.middle_name = middle_name;
        }
        if let Some(last_name) = last_name {
            self.last_name = last_name;
        }
        if let Some(dob) = dob {
            self.dob = dob;
        }
    }

    fn full_name(&self) -> String {
        if *&self.middle_name.trim().is_empty() {
            format!("{} {}", self.first_name, self.last_name)
        } else {
            format!(
                "{} {} {}",
                self.first_name, self.middle_name, self.last_name
            )
        }
    }
    fn age(&self) -> i64 {
        let now = chrono::Local::now().naive_local();
        now.signed_duration_since(self.dob).num_days() / 365
    }
}

#[derive(Debug)]
struct Pepole(Vec<Person>);

#[allow(dead_code)]
impl Pepole {
    fn new() -> Pepole {
        Pepole(Vec::new())
    }
    fn add(&mut self, person: Person) {
        self.0.push(person);
    }
    fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }
    fn update(&mut self, index: usize, person: Person) {
        self.0[index] = person;
    }
    fn get(&self, index: usize) -> &Person {
        &self.0[index]
    }
    fn get_mut(&mut self, index: usize) -> &mut Person {
        &mut self.0[index]
    }
    fn len(&self) -> usize {
        self.0.len()
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
        "Jane".to_string(),
        Some("K".to_string()),
        "Doe".to_string(),
        NaiveDateTime::parse_from_str("1940-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    ));
    // print list of people
    for (i, p) in k.0.iter().enumerate() {
        println!("{}: {} is {}", i, p.full_name(), p.age());
    }
    // update the 2nd person
    k.update(
        1,
        Person::new(
            "Jane".to_string(),
            Some("A".to_string()),
            "Smith".to_string(),
            NaiveDateTime::parse_from_str("1930-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
    );
    // print list of people
    for (i, p) in k.0.iter().enumerate() {
        println!("{}: {} is {}", i, p.full_name(), p.age());
    }
}

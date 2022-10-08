use chrono::NaiveDateTime;

#[allow(dead_code, unused_variables)]
#[derive(Debug)]
struct Person {
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    dob: NaiveDateTime,
}
//implmenting the Person struct
impl Person {
    fn new(first_name: String, middle_name : Option<String>, last_name: String, dob: NaiveDateTime) -> Person {
        Person {
            first_name,
            middle_name : match middle_name {
                Some(middle_name) => Some(middle_name),
                None => None,
            },
            last_name,
            dob,
        }
    }
    fn update(&mut self, first_name: String, middle_name : Option<String> , last_name: String, dob: NaiveDateTime) {
        self.first_name = first_name;
        self.middle_name = match middle_name {
            Some(middle_name) => Some(middle_name),
            None => Some("".to_string()),
        };
        self.last_name = last_name;
        self.dob = dob;
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    fn age(&self) -> i64 {
        let now = chrono::Local::now().naive_local();
        now.signed_duration_since(self.dob).num_days() / 365
    }
}

fn main() {
    let p = Person::new(
        "John".to_string(),
        "Doe".to_string(),
        NaiveDateTime::parse_from_str("1990-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    );
    println!("{:?}", p);
    println!("Full name: {}", p.full_name());
    let mut k = p;
    k.update(
        "Jarret".to_string(),
        "Doeboy".to_string(),
        NaiveDateTime::parse_from_str("2010-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    );
    println!("{:?}", k);
    println!("Full name: {}", k.full_name());
}

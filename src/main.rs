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

struct Pepole {
    people: Vec<Person>,
}


fn main() {
    let p = Person::new(
        "John".to_string(),
        Some("R".to_string()),
        "Doe".to_string(),
        NaiveDateTime::parse_from_str("1990-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    );
    println!("{:?}", p);
    println!("Full name: {}", p.full_name());
    let mut k = p;
    k.update(
        Some("K".to_string()),
        Some("R".to_string()),
        Some("Doe".to_string()),
        Some(NaiveDateTime::parse_from_str("1990-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()),
    );
    println!("{:?}", k);
    println!("Full name: {}", k.full_name());
}

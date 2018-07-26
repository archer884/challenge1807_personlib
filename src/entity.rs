use chrono::NaiveDate;
use data;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity<T> {
    id: u64,

    #[serde(flatten)]
    data: T,
}

impl<T> Entity<T> {
    pub fn new(id: u64, data: T) -> Self {
        Self { id, data }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    name: String,
    email: String,
    company: String,
    birth_date: NaiveDate,
}

impl Distribution<Person> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Person {
        // Unwrap is safe here because these are static and we know them to be populated.
        let name = rng.choose(data::names()).unwrap();
        let surname = rng.choose(data::surnames()).unwrap();
        let &(company, domain) = rng.choose(data::companies()).unwrap();

        Person {
            name: build_full_name(name, surname),
            email: build_email(name, surname, domain),
            company: company.into(),
            birth_date: build_birth_date(rng),
        }
    }
}

fn build_full_name(name: &str, surname: &str) -> String {
    let mut s = String::with_capacity(name.len() + surname.len() + 1);
    s.push_str(name);
    s.push(' ');
    s.push_str(surname);
    s
}

fn build_email(name: &str, surname: &str, domain: &str) -> String {
    let mut s = String::with_capacity(surname.len() + domain.len() + 2);
    s.push(name.as_bytes()[0] as char);
    s.push_str(surname);
    s.push('@');
    s.push_str(domain);

    // D'oh.
    s.to_lowercase()
}

fn build_birth_date<R: Rng + ?Sized>(rng: &mut R) -> NaiveDate {
    let year = rng.gen_range(1977, 1998);
    let month = rng.gen_range(1, 13);
    let day = rng.gen_range(1, get_max_day_for_month(month) + 1);

    NaiveDate::from_ymd(year, month, day)
}

fn get_max_day_for_month(month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => 28,
    }
}

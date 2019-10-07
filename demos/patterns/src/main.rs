use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
}


pub mod Seat {

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FirstClassSeat { }

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EconomyPlusSeat { }

}


#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Seat {
    F(Seat::FirstClassSeat),
    EconomyPlus(EconomyPlusSeat),
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Clone)]
pub enum Crate {
    Registry(RegistryCrate),
    GitHub(GitHubRepo),
    Local(String),
}

impl Seat {
    pub(crate) fn id(&self) -> String {
        match *self {
            Seat::FirstClassSeat(ref details) => format!("reg/{}/{}", details.name, details.version),
            Seat::EconomyPlus(ref repo) => format!("gh/{}/{}", repo.org, repo.name),
        }
    }

    // pub(crate) fn to_rustwide(&self) -> RustwideCrate {
    //     match self {
    //         Self::Registry(krate) => RustwideCrate::crates_io(&krate.name, &krate.version),
    //         Self::GitHub(repo) => {
    //             RustwideCrate::git(&format!("https://github.com/{}/{}", repo.org, repo.name))
    //         }
    //         Self::Local(name) => RustwideCrate::local(&LOCAL_CRATES_DIR.join(name)),
    //     }
    // }
}


pub struct SeatConfig {
    pub reclines: bool,
}


pub struct PlaneConfig {
    pub economy_plus_seats: HashMap<String, SeatConfig>,
    pub first_class_seats: HashMap<String, SeatConfig>,
    pub business_seats: HashMap<String, SeatConfig>,
    pub economy_seats: HashMap<String, SeatConfig>,
}

impl PlaneConfig {

    pub fn seat_config(&self, s: &Seat) {
          match *s {
            Crate::Registry(ref details) => self.crates.get(&details.name),
            Crate::GitHub(ref repo) => self.github_repos.get(&repo.slug()),
            Crate::Local(ref name) => self.local_crates.get(name),
        }
    }

    pub fn reclines(&self, c: &Seat) -> bool {
        self.seat_config.map(|c| c.quiet).unwrap_or(false)
    }
}

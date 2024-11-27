use music_theory_api::notation::{note::{Letter, Solfege}, symbol::Symbol};

fn main() {
    println!("{:?}{}", Letter::from(Solfege::Si), Symbol::Flat);
}

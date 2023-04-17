use fake::locales::{EN};
use fake::Fake;

fn main() {
    use fake::faker::internet::raw::*;
    let email: String = SafeEmail(EN).fake();
    println!("{}", email);
}

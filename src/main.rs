use std::fmt;

use chrono::{NaiveDate};
use inquire::{error::InquireResult, validator::Validation};
use inquire_derive::Selectable;

#[derive(Debug, Copy, Clone, Selectable)]
enum Color {
    Red,
    Green,
    Blue,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() -> InquireResult<()> {
    let _color = Color::select("Choose a color:").prompt()?;
    Ok(())

    // let user1: User = User {
    //     name: prompt_name(),
    //     age: prompt_age(),
    //     email: prompt_email(),
    // }
}

fn prompt_date() -> NaiveDate {
    let prompt = "Select the user's birth date";
    let selected_date = inquire::DateSelect::new(prompt)
        .prompt()
        .expect("Failed to select DOB");

    println!("You selected {0}", selected_date.format("%Y month: %B day"));
    selected_date
}

fn prompt_name() -> String {
    let name_validator = |i: &str| -> Result<Validation, _> {
        let first_char = i.chars().next().unwrap() as u8;
        match first_char {
            65..=90 => {
                return Ok(Validation::Valid)
            }
            _ => {
                return Ok(Validation::Invalid("Make sure it is right characters".into()))
            }
        }
    };
    let msg = "What is your name?";
    let name = inquire::Text::new(&msg)
        .with_validator(name_validator)
        .prompt()
        .expect("Error getting name");

    println!("Name is {0}", name);
    name.to_string()
}

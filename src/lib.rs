use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;

use simple_error::SimpleError;

extern crate simple_error;

pub mod year_2022;
pub type DayReturnType = Result<(String, String), Box<dyn Error>>;

struct Year {
    year: u32,
    days: Vec<Day>,
}

impl Year {
    fn display_days(&self) {
        for (i, day) in self.days.iter().enumerate() {
            println!("Day {}: {}", i + 1, day.name);
        }
    }

    fn get_day(&self, input: &str) -> Result<&Day, String> {
        let input = input.replace(':', "");
        let input = if input.len() > 3 && &input.to_lowercase()[..3] == "day" {
            match input.split(' ').nth(1) {
                Some(value) => value,
                None => {
                    return Err(format!("Invalid Day \"{}\"", input));
                }
            }
        } else {
            &input
        };

        let parsed_input: u32 = match input.parse() {
            Ok(value) => value,
            Err(_) => {
                return Err(format!("Invalid Day \"{}\"", input));
            }
        };

        for (i, day) in self.days.iter().enumerate() {
            if parsed_input == (i + 1) as u32 {
                return Ok(day);
            }
        }

        Err(format!("Couldn't Find Day {}", parsed_input))
    }

    fn create_years() -> Vec<Self> {
        vec![Self {
            year: 2022,
            days: vec![
                Day {
                    name: "Calorie Counting".to_string(),
                    function: crate::year_2022::day_1::execute,
                },
                Day {
                    name: "Rock Paper Scissors".to_string(),
                    function: crate::year_2022::day_2::execute,
                },
                Day {
                    name: "Rucksack Reorganization".to_string(),
                    function: crate::year_2022::day_3::execute,
                },
                Day {
                    name: "Camp Cleanup".to_string(),
                    function: crate::year_2022::day_4::execute,
                },
                Day {
                    name: "Supply Stacks".to_string(),
                    function: crate::year_2022::day_5::execute,
                },
                Day {
                    name: "Tuning Trouble".to_string(),
                    function: crate::year_2022::day_6::execute,
                },
                Day {
                    name: "No Space Left On Device".to_string(),
                    function: crate::year_2022::day_7::execute,
                },
                Day {
                    name: "Treetop Tree House".to_string(),
                    function: crate::year_2022::day_8::execute,
                },
            ],
        }]
    }

    fn get_year<'a>(years: &'a [Year], input: &str) -> Result<&'a Year, String> {
        let parsed_input: u32 = match input.parse() {
            Ok(val) => val,
            Err(_) => {
                return Err(format!("\"{}\" Isn't A Valid Year!", input));
            }
        };

        for (i, year) in years.iter().enumerate() {
            if parsed_input == (i + 1) as u32 || parsed_input == year.year {
                return Ok(year);
            }
        }

        Err(format!("Couldn't Find The Year {}!", input))
    }

    fn display_years(years: &[Year]) {
        for (i, year) in years.iter().enumerate() {
            println!("{}. {}", i + 1, year.year);
        }
    }

    fn get_input(&self, day: &Day) -> Result<String, String> {
        let mut path = String::new();
        let mut day_num = 0;

        for (i, day_iter) in self.days.iter().enumerate() {
            if day == day_iter {
                day_num = i + 1;
                path = format!("./inputs/{}/day_{}.txt", self.year, day_num);
                break;
            }
        }

        if path.is_empty() {
            return Err(format!(
                "Couldn't Find Day \"{}\" In Year {}",
                day.name, self.year
            ));
        };

        match fs::read_to_string(&path) {
            Ok(contents) => Ok(contents),
            Err(e) => Err(format!(
                "Failed To Open Input For Year {} Day {} (Path: \"{}\")\nReason: {}",
                self.year, day_num, path, e,
            )),
        }
    }
}

struct Day {
    name: String,
    function: fn(&str) -> DayReturnType,
}

impl Day {
    fn execute(&self, input: &str) -> DayReturnType {
        (self.function)(input)
    }
}

impl core::cmp::PartialEq for Day {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub fn cls() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

pub fn select_day() -> DayReturnType {
    let years = Year::create_years();

    let selected_year;
    let selected_day;

    loop {
        println!("Year Select:\n");
        Year::display_years(&years);

        let input = get_input("\nPlease Select A Year: ");
        selected_year = match Year::get_year(&years, &input) {
            Ok(year) => year,
            Err(e) => {
                cls();
                eprintln!("{}", e);
                continue;
            }
        };

        break;
    }

    cls();
    println!("Selected Year {}", selected_year.year);

    loop {
        println!("Days Select For {}:\n", selected_year.year);
        selected_year.display_days();

        let input = get_input("\nPlease Select A Day: ");
        selected_day = match selected_year.get_day(&input) {
            Ok(year) => year,
            Err(e) => {
                cls();
                eprintln!("{}", e);
                continue;
            }
        };

        break;
    }

    cls();

    let input = match selected_year.get_input(selected_day) {
        Ok(val) => val,
        Err(e) => {
            return Err(Box::new(SimpleError::new(e)));
        }
    };

    selected_day.execute(&input)
}
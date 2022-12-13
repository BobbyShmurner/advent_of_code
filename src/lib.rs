use std::error::Error;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;

use clearscreen::clear;

pub mod macros;
pub mod year_2022;

pub type BoxedError = Box<dyn Error>;
pub type DayReturnType = Result<(String, String), BoxedError>;

extern crate simple_error;
use macros::*;

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

    fn get_day(&self, input: &str) -> Result<&Day, BoxedError> {
        let input = input.replace(':', "");
        let input = if input.len() > 3 && &input.to_lowercase()[..3] == "day" {
            unwrap_or_return_option!(input.split(' ').nth(1), "Invalid Day \"{}\"", input)
        } else {
            &input
        };

        let parsed_input: u32 = unwrap_or_return!(input.parse(), "Invalid Day \"{}\"", input);

        for (i, day) in self.days.iter().enumerate() {
            if parsed_input == (i + 1) as u32 {
                return Ok(day);
            }
        }

        return_err!("Couldn't Find Day {}", parsed_input);
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
                Day {
                    name: "Rope Bridge".to_string(),
                    function: crate::year_2022::day_9::execute,
                },
                Day {
                    name: "Cathode-Ray Tube".to_string(),
                    function: crate::year_2022::day_10::execute,
                },
                Day {
                    name: "Monkey in the Middle".to_string(),
                    function: crate::year_2022::day_11::execute,
                },
                Day {
                    name: "Hill Climbing Algorithm".to_string(),
                    function: crate::year_2022::day_12::execute,
                },
            ],
        }]
    }

    fn get_year<'a>(years: &'a [Year], input: &str) -> Result<&'a Year, BoxedError> {
        let parsed_input: u32 =
            unwrap_or_return!(input.parse(), "\"{}\" Isn't A Valid Year!", input);

        for (i, year) in years.iter().enumerate() {
            if parsed_input == (i + 1) as u32 || parsed_input == year.year {
                return Ok(year);
            }
        }

        return_err!("Couldn't Find The Year \"{}\"!", input);
    }

    fn display_years(years: &[Year]) {
        for (i, year) in years.iter().enumerate() {
            println!("{}. {}", i + 1, year.year);
        }
    }

    fn get_input(&self, day: &Day) -> Result<String, BoxedError> {
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
            return_err!("Couldn't Find Day \"{}\" In Year {}", day.name, self.year);
        };

        let folder = path.split("/day_").into_iter().next().unwrap();
        unwrap_or_return!(
            fs::create_dir_all(folder),
            error: e,
            "Failed to create folder \"{}\"\nReason: {}",
            path,
            e
        );

        if !std::path::Path::new(&path).exists() {
            unwrap_or_return!(
                fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&path),
                error: e,
                "Failed to create file \"{}\"\nReason: {}",
                path,
                e
            );
        }

        let contents = unwrap_or_return!(
            fs::read_to_string(&path),
            error: e,
            "Failed to open input for Year {}, Day {} (Path: \"{}\")\nReason: {}",
            self.year,
            day_num,
            path,
            e
        );

        if contents.trim().is_empty() {
            return_err!(
                "Failed to load input for Year {}, Day {} (Path: \"{}\")\nReason: File is empty",
                self.year,
                day_num,
                path,
            );
        }

        Ok(contents)
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
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub fn pause(prompt: &str) {
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();

    write!(stdout, "{prompt}").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn select_day() -> DayReturnType {
    let years = Year::create_years();

    let selected_year;
    let selected_day;

    loop {
        println!("Year Select:\n");
        Year::display_years(&years);

        let input = get_input("\nPlease Select A Year: ");
        selected_year = unwrap_or_else!(Year::get_year(&years, &input), error: e, {
            clear().unwrap();
            eprintln!("{e}");
            continue;
        });

        break;
    }

    clear().unwrap();
    println!("Selected Year {}", selected_year.year);

    loop {
        println!("Days Select For {}:\n", selected_year.year);
        selected_year.display_days();

        let input = get_input("\nPlease Select A Day: ");
        selected_day = unwrap_or_else!(selected_year.get_day(&input), error: e, {
            clear().unwrap();
            eprintln!("{e}");
            continue;
        });

        break;
    }

    clear().unwrap();

    let input = unwrap_or_return!(selected_year.get_input(selected_day));
    selected_day.execute(&input)
}

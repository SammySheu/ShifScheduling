use crate::employee::Employee;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

/// Interactive mode to get employee data from user input.
/// Demonstrates: user input, validation, loops, conditionals.
///
/// Returns: (employees, records) tuple
/// - employees: HashMap<String, Employee> - all employee objects
/// - records: usize - number of employee preference records
pub fn user_interactive_input() -> (HashMap<String, Employee>, usize) {
    let mut employees: HashMap<String, Employee> = HashMap::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    println!("\n=== Employee Scheduling System - Interactive Mode ===\n");
    println!("Days: Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday");
    println!("Shifts: Morning, Afternoon, Evening");
    println!();

    let mut records = 0;

    loop {
        print!("Enter employee name (or 'done' to finish): ");
        io::stdout().flush().unwrap();

        let mut name = String::new();
        handle.read_line(&mut name).unwrap();
        let name = name.trim();

        if name.to_lowercase() == "done" {
            break;
        }

        if name.is_empty() {
            println!("Name cannot be empty. Please try again.");
            continue;
        }

        let employee_name = name.to_string();
        
        println!("\nEnter shift preferences for {}:", name);
        println!("For each day, enter preferred shift or press Enter to skip");

        let days = vec![
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ];
        let shifts = vec!["Morning", "Afternoon", "Evening"];

        for day in &days {
            loop {
                print!(
                    "\nEnter your choices for {} (1: Morning, 2: Afternoon, 3: Evening or Enter to skip)\nTwo choices have to be separated by space: ",
                    day
                );
                io::stdout().flush().unwrap();

                let mut shift_input = String::new();
                handle.read_line(&mut shift_input).unwrap();
                let shift_input = shift_input.trim();

                if shift_input.is_empty() {
                    break;
                }

                let shift_input_list: Vec<&str> = shift_input.split_whitespace().collect();
                
                if shift_input_list.len() > 2 {
                    println!("    Invalid shift. Please enter maximum two choices separated by space.");
                    continue;
                }

                let mut shift_input_str: Vec<String> = Vec::new();
                let mut invalid_input = false;

                for shift_choice in &shift_input_list {
                    if let Ok(num) = shift_choice.parse::<usize>() {
                        if num >= 1 && num <= 3 {
                            shift_input_str.push(shifts[num - 1].to_string());
                        } else {
                            println!("    Invalid shift. Please enter maximum two choices separated by space.");
                            invalid_input = true;
                            break;
                        }
                    } else {
                        println!("    Invalid shift. Please enter maximum two choices separated by space.");
                        invalid_input = true;
                        break;
                    }
                }

                if invalid_input {
                    continue;
                }

                // Create or get employee
                let employee = employees
                    .entry(employee_name.clone())
                    .or_insert_with(|| Employee::new(employee_name.clone(), 0));
                
                employee.add_preference(day.to_string(), shift_input_str);
                records += 1;
                break;
            }
        }
    }

    (employees, records)
}

/// Load employee preferences from CSV file.
///
/// Returns: (employees, records) tuple
/// - employees: HashMap<String, Employee> - all employee objects
/// - records: usize - number of employee preference records
pub fn get_data_from_csv(filename: &str) -> (HashMap<String, Employee>, usize) {
    println!("\n=== Loading preference requests from {} ===\n", filename);

    let mut employees: HashMap<String, Employee> = HashMap::new();
    let mut records = 0;

    match csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)
    {
        Ok(mut reader) => {
            for result in reader.records() {
                match result {
                    Ok(row) => {
                        // Skip empty rows
                        if row.is_empty() || row.get(0).unwrap_or("").trim().is_empty() {
                            continue;
                        }

                        // Parse CSV - minimum 3 columns required
                        if row.len() < 3 {
                            continue;
                        }

                        let name = row.get(0).unwrap().trim().to_string();
                        let day = row.get(1).unwrap().trim().to_string();
                        let shift_prefs_raw = row.get(2).unwrap().trim();

                        // Parse shift preferences
                        let mut shift_prefs: Vec<String> = Vec::new();
                        if !shift_prefs_raw.is_empty() {
                            for s in shift_prefs_raw.split(',') {
                                let trimmed = s.trim();
                                if !trimmed.is_empty() {
                                    // Capitalize first letter
                                    let mut chars = trimmed.chars();
                                    let capitalized = match chars.next() {
                                        None => String::new(),
                                        Some(first) => {
                                            first.to_uppercase().collect::<String>()
                                                + &chars.as_str().to_lowercase()
                                        }
                                    };
                                    shift_prefs.push(capitalized);
                                }
                            }
                        }

                        if shift_prefs.is_empty() {
                            continue;
                        }

                        // Create or get employee
                        let employee = employees
                            .entry(name.clone())
                            .or_insert_with(|| Employee::new(name.clone(), 0));
                        
                        employee.add_preference(day, shift_prefs);
                        records += 1;
                    }
                    Err(e) => {
                        println!("  ✗ Error reading CSV row: {}", e);
                    }
                }
            }

            println!("  Loaded {} employee preference records", records);
        }
        Err(e) => {
            println!("  ✗ Error: File '{}' not found or cannot be read: {}", filename, e);
        }
    }

    (employees, records)
}


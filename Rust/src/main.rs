/*!
Employee Scheduling System
This application manages employee schedules for a company operating 7 days a week.
*/

mod employee;
mod program_input;
mod scheduler;

use program_input::{get_data_from_csv, user_interactive_input};
use scheduler::Scheduler;
use std::fs::File;
use std::io::{self, Write};

/// Main function to run the scheduling system.
/// Demonstrates: program flow, function calls, user interaction.
fn main() {
    println!("{}", "=".repeat(80));
    println!("{:^80}", "EMPLOYEE SCHEDULING SYSTEM");
    println!("{}", "=".repeat(80));

    let (employees, records) = loop {
        println!("\nSelect input mode:");
        println!("1. Enter employee data interactively");
        println!("2. Load from CSV file");

        print!("\nEnter choice (1 or 2): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                let (employees, records) = user_interactive_input();
                break (employees, records);
            }
            "2" => {
                print!("Enter CSV filename (default: test_data.csv): ");
                io::stdout().flush().unwrap();

                let mut filename = String::new();
                io::stdin().read_line(&mut filename).unwrap();
                let filename = filename.trim();

                let filename = if filename.is_empty() {
                    "test_data.csv"
                } else {
                    filename
                };

                let (employees, records) = get_data_from_csv(filename);
                if employees.is_empty() {
                    println!("\nNo employees loaded. Exiting.");
                    return;
                }
                println!("Loaded {} employee preference records", records);
                break (employees, records);
            }
            _ => {
                println!("Invalid choice. Please enter 1 or 2.");
            }
        }
    };

    // Create scheduler and add employees
    let mut scheduler = Scheduler::new();

    for (_name, employee) in employees {
        scheduler.add_employee(employee);
    }

    scheduler.create_schedule(records);

    // Display the results
    scheduler.display_schedule();

    // Save schedule to file
    print!("\nWould you like to save the schedule to a file? (yes/no): ");
    io::stdout().flush().unwrap();

    let mut save_choice = String::new();
    match io::stdin().read_line(&mut save_choice) {
        Ok(_) => {
            let save_choice = save_choice.trim().to_lowercase();

            if save_choice == "yes" || save_choice == "y" {
                print!("Enter filename (default: schedule.txt): ");
                io::stdout().flush().unwrap();

                let mut filename = String::new();
                io::stdin().read_line(&mut filename).unwrap();
                let filename = filename.trim();

                let filename = if filename.is_empty() {
                    "schedule.txt"
                } else {
                    filename
                };

                // Save schedule to file
                match save_schedule_to_file(&scheduler, filename) {
                    Ok(_) => println!("\n✓ Schedule saved to {}", filename),
                    Err(e) => println!("\n✗ Error saving file: {}", e),
                }
            }
        }
        Err(_) => {
            // Handle non-interactive mode (e.g., piped input)
            println!("\n(Non-interactive mode - schedule not saved to file)");
        }
    }
}

/// Save the schedule to a file.
fn save_schedule_to_file(scheduler: &Scheduler, filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;

    // Write schedule header
    writeln!(file, "\n{}", "=".repeat(80))?;
    writeln!(file, "{:^80}", "WEEKLY EMPLOYEE SCHEDULE")?;
    writeln!(file, "{}\n", "=".repeat(80))?;

    for day in &Scheduler::DAYS {
        writeln!(file, "\n{}", "─".repeat(80))?;
        writeln!(file, "  {}", day.to_uppercase())?;
        writeln!(file, "{}", "─".repeat(80))?;

        for shift in &Scheduler::SHIFTS {
            let employees = &scheduler.schedule[&day.to_string()][&shift.to_string()];
            let employee_names: Vec<String> =
                employees.iter().map(|emp| emp.name.clone()).collect();
            let employee_names_str = if employee_names.is_empty() {
                "No one assigned".to_string()
            } else {
                employee_names.join(", ")
            };

            let status = if employees.len() >= Scheduler::MIN_EMPLOYEES_PER_SHIFT {
                "✓"
            } else {
                "⚠"
            };

            writeln!(
                file,
                "  {} {:12} ({} staff): {}",
                status,
                shift,
                employees.len(),
                employee_names_str
            )?;
        }
    }

    writeln!(file, "\n{}", "=".repeat(80))?;

    // Write employee summary
    writeln!(file, "\n{:^80}", "EMPLOYEE SUMMARY")?;
    writeln!(file, "{}\n", "=".repeat(80))?;

    let mut sorted_employees = scheduler.employees.clone();
    sorted_employees.sort_by(|a, b| a.name.cmp(&b.name));

    for employee in &sorted_employees {
        let days_working = employee.get_days_working();
        writeln!(file, "  {}:", employee.name)?;
        writeln!(
            file,
            "    Total days working: {}/{}",
            days_working,
            Scheduler::MAX_DAYS_PER_WEEK
        )?;

        if days_working > 0 {
            writeln!(file, "    Schedule:")?;
            for day in &Scheduler::DAYS {
                if let Some(shift) = employee.assigned_shifts.get(*day) {
                    writeln!(file, "      - {}: {}", day, shift)?;
                }
            }
        } else {
            writeln!(file, "    No shifts assigned")?;
        }
        writeln!(file)?;
    }

    writeln!(file, "{}", "=".repeat(80))?;

    Ok(())
}


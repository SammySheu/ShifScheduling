use crate::employee::Employee;
use rand::seq::SliceRandom;
use std::collections::HashMap;

/// Manages the scheduling of employees.
pub struct Scheduler {
    pub employees: Vec<Employee>,
    /// day -> (shift -> list of employees)
    pub schedule: HashMap<String, HashMap<String, Vec<Employee>>>,
    pub need_to_fill_shifts: Vec<(String, String)>,
}

impl Scheduler {
    pub const DAYS: [&'static str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];
    pub const SHIFTS: [&'static str; 3] = ["Morning", "Afternoon", "Evening"];
    pub const MAX_DAYS_PER_WEEK: usize = 5;
    pub const MIN_EMPLOYEES_PER_SHIFT: usize = 2;

    /// Create a new scheduler.
    pub fn new() -> Self {
        let mut scheduler = Scheduler {
            employees: Vec::new(),
            schedule: HashMap::new(),
            need_to_fill_shifts: Vec::new(),
        };
        scheduler.initialize_schedule();
        scheduler
    }

    /// Initialize empty schedule structure.
    fn initialize_schedule(&mut self) {
        for day in &Self::DAYS {
            let mut day_schedule = HashMap::new();
            for shift in &Self::SHIFTS {
                day_schedule.insert(shift.to_string(), Vec::new());
            }
            self.schedule.insert(day.to_string(), day_schedule);
        }
    }

    /// Add an employee to the system.
    pub fn add_employee(&mut self, employee: Employee) {
        self.employees.push(employee);
    }

    /// Create the weekly schedule based on employee preferences.
    /// Implements various control structures: loops, conditionals, branching.
    pub fn create_schedule(&mut self, mut records: usize) {
        println!("\n=== Starting Scheduling Process ===\n");

        // Phase 1: Assign employees to their preferred shifts
        println!("Phase 1: Processing employee preferences...");
        while records > 0 {
            for i in 0..self.employees.len() {
                loop {
                    if self.employees[i].preferences.is_empty() {
                        break;
                    }
                    let success = self.assign_preferred_shifts(i);
                    records = records.saturating_sub(1);
                    if success {
                        break;
                    }
                }
            }
        }

        // Phase 2: Ensure minimum staff per shift
        println!("Phase 2: Ensuring minimum staffing requirements...");
        self.ensure_minimum_staffing();

        println!("\n=== Scheduling Complete ===\n");
    }

    /// Assign employee to their preferred shifts where possible.
    /// Demonstrates: loops, conditionals, branching logic.
    fn assign_preferred_shifts(&mut self, employee_idx: usize) -> bool {
        // Get the first preference (FIFO order)
        let (day, shifts) = {
            let employee = &self.employees[employee_idx];
            if employee.preferences.is_empty() {
                return false;
            }
            // Get the first key-value pair
            let (day, shifts) = employee.preferences.iter().next().unwrap();
            (day.clone(), shifts.clone())
        };

        // Remove this preference
        self.employees[employee_idx].preferences.remove(&day);

        let employee_name = self.employees[employee_idx].name.clone();
        let days_working = self.employees[employee_idx].get_days_working();

        if days_working >= Self::MAX_DAYS_PER_WEEK {
            println!(
                "  - {} has reached max working days ({})",
                employee_name,
                Self::MAX_DAYS_PER_WEEK
            );
            return false;
        }

        if self.employees[employee_idx].is_working_on(&day) {
            return false;
        }

        for pref_shift in &shifts {
            if self.try_assign_shift(employee_idx, &day, pref_shift) {
                println!(
                    "  ✓ Assigned {} to {} {}",
                    employee_name, day, pref_shift
                );
                return true;
            }
        }

        false
    }

    /// Try to assign an employee to a specific shift.
    /// Returns true if successful, false otherwise.
    fn try_assign_shift(&mut self, employee_idx: usize, day: &str, shift: &str) -> bool {
        // Validation checks
        if self.employees[employee_idx].get_days_working() >= Self::MAX_DAYS_PER_WEEK {
            return false;
        }

        if self.employees[employee_idx].is_working_on(day) {
            return false;
        }

        let current_staff = self.schedule[day][shift].len();
        if current_staff >= Self::MIN_EMPLOYEES_PER_SHIFT {
            return false;
        }

        // Assign the shift
        let employee_clone = self.employees[employee_idx].clone();
        self.schedule
            .get_mut(day)
            .unwrap()
            .get_mut(shift)
            .unwrap()
            .push(employee_clone);
        
        self.employees[employee_idx].assign_shift(day.to_string(), shift.to_string());

        true
    }

    /// Ensure each shift has minimum required employees.
    /// Demonstrates: nested loops, conditionals, list operations.
    fn ensure_minimum_staffing(&mut self) {
        let mut rng = rand::thread_rng();

        for day in &Self::DAYS {
            for shift in &Self::SHIFTS {
                let mut current_staff = self.schedule[&day.to_string()][&shift.to_string()].len();

                // Check if we need more employees for this shift
                while current_staff < Self::MIN_EMPLOYEES_PER_SHIFT {
                    // Find available employees
                    let mut available = self.find_available_employees(day);

                    if available.is_empty() {
                        println!(
                            "  ⚠ Warning: Cannot meet minimum staffing for {} {}",
                            day, shift
                        );
                        self.need_to_fill_shifts
                            .push((day.to_string(), shift.to_string()));
                        break;
                    }

                    // Randomly select an employee from available pool
                    available.shuffle(&mut rng);
                    let selected_idx = available[0];

                    // Assign the employee
                    if self.try_assign_shift(selected_idx, day, shift) {
                        let employee_name = self.employees[selected_idx].name.clone();
                        println!(
                            "  + Added {} to {} {} (minimum staffing)",
                            employee_name, day, shift
                        );
                        current_staff += 1;
                    } else {
                        // Remove from available list to avoid infinite loop
                        available.remove(0);

                        if available.is_empty() {
                            break;
                        }
                    }
                }
            }
        }
    }

    /// Find employees available to work on a specific day.
    /// Demonstrates: list comprehension, boolean logic.
    fn find_available_employees(&self, day: &str) -> Vec<usize> {
        let mut available = Vec::new();

        for (idx, employee) in self.employees.iter().enumerate() {
            // Check multiple conditions
            if !employee.is_working_on(day)
                && employee.get_days_working() < Self::MAX_DAYS_PER_WEEK
            {
                available.push(idx);
            }
        }

        available
    }

    /// Display the final schedule in a readable format.
    /// Demonstrates: string formatting, nested loops, output formatting.
    pub fn display_schedule(&self) {
        println!("\n{}", "=".repeat(80));
        println!("{:^80}", "WEEKLY EMPLOYEE SCHEDULE");
        println!("{}\n", "=".repeat(80));

        for day in &Self::DAYS {
            println!("\n{}", "─".repeat(80));
            println!("  {}", day.to_uppercase());
            println!("{}", "─".repeat(80));

            for shift in &Self::SHIFTS {
                let employees = &self.schedule[&day.to_string()][&shift.to_string()];
                let employee_names: Vec<String> =
                    employees.iter().map(|emp| emp.name.clone()).collect();
                let employee_names_str = if employee_names.is_empty() {
                    "No one assigned".to_string()
                } else {
                    employee_names.join(", ")
                };

                // Format output with proper alignment
                let status = if employees.len() >= Self::MIN_EMPLOYEES_PER_SHIFT {
                    "✓"
                } else {
                    "⚠"
                };

                println!(
                    "  {} {:12} ({} staff): {}",
                    status,
                    shift,
                    employees.len(),
                    employee_names_str
                );
            }
        }

        println!("\n{}", "=".repeat(80));
        self.display_employee_summary();
    }

    /// Display summary of each employee's schedule.
    fn display_employee_summary(&self) {
        println!("\n{:^80}", "EMPLOYEE SUMMARY");
        println!("{}\n", "=".repeat(80));

        // Sort employees by name for consistent output
        let mut sorted_employees = self.employees.clone();
        sorted_employees.sort_by(|a, b| a.name.cmp(&b.name));

        for employee in &sorted_employees {
            let days_working = employee.get_days_working();
            println!("  {}:", employee.name);
            println!(
                "    Total days working: {}/{}",
                days_working,
                Self::MAX_DAYS_PER_WEEK
            );

            if days_working > 0 {
                println!("    Schedule:");
                // Sort by day of week
                for day in &Self::DAYS {
                    if let Some(shift) = employee.assigned_shifts.get(*day) {
                        println!("      - {}: {}", day, shift);
                    }
                }
            } else {
                println!("    No shifts assigned");
            }
            println!();
        }

        println!("{}", "=".repeat(80));
    }
}


use std::collections::HashMap;

/// Represents an employee with their shift preferences.
#[derive(Clone)]
pub struct Employee {
    pub name: String,
    /// CSV row order (lower = higher priority)
    #[allow(dead_code)]
    pub request_priority: i32,
    /// day -> shift preference (list of shifts in priority order)
    pub preferences: HashMap<String, Vec<String>>,
    /// day -> assigned shift
    pub assigned_shifts: HashMap<String, String>,
    /// 0 = first choice, 1 = second choice, etc.
    #[allow(dead_code)]
    pub current_choice_level: usize,
    /// Track how many choices have been used
    #[allow(dead_code)]
    pub choices_used: usize,
}

impl Employee {
    /// Create a new employee with the given name and optional request priority.
    pub fn new(name: String, request_priority: i32) -> Self {
        Employee {
            name,
            request_priority,
            preferences: HashMap::new(),
            assigned_shifts: HashMap::new(),
            current_choice_level: 0,
            choices_used: 0,
        }
    }

    /// Add shift preference for a specific day.
    ///
    /// # Arguments
    /// * `day` - Day of the week
    /// * `preferred_shifts` - List of shifts in order of preference
    pub fn add_preference(&mut self, day: String, preferred_shifts: Vec<String>) {
        self.preferences.insert(day, preferred_shifts);
    }

    /// Return the number of days the employee is working.
    pub fn get_days_working(&self) -> usize {
        self.assigned_shifts.len()
    }

    /// Check if employee is already working on a specific day.
    pub fn is_working_on(&self, day: &str) -> bool {
        self.assigned_shifts.contains_key(day)
    }

    /// Assign a shift to the employee for a specific day.
    pub fn assign_shift(&mut self, day: String, shift: String) {
        self.assigned_shifts.insert(day, shift);
    }
}

impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}


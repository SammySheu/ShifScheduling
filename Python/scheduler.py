from typing import List, Dict, Tuple
from employee import Employee
import random


class Scheduler:
    """Manages the scheduling of employees."""

    DAYS = ['Monday', 'Tuesday', 'Wednesday',
            'Thursday', 'Friday', 'Saturday', 'Sunday']
    SHIFTS = ['Morning', 'Afternoon', 'Evening']
    MAX_DAYS_PER_WEEK = 5
    MIN_EMPLOYEES_PER_SHIFT = 2

    def __init__(self):
        self.employees: List[Employee] = []
        self.schedule: Dict[str, Dict[str, List[Employee]]] = {}
        self.need_to_fill_shifts: List[Tuple[str, str]] = []
        self._initialize_schedule()

    def _initialize_schedule(self):
        """Initialize empty schedule structure."""
        for day in self.DAYS:
            self.schedule[day] = {shift: [] for shift in self.SHIFTS}

    def add_employee(self, employee: Employee):
        """Add an employee to the system."""
        self.employees.append(employee)

    def create_schedule(self, records: int):
        """
        Create the weekly schedule based on employee preferences.
        Implements various control structures: loops, conditionals, branching.
        """
        print("\n=== Starting Scheduling Process ===\n")

        # Phase 1: Assign employees to their preferred shifts
        print("Phase 1: Processing employee preferences...")
        while records > 0:
            for employee in self.employees:
                while True:
                    if not employee.preferences:
                        break
                    success = self._assign_preferred_shifts(employee)
                    records -= 1
                    if success:
                        break

        # Phase 2: Ensure minimum staff per shift
        print("Phase 2: Ensuring minimum staffing requirements...")
        self._ensure_minimum_staffing()

        # print("Phase 3: Assigning shifts to employees...")
        # self._ensure_maximum_shifts()

        print("\n=== Scheduling Complete ===\n")

    def _assign_preferred_shifts(self, employee: Employee) -> bool:
        """
        Assign employee to their preferred shifts where possible.
        Demonstrates: loops, conditionals, branching logic.
        """
        day, shifts = employee.preferences.popitem(last=False)
        if employee.get_days_working() >= self.MAX_DAYS_PER_WEEK:
            print(
                f"  - {employee.name} has reached max working days ({self.MAX_DAYS_PER_WEEK})")
            return False
        if employee.is_working_on(day):
            return False
        for pref_shift in shifts:
            if self._try_assign_shift(employee, day, pref_shift):
                print(
                    f"  ✓ Assigned {employee.name} to {day} {pref_shift}")
                return True
        return False

    def _try_assign_shift(self, employee: Employee, day: str, shift: str) -> bool:
        """
        Try to assign an employee to a specific shift.
        Returns True if successful, False otherwise.
        """
        # Validation checks
        if employee.get_days_working() >= self.MAX_DAYS_PER_WEEK:
            return False

        if employee.is_working_on(day):
            return False

        if len(self.schedule[day][shift]) >= self.MIN_EMPLOYEES_PER_SHIFT:
            return False
        # Assign the shift
        self.schedule[day][shift].append(employee)
        employee.assign_shift(day, shift)
        return True

    def _ensure_minimum_staffing(self):
        """
        Ensure each shift has minimum required employees.
        Demonstrates: nested loops, conditionals, list operations.
        """
        for day in self.DAYS:
            for shift in self.SHIFTS:
                current_staff = len(self.schedule[day][shift])

                # Check if we need more employees for this shift
                while current_staff < self.MIN_EMPLOYEES_PER_SHIFT:
                    # Find available employees
                    available = self._find_available_employees(day)

                    if not available:
                        print(
                            f"  ⚠ Warning: Cannot meet minimum staffing for {day} {shift}")
                        self.need_to_fill_shifts.append((day, shift))
                        break

                    # Randomly select an employee from available pool
                    selected = random.choice(available)
                    # selected = available[0]

                    # Assign the employee
                    if self._try_assign_shift(selected, day, shift):
                        print(
                            f"  + Added {selected.name} to {day} {shift} (minimum staffing)")
                        current_staff += 1
                    else:
                        # Remove from available list to avoid infinite loop
                        available.remove(selected)

                        if not available:
                            break

    def _find_available_employees(self, day: str) -> List[Employee]:
        """
        Find employees available to work on a specific day.
        Demonstrates: list comprehension, boolean logic.
        """
        available = []

        for employee in self.employees:
            # Check multiple conditions
            if (not employee.is_working_on(day) and
                    employee.get_days_working() < self.MAX_DAYS_PER_WEEK):
                available.append(employee)

        return available

    def display_schedule(self):
        """
        Display the final schedule in a readable format.
        Demonstrates: string formatting, nested loops, output formatting.
        """
        print("\n" + "="*80)
        print(" " * 25 + "WEEKLY EMPLOYEE SCHEDULE")
        print("="*80 + "\n")

        for day in self.DAYS:
            print(f"\n{'─'*80}")
            print(f"  {day.upper()}")
            print(f"{'─'*80}")

            for shift in self.SHIFTS:
                employees = self.schedule[day][shift]
                employee_names = ", ".join([emp.name for emp in employees])

                # Format output with proper alignment
                if len(employees) >= self.MIN_EMPLOYEES_PER_SHIFT:
                    status = "✓"
                else:
                    status = "⚠"

                print(
                    f"  {status} {shift:12} ({len(employees)} staff): {employee_names if employees else 'No one assigned'}")

        print("\n" + "="*80)
        self._display_employee_summary()

    def _display_employee_summary(self):
        """Display summary of each employee's schedule."""
        print("\n" + " "*25 + "EMPLOYEE SUMMARY")
        print("="*80 + "\n")

        # Sort employees by name for consistent output
        sorted_employees = sorted(self.employees, key=lambda e: e.name)

        for employee in sorted_employees:
            days_working = employee.get_days_working()
            print(f"  {employee.name}:")
            print(
                f"    Total days working: {days_working}/{self.MAX_DAYS_PER_WEEK}")

            if days_working > 0:
                print("    Schedule:")
                # Sort by day of week
                for day in self.DAYS:
                    if day in employee.assigned_shifts:
                        shift = employee.assigned_shifts[day]
                        print(f"      - {day}: {shift}")
            else:
                print("    No shifts assigned")
            print()

        print("="*80)

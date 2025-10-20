from employee import Employee
import csv
from typing import List, Tuple


def user_interactive_input() -> Tuple[List[Employee], int]:
    """
    Interactive mode to get employee data from user input.
    Demonstrates: user input, validation, loops, conditionals.
    """
    employees = {}

    print("\n=== Employee Scheduling System - Interactive Mode ===\n")
    print("Days: Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday")
    print("Shifts: Morning, Afternoon, Evening")
    print()

    while True:
        name = input("Enter employee name (or 'done' to finish): ").strip()

        if name.lower() == 'done':
            break

        if not name:
            print("Name cannot be empty. Please try again.")
            continue

        employee = Employee(name)

        print(f"\nEnter shift preferences for {name}:")
        print("For each day, enter preferred shift or press Enter to skip")

        days = ['Monday', 'Tuesday', 'Wednesday',
                'Thursday', 'Friday', 'Saturday', 'Sunday']
        shifts = ['Morning', 'Afternoon', 'Evening']

        records = 0
        for day in days:
            while True:
                shift_input = input(
                    f"\nEnter your choices for  {day} (1: Morning, 2: Afternoon, 3: Evening or Enter to skip)\nTwo choices have to be separated by space: ").strip()
                if shift_input == '':
                    break
                shift_input_list = shift_input.split(' ')
                if len(shift_input_list) > 2:
                    print(
                        "    Invalid shift. Please enter maximumtwo choices separated by space.")
                    continue
                shift_input_str = []
                invalid_input = False
                for _shift_input in shift_input_list:
                    if _shift_input.isdigit() and int(_shift_input) in [1, 2, 3]:
                        shift_input_str.append(shifts[int(_shift_input) - 1])
                    else:
                        print(
                            "    Invalid shift. Please enter maximum two choices separated by space.")
                        invalid_input = True
                if invalid_input:
                    continue
                    # Create or get employee
                if name not in employees:
                    employees[name] = employee
                employee.add_preference(day, shift_input_str)
                records += 1
                break
    return employees, records


def get_data_from_csv(filename: str = 'test_data.csv') -> Tuple[List[Employee], int]:
    """
    Load employee preferences from CSV file.
    Returns:
        (employees, records) tuple
        - employees: List[Employee] - all employee objects
        - records: int - number of employee preference records
    """
    print(f"\n=== Loading preference requests from {filename} ===\n")

    employees = {}  # name -> employee object
    records = 0

    try:
        with open(filename, 'r', encoding='utf-8') as csvfile:
            csv_reader = csv.reader(csvfile)
            for row in csv_reader:
                # Skip empty rows
                if not row or not row[0].strip():
                    continue

                # Parse CSV
                if len(row) < 3:
                    continue

                name = row[0].strip()
                day = row[1].strip()
                shift_prefs_raw = row[2].strip()

                # Parse shift preferences
                shift_prefs = []
                if shift_prefs_raw:
                    shift_prefs = [s.strip().capitalize()
                                   for s in shift_prefs_raw.split(',')]
                    shift_prefs = [s for s in shift_prefs if s]

                if not shift_prefs:
                    continue

                # Create or get employee
                if name not in employees:
                    employees[name] = Employee(name)
                employees[name].add_preference(day, shift_prefs)
                records += 1

        print(f"  Loaded {records} employee preference records")
        return employees, records

    except FileNotFoundError:
        print(f"  ✗ Error: File '{filename}' not found")
        return [], []
    except (IOError, OSError, csv.Error) as e:
        print(f"  ✗ Error reading CSV: {e}")
        return [], []

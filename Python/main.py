"""
Employee Scheduling System
This application manages employee schedules for a company operating 7 days a week.
"""

from program_input import user_interactive_input, get_data_from_csv
from scheduler import Scheduler


def main():
    """
    Main function to run the scheduling system.
    Demonstrates: program flow, function calls, user interaction.
    """
    print("="*80)
    print(" " * 20 + "EMPLOYEE SCHEDULING SYSTEM")
    print("="*80)

    while True:
        print("\nSelect input mode:")
        print("1. Enter employee data interactively")
        print("2. Load from CSV file")

        choice = input("\nEnter choice (1 or 2): ").strip()

        if choice == '1':
            employees, records = user_interactive_input()
            break
        elif choice == '2':
            filename = input(
                "Enter CSV filename (default: test_data.csv): ").strip()
            if not filename:
                filename = "test_data.csv"

            employees, records = get_data_from_csv(filename)
            if len(employees) == 0:
                print("\nNo employees loaded. Exiting.")
                return
            print(f"Loaded {records} employee preference records")
            break
        else:
            print("Invalid choice. Please enter 1 or 2.")

    # Create scheduler and add employees
    scheduler = Scheduler()

    for _name in employees:
        scheduler.add_employee(employees[_name])

    scheduler.create_schedule(records)

    # Display the results
    scheduler.display_schedule()

    # Save schedule to file
    try:
        save_choice = input(
            "\nWould you like to save the schedule to a file? (yes/no): ").strip().lower()

        if save_choice in ['yes', 'y']:
            filename = input(
                "Enter filename (default: schedule.txt): ").strip()
            if not filename:
                filename = "schedule.txt"

            # Redirect output to file
            import sys
            original_stdout = sys.stdout

            try:
                with open(filename, 'w', encoding='utf-8') as f:
                    sys.stdout = f
                    scheduler.display_schedule()

                sys.stdout = original_stdout
                print(f"\n✓ Schedule saved to {filename}")
            except (IOError, OSError) as e:
                sys.stdout = original_stdout
                print(f"\n✗ Error saving file: {e}")
    except EOFError:
        # Handle non-interactive mode (e.g., piped input)
        print("\n(Non-interactive mode - schedule not saved to file)")
    except KeyboardInterrupt:
        print("\n\nOperation cancelled by user.")


if __name__ == "__main__":
    main()

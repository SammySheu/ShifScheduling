from typing import List
from collections import OrderedDict


class Employee:
    """Represents an employee with their shift preferences."""

    def __init__(self, name: str):
        self.name = name
        self.preferences = OrderedDict()  # day -> shift preference
        self.assigned_shifts = {}  # day -> assigned shift

    def add_preference(self, day: str, preferred_shifts: List[str]):
        """
        Add shift preference for a specific day.

        Args:
            day: Day of the week
            preferred_shifts: List of shifts in order of preference
        """
        self.preferences[day] = preferred_shifts

    def get_days_working(self) -> int:
        """Return the number of days the employee is working."""
        return len(self.assigned_shifts)

    def is_working_on(self, day: str) -> bool:
        """Check if employee is already working on a specific day."""
        return day in self.assigned_shifts

    def assign_shift(self, day: str, shift: str):
        """Assign a shift to the employee for a specific day."""
        self.assigned_shifts[day] = shift

    def __str__(self):
        return self.name

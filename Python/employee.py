from typing import List
from collections import OrderedDict


class Employee:
    """Represents an employee with their shift preferences."""

    def __init__(self, name: str, request_priority: int = 0):
        self.name = name
        # CSV row order (lower = higher priority)
        self.request_priority = request_priority
        self.preferences = OrderedDict()  # day -> shift preference
        self.priority_preferences = {}  # day -> list of shifts in priority order
        self.assigned_shifts = {}  # day -> assigned shift
        # 0 = first choice, 1 = second choice, etc.
        self.current_choice_level = 0
        self.choices_used = 0  # Track how many choices have been used

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

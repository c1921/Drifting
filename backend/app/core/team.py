# ===========================================
# File: core/team.py
# Description: Defines the Team class - a group of characters traveling and acting together.
# ===========================================

from __future__ import annotations

import math
from typing import List, Optional

from .character import Character


class Team:
    """
    Team is a unit of multiple characters acting together.
    Represents player or AI controlled groups managing members, resources, and travel.
    """

    def __init__(self, name: str, leader: Character, is_player_controlled: bool = True):
        # ===== Basic Properties =====
        self.name = name
        self.leader = leader
        self.is_player_controlled = is_player_controlled

        # ===== Member System =====
        self.members: List[Character] = [leader]

        # ===== Resource System =====
        self.gold: int = 100
        self.food: int = 50
        self.capacity: float = 500.0  # Maximum carry weight
        self.inventory: List[dict] = []  # [{"item": item_obj, "quantity": 5}]

        # ===== Status System =====
        self.morale: float = 100.0
        self.speed: float = 1.0
        self.location: str = "Unknown"
        self.destination: Optional[str] = None
        self.distance_to_destination: float = 0.0
        self.traveling: bool = False

    # -------------------------------------------------------------------------
    # Member Management
    # -------------------------------------------------------------------------

    def add_member(self, character: Character):
        """Add a new member to the team."""
        if character not in self.members:
            self.members.append(character)
            character.affiliation = self
            print(f"[Team] {character.name} joined team {self.name}.")

    def remove_member(self, character: Character):
        """Remove a member from the team."""
        if character in self.members:
            self.members.remove(character)
            character.affiliation = None
            print(f"[Team] {character.name} left team {self.name}.")

    def list_members(self):
        """Print team member summary."""
        print(f"Team {self.name} members:")
        for member in self.members:
            print(f" - {member.name} ({member.role}) HP:{member.health} Morale:{member.morale}")

    # -------------------------------------------------------------------------
    # Resource System
    # -------------------------------------------------------------------------

    def add_gold(self, amount: int):
        self.gold += amount
        print(f"[Team] Gained gold {amount}. Current gold: {self.gold}")

    def spend_gold(self, amount: int):
        if self.gold >= amount:
            self.gold -= amount
            print(f"[Team] Spent gold {amount}. Remaining gold: {self.gold}")
            return True
        print(f"[Team] Not enough gold to spend {amount}.")
        return False

    def add_food(self, amount: int):
        self.food += amount
        print(f"[Team] Gained food {amount}. Current food: {self.food}")

    def consume_food(self, days: int = 1):
        """Consume food based on team size and days passed."""
        consumption = len(self.members) * days
        self.food -= consumption
        print(f"[Team] Consumed food {consumption}. Remaining food: {self.food}")
        if self.food < 0:
            self.morale -= 10
            self.food = 0
            print("[Team] Food shortage! Morale decreased.")

    # -------------------------------------------------------------------------
    # Travel System
    # -------------------------------------------------------------------------

    def start_travel(self, destination: str, distance: float):
        """Begin traveling toward a destination."""
        self.destination = destination
        self.distance_to_destination = distance
        self.traveling = True
        print(f"[Team] Departing for {destination} ({distance} km)")

    def update_travel(self):
        """Update travel progress; call once per in-game day."""
        if not self.traveling:
            return

        daily_speed = self.get_effective_speed()
        self.distance_to_destination -= daily_speed
        self.consume_food(1)

        if self.distance_to_destination <= 0:
            print(f"[Team] Arrived at {self.destination}!")
            self.location = self.destination or self.location
            self.destination = None
            self.traveling = False
            self.distance_to_destination = 0.0
        else:
            remaining = math.ceil(self.distance_to_destination)
            print(f"[Team] Traveling... {remaining} km remaining.")

    def get_effective_speed(self) -> float:
        """Calculate movement speed based on members and inventory."""
        base_speed = 10.0
        avg_endurance = sum(member.attributes["endurance"] for member in self.members) / len(self.members)
        weight_penalty = len(self.inventory) * 0.1
        fatigue_penalty = sum(member.fatigue for member in self.members) / (10 * len(self.members))
        return max(2.0, base_speed + avg_endurance * 0.2 - weight_penalty - fatigue_penalty)

    # -------------------------------------------------------------------------
    # Status Management
    # -------------------------------------------------------------------------

    def rest(self, days: int = 1):
        """Rest the team, improving morale and reducing fatigue."""
        for member in self.members:
            member.rest(days)
        self.morale = min(100, self.morale + 5 * days)
        print(f"[Team] Rested for {days} day(s); morale now {self.morale}.")

    def summary(self) -> dict:
        """Return a serializable snapshot of team status."""
        return {
            "name": self.name,
            "members": [member.name for member in self.members],
            "gold": self.gold,
            "food": self.food,
            "morale": self.morale,
            "location": self.location,
            "traveling": self.traveling,
        }


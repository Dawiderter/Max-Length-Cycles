from random import Random
from typing import List
import random

def generateSequence(n : int) -> List[int]:
    return list(range(n))

def permute(sequence : List[int]) -> List[int]:
    n = len(sequence)
    for id in reversed(range(0, n)):
        random_id = random.randint(0, id)
        temp = sequence[id]
        sequence[id] = sequence[random_id]
        sequence[random_id] = temp
    return sequence

def find_cycles(sequence : List[int]) -> List[List[int]]:
    n = len(sequence)
    already_seen = [False for i in range(n)]
    found_cycles = []
    for id in range(n):
        cycle = []
        while not already_seen[id]:
            cycle.append(id)
            already_seen[id] = True
            id = sequence[id]
        if cycle:
            found_cycles.append(cycle)
    return found_cycles

def number_of_cycles(sequence: List[int]) -> int:
    return len(find_cycles(sequence))

def max_length_cycle(sequence: List[int]) -> int:
    max_c = 0
    for cycle in find_cycles(sequence):
        max_c = max(max_c, len(cycle))
    return max_c





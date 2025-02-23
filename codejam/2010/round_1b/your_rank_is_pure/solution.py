from functools import cache
from typing import Dict, Tuple

class ModInt:
    m = 100003
    def __init__(self, i):
        self.i = i % self.m
    def __add__(self, other):
        return ModInt(self.i + other.i)
    def __mul__(self, other):
        return ModInt(self.i * other.i)

def read() -> int:
    n = int(input())
    return n

@cache
def compute_combination(n: int, r: int):
    if r < 0 or r > n: return ModInt(0)
    if r == 0 or r == n: return ModInt(1)
    return compute_combination(n - 1, r) + compute_combination(n - 1, r - 1)

@cache
def compute_recurrence(n: int, p: int):
    if p == 1: return ModInt(1)
    if n <= p: return ModInt(0)
    return sum((compute_recurrence(p, q) * compute_combination(n - p - 1, p - q - 1) for q in range(1, p)), ModInt(0))

def solve(n: int) -> int:
    return sum((compute_recurrence(n, p) for p in range(1, n)), ModInt(0)).i

test_case_count = int(input())
for test_case_index in range(1, 1 + test_case_count):
  print(f"Case #{test_case_index}: {solve(read())}")

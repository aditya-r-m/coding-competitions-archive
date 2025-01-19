from functools import reduce
from operator import sub

def read() -> [int]:
    return list(map(int, input().split()))[1:]

def gcd(a: int, b: int) -> int:
    if not b: return a
    return gcd(b, a % b)

def solve(event_times: [int]) -> int:
    g = reduce(gcd, map(abs, map(sub, event_times[1:], event_times)))
    return -event_times[0] % g

c = int(input())
for i in range(1, 1 + c):
    print(f"Case #{i}: {solve(read())}")

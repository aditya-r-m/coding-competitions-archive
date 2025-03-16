from math import sqrt

def read() -> tuple[int, int, int]:
    l, p, c = map(int, input().split())
    return (l, p, c)

def solve(l: int, p: int, c: int) -> int:
    s = 0
    while p / l > c:
        s, m = s + 1, round(l * sqrt(p / l))
        l, p = (l, m) if m / l > p / m else (m, p)
    return s

test_case_count = int(input())
for test_case_index in range(1, 1 + test_case_count):
    print(f"Case #{test_case_index}: {solve(*read())}")

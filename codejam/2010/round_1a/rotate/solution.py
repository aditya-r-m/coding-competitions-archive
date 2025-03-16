def read() -> tuple[int, int, list[list[str]]]:
    (n, k), grid = map(int, input().split()), []
    for _ in range(n): grid.append(list(input().strip()))
    return (n, k, grid)

def solve(n: int, k: int, grid: list[list[str]]) -> str:
    red_wins, blue_wins = False, False
    for row in grid:
        j = 0
        for i in range(-1, -1 - n, -1):
            if row[i] != '.': continue
            j = min(i, j) - 1
            while j >= -n and row[j] == '.': j -= 1
            if j >= -n: row[i], row[j] = row[j], row[i]
    bounded = lambda i1, j1: 0 <= i1 < n and 0 <= j1 < n
    match = lambda i0, j0, i1, j1: bounded(i1, j1) and grid[i0][j0] == grid[i1][j1]
    for i in range(n):
        for j in range(n):
            for (di, dj) in [(0, 1), (1, 0), (1, 1), (1, -1)]:
                m = 1
                while m < k and match(i, j, i + m * di, j + m * dj): m += 1
                red_wins = red_wins or (m == k and grid[i][j] == 'R')
                blue_wins = blue_wins or (m == k and grid[i][j] == 'B')
    if red_wins and blue_wins: return 'Both'
    elif red_wins: return 'Red'
    elif blue_wins: return 'Blue'
    else: return 'Neither'

test_case_count = int(input())
for test_case_index in range(1, 1 + test_case_count):
    print(f"Case #{test_case_index}: {solve(*read())}")

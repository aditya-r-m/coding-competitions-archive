from collections import defaultdict
from heapq import heappush, heappop

def read() -> [[bool]]:
    grid = []
    m, _ = map(int, input().split())
    for _ in range(m):
        grid.append([])
        for c in input(): grid[-1] += map(int, bin(int(c, 16))[2:].zfill(4))
    return grid

def solve(grid) -> str:
    result_map = defaultdict(lambda: 0)
    m, n = len(grid), len(grid[0])
    chessboard_max_len = [[1] * n for _ in range(m)]
    for i in range(-2, -m-1, -1):
        for j in range(-2, -n-1, -1):
            if grid[i][j] != grid[i + 1][j] and grid[i][j] != grid[i][j + 1]:
                chessboard_max_len[i][j] = min(chessboard_max_len[i + 1][j], chessboard_max_len[i][j + 1])
                if grid[i][j] == grid[i + chessboard_max_len[i][j]][j + chessboard_max_len[i][j]]:
                    chessboard_max_len[i][j] += 1
    q = []
    for i in range(m):
        for j in range(n):
            heappush(q, (-chessboard_max_len[i][j], i, j))
    while q:
        (l, i, j) = (lambda t: (-t[0], t[1], t[2]))(heappop(q))
        if l != chessboard_max_len[i][j]: continue
        result_map[l] += 1
        for oi in range(max(0, i - l), i + l):
            for oj in range(max(0, j - l), j + l):
                if oi >= i and oj >= j: chessboard_max_len[oi][oj] = 0
                elif max(i - oi, j - oj) < chessboard_max_len[oi][oj]:
                    chessboard_max_len[oi][oj] = max(i - oi, j - oj)
                    heappush(q, (-chessboard_max_len[oi][oj], oi, oj))
    result_lst = list(reversed(sorted(list(result_map.items()))))
    result_str = f"{len(result_lst)}\n"
    for (k, v) in result_lst: result_str += f"{k} {v}\n"
    return result_str

test_case_count = int(input())
for test_case_index in range(1, 1 + test_case_count):
    print(f"Case #{test_case_index}: {solve(read())}", end='')

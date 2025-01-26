def read() -> (int, int, [int]):
    len_runs, capacity, _ = map(int, input().split())
    groups = list(map(int, input().split()))
    return (len_runs, capacity, groups)

def solve(len_runs: int, capacity: int, groups: [int]) -> int:
    score, current_group, cache = 0, 0, dict()
    while len_runs:
        current_score, previous_group = 0, current_group
        for _ in range(len(groups)):
            if current_score + groups[current_group] > capacity: break
            current_score += groups[current_group]
            current_group += 1
            current_group %= len(groups)
        if previous_group not in cache:
            cache[previous_group] = (current_group, current_score)
        score += current_score
        len_runs -= 1
        if current_group in cache:
            (next_group, current_score), len_cycle = cache[current_group], 1
            while next_group != current_group:
                len_cycle += 1
                current_score += cache[next_group][1]
                next_group = cache[next_group][0]
            score += (len_runs // len_cycle) * current_score
            len_runs %= len_cycle
    return score

c = int(input())
for i in range(1, 1+c):
    print(f"Case #{i}: {solve(*read())}")


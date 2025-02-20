def read() -> ((int, int), (int, int)):
    [a_min, a_max, b_min, b_max] = map(int, input().split())
    return ((a_min, 1 + a_max), (b_min, 1 + b_max))

def solve(range_pair: ((int, int), (int, int))) -> int:
    winning_positions = 0
    is_thin = lambda height, length: 5 * (height ** 2) < (2 * length - height) ** 2
    for ((winning_step, limiting_step), range_opponent) in [range_pair, range_pair[::-1]]:
        for opponent_step in range(*range_opponent):
            while winning_step < limiting_step and not is_thin(opponent_step, winning_step): winning_step += 1
            winning_positions += limiting_step - winning_step
    return winning_positions

test_case_count = int(input())
for test_case_index in range(1, 1 + test_case_count):
    print(f"Case #{test_case_index}: {solve(read())}")

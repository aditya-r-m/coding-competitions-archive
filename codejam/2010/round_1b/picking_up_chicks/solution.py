def read() -> tuple[int, int, int, list[int], list[int]]:
    count_total, count_required, limit_distance, limit_time = list(map(int, input().split()))
    starting_points = list(map(int, input().split()))
    velocities = list(map(int, input().split()))
    return (count_total, count_required, limit_distance, limit_time, starting_points, velocities)

def solve(
    count_total: int,
    count_required: int,
    limit_distance: int,
    limit_time: int,
    starting_points: list[int],
    velocities: list[int]
) -> str:
    count_crossed, index_current, swaps_performed = 0, count_total - 1, 0
    while count_required > count_crossed and index_current + 1:
        if limit_distance - starting_points[index_current] <= velocities[index_current] * limit_time:
            swaps_performed += count_total - 1 - count_crossed - index_current
            count_crossed += 1
        index_current -= 1
    if count_required > count_crossed: return "IMPOSSIBLE"
    else: return swaps_performed

test_case_count = int(input())
for test_case_index in range(1, 1 + test_case_count):
    print(f"Case #{test_case_index}: {solve(*read())}")

def read() -> [(int, int)]:
    coordinates = []
    n = int(input())
    for _ in range(n):
        x, y = map(int, input().split())
        coordinates.append((x, y))
    return coordinates

def count_inversions_and_sort(permutation: [int]) -> int:
    if len(permutation) < 2: return 0
    left_permutation, right_permutation = permutation[:len(permutation)>>1], permutation[len(permutation)>>1:]
    left_index = right_index = 0
    inversion_count = count_inversions_and_sort(left_permutation) + count_inversions_and_sort(right_permutation)
    while left_index + right_index < len(permutation):
        if left_index < len(left_permutation) and (
                right_index == len(right_permutation)
                or
                left_permutation[left_index] < right_permutation[right_index]
            ):
            permutation[left_index + right_index] = left_permutation[left_index]
            left_index += 1
        else:
            permutation[left_index + right_index] = right_permutation[right_index]
            right_index += 1
            inversion_count += len(left_permutation) - left_index
    return inversion_count

def solve(coordinates: [(int, int)]) -> int:
    return count_inversions_and_sort([y for (_, y) in sorted(coordinates)])

test_case_count = int(input())
for test_case_index in range(1, 1 + test_case_count):
    print(f"Case #{test_case_index}: {solve(read())}")

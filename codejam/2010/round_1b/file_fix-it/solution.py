def read() -> tuple[list[str], list[str]]:
    c, n = list(map(int, input().split()))
    current_paths, required_paths = [], []
    for _ in range(c): current_paths.append(input())
    for _ in range(n): required_paths.append(input())
    return (current_paths, required_paths)

def solve(current_paths: list[str], required_paths: list[str]) -> int:
    count_commands_required, command_required, root = 0, 0, { '': dict() }
    for paths in [current_paths, required_paths]:
        for current_path in paths:
            current_node = root
            for current_directory in current_path.split('/'):
                if current_directory not in current_node:
                    current_node[current_directory] = dict()
                    count_commands_required += command_required
                current_node = current_node[current_directory]
        command_required = 1
    return count_commands_required

test_case_count = int(input())
for test_case_index in range(1, 1 + test_case_count):
    print(f"Case #{test_case_index}: {solve(*read())}")

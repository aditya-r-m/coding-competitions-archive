def read() -> (int, int):
    return map(int, input().split())

def solve(n: int, k: int) -> str:
    if (k + 1) & ((1 << n) - 1): return "OFF"
    else: return "ON"

c = int(input())
for i in range(1, 1+c):
    print(f"Case #{i}: {solve(*read())}")


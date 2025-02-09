from collections import deque
from math import inf

class SlidingWindow:
    def __init__(self, values: [int]):
        self.len = len(values)
        if not self.len: return
        self.deque = deque()
        self.index = 0
        self.deque.append((0, values[0]))
        for value in values[1:]: self.insert(value)

    def insert(self, value: int):
        if not self.len: return
        self.index += 1
        last_entry = self.deque.pop()
        while self.deque and last_entry[1] > value:
            last_entry = self.deque.pop()
        if last_entry[1] < value: self.deque.append(last_entry)
        self.deque.append((self.index, value))
        first_entry = self.deque.popleft()
        if first_entry[0] > self.index - self.len: self.deque.appendleft(first_entry)

    def min(self) -> int:
        if not self.len: return inf
        first_entry = self.deque.popleft()
        self.deque.appendleft(first_entry)
        return first_entry[1]

limit_value = 256

def read() -> (int, int, int, [int]):
    cost_deletion, cost_insertion, window_len, _ = map(int, input().split())
    values = list(map(int, input().split()))
    return cost_deletion, cost_insertion, window_len, values

def solve(cost_deletion: int, cost_insertion: int, window_len: int, values: [int]) -> int:
    cache = [0] * limit_value
    for value in values:
        next_cache = list(map(lambda x: x + cost_deletion, cache))
        sliding_window = SlidingWindow([inf] * window_len + cache[0: min(limit_value, window_len + 1)])
        for n in range(limit_value):
            next_cache[n] = min(next_cache[n], abs(value - n) + sliding_window.min())
            sliding_window.insert(cache[min(limit_value - 1, n + window_len + 1)])
        sliding_window = SlidingWindow([inf] * window_len)
        for n in range(limit_value):
            next_cache[n] = min(next_cache[n], cost_insertion + sliding_window.min())
            sliding_window.insert(next_cache[n])
        sliding_window = SlidingWindow([inf] * window_len)
        for n in reversed(range(limit_value)):
            next_cache[n] = min(next_cache[n], cost_insertion + sliding_window.min())
            sliding_window.insert(next_cache[n])
        cache = next_cache
    return min(cache)

test_case_count = int(input())
for test_case_index in range(1, 1 + test_case_count):
    print(f"Case #{test_case_index}: {solve(*read())}")


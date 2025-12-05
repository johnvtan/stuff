from typing import Tuple


def parse(s: str) -> Tuple[list[Tuple[int, int]], list[int]]:
    ranges, ingredients = s.split("\n\n")
    ingredients = list(map(int, ingredients.split("\n")))

    ranges = map(lambda x: x.split("-"), ranges.split("\n"))
    ranges = list(map(lambda r: (int(r[0]), int(r[1])), ranges))

    return ranges, ingredients


def p1(ranges: list[Tuple[int, int]], ingredients: list[int]) -> int:
    return sum(any(r[0] <= ingr <= r[1] for r in ranges) for ingr in ingredients)


def p2(ranges: list[Tuple[int, int]]) -> int:
    ranges = sorted(ranges, key=lambda x: x[0])
    merged = []
    start, end = ranges[0]

    for next_start, next_end in ranges[1:]:
        if end >= next_end:
            continue

        if next_start <= end <= next_end:
            end = next_end
            continue

        if end < next_start:
            merged.append((start, end))
            start, end = next_start, next_end
            continue

        assert False, "shouldn't happen?"

    merged.append((start, end))

    return sum(map(lambda x: x[1] - x[0] + 1, merged))


with open("input.txt") as f:
    input_string = f.read().strip()

r, i = parse(input_string)
print(p1(r, i))

print(p2(r))

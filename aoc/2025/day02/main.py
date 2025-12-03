from typing import Tuple


def parse(s: str) -> list[Tuple[int, int]]:
    def parse_one(line: str) -> Tuple[int, int]:
        l, r = line.split("-")
        assert int(l) <= int(r)
        return int(l), int(r)

    return list(map(parse_one, s.split(",")))


# part 1
def is_invalid(i: int) -> bool:
    copy = i
    digits = 0
    while copy > 0:
        digits += 1
        copy //= 10

    if digits % 2:
        return False

    upper = i // (10 ** (digits // 2))
    lower = i % (10 ** (digits // 2))
    # print(f'\t{upper} {lower}')
    return upper == lower


# part 2
def has_any_repeats(i: int) -> bool:
    digits = []
    while i > 0:
        digits.append(i % 10)
        i //= 10

    def has_repeats_of_size(digits: list[int], size: int) -> bool:
        expected = digits[0:size]
        for i in range(size, len(digits), size):
            if digits[i : i + size] != expected:
                return False
        return True

    max_repeat_size = len(digits) // 2
    for repeat_size in range(1, max_repeat_size + 1):
        if len(digits) % repeat_size:
            continue

        if has_repeats_of_size(digits, repeat_size):
            return True

    return False


def get_invalid_in_range(lower: int, upper: int) -> list[int]:
    ret = [cand for cand in range(lower, upper + 1) if has_any_repeats(cand)]
    return ret


input_string = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124"

with open("input.txt") as f:
    input_string = f.read().strip()

s = 0
for l, r in parse(input_string):
    s += sum(get_invalid_in_range(l, r))
print(s)

from typing import Tuple
from math import ceil


def parse(s: str) -> list[Tuple[str, int]]:
    def parse_one_line(line: str) -> Tuple[str, int]:
        return (line[0], int(line[1:].strip()))

    return list(map(parse_one_line, s.split("\n")))


def count_zeros(rotations: list[Tuple[str, int]]) -> int:
    dial = 50
    count = 0
    for dir, mag in rotations:
        if dir == "L":
            dial -= mag
            while dial < 0:
                dial += 100
        else:
            dial += mag
            while dial >= 100:
                dial -= 100

        if dial == 0:
            count += 1
    return count


def count_zeros2(rotations: list[Tuple[str, int]]) -> int:
    dial = 50
    count = 0
    for dir, mag in rotations:
        old_dial = dial
        if dir == "L":
            dial -= mag
            while dial < 0:
                dial += 100
            if dial > old_dial and old_dial != 0 and dial != 0:
                count += 1
        else:
            dial += mag
            while dial >= 100:
                dial -= 100
            if dial < old_dial and old_dial != 0 and dial != 0:
                count += 1

        if dial == 0:
            count += 1

        count += (mag // 100) - (old_dial == dial == 0)
    return count


with open("input.txt") as f:
    input_string = f.read().strip()

print(count_zeros2(parse(input_string)))

with open("input.txt") as f:
    lines = f.read().strip().split("\n")

test_lines = """\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............""".split()


def p1(lines: list[str]) -> int:
    beam_cols = set()
    beam_cols.add(lines[0].find("S"))
    nsplit = 0

    for line in lines[1:]:
        to_split = []
        for beam in beam_cols:
            if line[beam] != "^":
                continue

            nsplit += 1
            to_split.append(beam)

        for beam in to_split:
            beam_cols.remove(beam)
            beam_cols.add(beam - 1)
            beam_cols.add(beam + 1)

    return nsplit


def p2(lines: list[str]) -> int:
    num_cols = len(lines[0])
    num_rows = len(lines)
    cache = {}

    def count_timelines_to(row: int, col: int) -> int:
        if col < 0 or col > num_cols:
            return 0

        if row >= num_rows:
            return 1

        if (row, col) not in cache:
            if lines[row][col] == "^":
                cache[(row, col)] = count_timelines_to(
                    row, col - 1
                ) + count_timelines_to(row, col + 1)
            else:
                cache[(row, col)] = count_timelines_to(row + 1, col)

        return cache[(row, col)]

    return count_timelines_to(1, lines[0].find("S"))

print(p1(lines))
print(p2(lines))

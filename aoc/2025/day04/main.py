def parse(s: str) -> list[list[str]]:
    return list(map(list, s.split("\n")))


def num_forklift_accessible(grid: list[list[str]]) -> int:
    rows = len(grid)
    cols = len(grid[0])

    def num_adjacent_paper(row: int, col: int) -> int:
        return sum(
            grid[r][c] == "@" if r != row or c != col else 0
            for r in range(max(0, row - 1), min(rows, row + 2))
            for c in range(max(0, col - 1), min(cols, col + 2))
        )

    return sum(
        num_adjacent_paper(r, c) < 4 if grid[r][c] == "@" else 0
        for r in range(rows)
        for c in range(cols)
    )


def remove_all_paper(grid: list[list[str]]) -> int:
    rows = len(grid)
    cols = len(grid[0])

    def num_adjacent_paper(row: int, col: int) -> int:
        return sum(
            grid[r][c] == "@" if r != row or c != col else 0
            for r in range(max(0, row - 1), min(rows, row + 2))
            for c in range(max(0, col - 1), min(cols, col + 2))
        )

    def remove_paper_once() -> int:
        removed = 0
        for r in range(rows):
            for c in range(cols):
                if grid[r][c] != "@":
                    continue
                if num_adjacent_paper(r, c) < 4:
                    grid[r][c] = "."
                    removed += 1
        return removed

    total_removed = 0
    while True:
        removed = remove_paper_once()
        if removed == 0:
            break
        total_removed += removed

    return total_removed


with open("input.txt") as f:
    input_string = f.read().strip()

# input_string = """..@@.@@@@.
# @@@.@.@.@@
# @@@@@.@.@@
# @.@@@@..@.
# @@.@@@@.@@
# .@@@@@@@.@
# .@.@.@.@@@
# @.@@@.@@@@
# .@@@@@@@@.
# @.@.@@@.@."""

print(num_forklift_accessible(parse(input_string)))
print(remove_all_paper(parse(input_string)))

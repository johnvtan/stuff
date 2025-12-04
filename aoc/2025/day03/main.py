def parse(s: str) -> list[list[int]]:
    def parse_line(line: str) -> list[int]:
        return list(map(int, line))

    return list(map(parse_line, s.split("\n")))


def largest_joltage2(bank: list[int]) -> int:
    max_digit = None
    max_joltage = 0

    for digit in reversed(bank):
        if max_digit is not None:
            max_joltage = max(max_joltage, digit * 10 + max_digit)
            max_digit = max(digit, max_digit)
        else:
            max_joltage = digit
            max_digit = digit

    return max_joltage


def largest_joltage(bank: list[int], digits: int) -> int:
    assert len(bank) > digits
    dp = [[0 for _ in range(digits)] for _ in range(len(bank))]
    for i, b in enumerate(bank):
        dp[i][0] = b

    def recurse(bank_idx: int, digit: int):
        if digit + bank_idx >= len(bank):
            return 0

        if digit == 0:
            return bank[bank_idx]

        if dp[bank_idx][digit]:
            return dp[bank_idx][digit]

        shifted = bank[bank_idx] * (10 ** (digit))
        best = 0

        for i in range(bank_idx + 1, len(bank)):
            candidate = recurse(i, digit - 1)
            if candidate:
                best = max(best, shifted + recurse(i, digit - 1))

        dp[bank_idx][digit] = best
        return best

    return max(recurse(i, digits - 1) for i in range(len(bank)))


with open("input.txt") as f:
    input_string = f.read().strip()

# input_string = """987654321111111
# 811111111111119
# 234234234234278
# 818181911112111"""

# input_string = "818181911112111"

print(sum(map(lambda x: largest_joltage(x, 2), parse(input_string))))
print(sum(map(lambda x: largest_joltage(x, 12), parse(input_string))))

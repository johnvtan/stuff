from operator import mul, add


def parse_line(line: str) -> list[int]:
    return list(map(int, line.split()))


def parse(s: str):
    by_line = s.split("\n")
    ops = []
    curr = ""
    for c in by_line[-1]:
        if curr == "":
            curr = c
        else:
            if c != " ":
                ops.append(curr[:-1])
                curr = c
            else:
                curr += c
    ops.append(curr)

    problems = [[op] for op in ops]
    for line in by_line[:-1]:
        i = 0
        for problem, op in zip(problems, ops):
            expected_size = len(op)
            acc = line[i : i + expected_size + 1]
            problem.append(acc)
            i += expected_size + 1

    return problems


def p1(problems: list) -> int:
    op_map = {
        "+": add,
        "*": mul,
    }

    res = 0
    for prob in problems:
        op = op_map[prob[0].strip()]
        acc = int(prob[1].strip())
        operands = prob[2:]
        for operand in operands:
            acc = op(acc, int(operand.strip()))
        res += acc
    return res


def transpose(s: list[str]) -> list[str]:
    # print(s)
    assert all(len(s[i]) == len(s[0]) for i in range(len(s)))

    maxsize = len(s[0])

    ret = [[] for _ in range(maxsize)]

    for i in reversed(range(maxsize)):
        for in_s in s:
            assert i <= len(in_s)
            if in_s[i] != " ":
                ret[i].append(in_s[i])

    # print(ret)
    return ["".join(list(r)) for r in ret if r]


def p2(problems: list) -> int:
    op_map = {
        "+": add,
        "*": mul,
    }

    ops = [p[0] for p in problems]
    operands = list(map(transpose, [p[1:] for p in problems]))

    res = 0
    for op, operand in zip(ops, operands):
        op = op_map[op.strip()]
        # print(op)
        acc = int(operand[0])
        ops = operand[1:]
        for o in ops:
            acc = op(acc, int(o))
        res += acc
    return res


with open("input.txt") as f:
    input_string = f.read().strip()

# input_string="""123 328  51 64
# 45 64  387 23
#  6 98  215 314
# *   +   *   + """

print(p1(parse(input_string)))
print(p2(parse(input_string)))

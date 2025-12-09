from typing import Tuple
import math

Point = Tuple[float, float, float]


def parse(s: str) -> list[Point]:
    def parse_one(line: str) -> Point:
        x, y, z = line.split(",")
        return float(x), float(y), float(z)

    return list(map(parse_one, s.split("\n")))


def p1(points: list[Point], n: int = 10) -> int:
    dists = [
        (math.dist(points[i], points[j]), points[i], points[j])
        for i in range(len(points))
        for j in range(i + 1, len(points))
    ]
    dists.sort()

    circuits = {p: set([p]) for p in points}

    dists = iter(dists)
    for _ in range(n):
        _, p1, p2 = next(dists)
        if circuits[p1] == circuits[p2]:
            continue

        combined = circuits[p1] | circuits[p2]
        assert p1 in combined and p2 in combined
        for p in combined:
            circuits[p] = combined

    deduped = set()
    for circuit in circuits.values():
        circuit = tuple(sorted(circuit))
        deduped.add(circuit)

    deduped = sorted(deduped, key=len, reverse=True)
    return math.prod(map(len, deduped[:3]))


def p2(points: list[Point]) -> int:
    dists = [
        (math.dist(points[i], points[j]), points[i], points[j])
        for i in range(len(points))
        for j in range(i + 1, len(points))
    ]
    dists.sort()

    circuits = {p: set([p]) for p in points}
    num_circuits = len(circuits)

    dists = iter(dists)
    p1, p2 = None, None
    while num_circuits > 1:
        _, p1, p2 = next(dists)
        if circuits[p1] == circuits[p2]:
            continue

        num_circuits -= 1
        combined = circuits[p1] | circuits[p2]
        assert p1 in combined and p2 in combined
        for p in combined:
            circuits[p] = combined

    assert p1 is not None and p2 is not None
    return p1[0] * p2[0]


with open("input.txt") as f:
    input_string = f.read().strip()

test_input = """\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"""

# print(p1(parse(input_string), 1000))
print(p2(parse(input_string)))

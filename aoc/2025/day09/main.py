from typing import Tuple
from collections import defaultdict
from matplotlib import pyplot as plt
from bisect import bisect_left


def pairwise(iterable):
    # pairwise('ABCDEFG') → AB BC CD DE EF FG

    iterator = iter(iterable)
    a = next(iterator, None)

    for b in iterator:
        yield a, b
        a = b


Point = Tuple[int, int]


class LineSegment:
    def __init__(self, p1: Point, p2: Point):
        assert p1[0] == p2[0] or p1[1] == p2[1]
        self.start = min(p1, p2)
        self.end = max(p1, p2)

    def vertical(self) -> bool:
        return self.start[0] == self.end[0]

    def horizontal(self) -> bool:
        return self.start[1] == self.end[1]


class Rectangle:
    def __init__(self, p1: Point, p2: Point):
        self.top_left: Point = (min(p1[0], p2[0]), min(p1[1], p2[1]))
        self.bottom_right: Point = (max(p1[0], p2[0]), max(p1[1], p2[1]))
        assert self.top().horizontal()
        assert self.bottom().horizontal()
        assert self.left().vertical()
        assert self.right().vertical()

    def intersected_by(self, seg: LineSegment) -> bool:
        # print(f'Rect({self.top_left}, {self.bottom_right}) intersected by {seg.start}, {seg.end}')
        if seg.vertical():
            if (
                seg.start[0] <= self.left().start[0]
                or seg.start[0] >= self.right().start[0]
            ):
                return False
            top_y = self.top().start[1]
            intersects_top = seg.start[1] <= top_y < seg.end[1]

            bottom_y = self.bottom().start[1]
            intersects_bottom = seg.start[1] < bottom_y <= seg.end[1]
            # print(f'\tintersects top {intersects_top} btm {intersects_bottom}')

            return intersects_top or intersects_bottom
        else:
            if (
                seg.start[1] <= self.top().start[1]
                or seg.start[1] >= self.bottom().start[1]
            ):
                return False

            left_x = self.left().start[0]
            intersects_left = seg.start[0] <= left_x < seg.end[0]

            right_x = self.right().start[0]
            intersects_right = seg.start[0] < right_x <= seg.end[0]

            # print(f'\tintersects left {intersects_left} right {intersects_right}')
            return intersects_left or intersects_right

    def top_left_corner(self) -> Point:
        return self.top_left

    def bottom_right_corner(self) -> Point:
        return self.bottom_right

    def top_right_corner(self) -> Point:
        return (self.bottom_right[0], self.top_left[1])

    def bottom_left_corner(self) -> Point:
        return (self.top_left[0], self.bottom_right[1])

    def top(self) -> LineSegment:
        return LineSegment(self.top_left_corner(), self.top_right_corner())

    def bottom(self) -> LineSegment:
        return LineSegment(self.bottom_left_corner(), self.bottom_right_corner())

    def left(self) -> LineSegment:
        return LineSegment(self.top_left_corner(), self.bottom_left_corner())

    def right(self) -> LineSegment:
        return LineSegment(self.top_right_corner(), self.bottom_right_corner())

    def area(self) -> int:
        return area(self.top_left, self.bottom_right)


# assert LineSegment((0, 0), (0, 10)).crosses(LineSegment((-1, 5), (10, 5)))
# assert not LineSegment((0, 0), (0, 10)).crosses(LineSegment((0, 5), (10, 5)))
# assert not LineSegment((10, 0), (0, 0)).crosses(LineSegment((10, -1), (10, 5)))
# assert LineSegment((11, 0), (0, 0)).crosses(LineSegment((10, -1), (10, 5)))


def parse(s: str) -> list[Point]:
    def parse_one(line: str) -> Point:
        x, y = line.split(",")
        return int(x), int(y)

    return list(map(parse_one, s.split("\n")))


def area(p1: Point, p2: Point) -> int:
    return (abs(p1[0] - p2[0]) + 1) * (abs(p1[1] - p2[1]) + 1)


def p1(points: list[Point]) -> int:
    return max(area(p1, p2) for p1 in points for p2 in points if p1 != p2)


def p2(points: list[Point]) -> int:
    """
    Find the two points that are the corners of the largest area rectangles that is
    completely enclosed by the points.

    Thoughts:
    - Can I construct a data structure that can answer: Given an interval on some axis, which
      rectangles overlap on that axis? Ideally in better than n^2 time.
    """

    # TODO: do we need to preprocess these segments?
    # Can I turn this into a "where is the green" function?
    # If I follow the line segments, how do I know which way is inside (i.e green)?
    segments = []
    for p1, p2 in pairwise(points):
        segments.append(LineSegment(p1, p2))
    segments.append(LineSegment(points[-1], points[0]))

    for seg in segments:
        print(f"all: {seg.start} {seg.end}")

    horizontal_segments = sorted(
        filter(lambda s: s.horizontal(), segments), key=lambda s: s.start[1]
    )
    for seg in horizontal_segments:
        print(f"hor: {seg.start} {seg.end}")

    horizontal_starts = [seg.start[0] for seg in horizontal_segments]
    vertical_segments = sorted(
        filter(lambda s: s.vertical(), segments), key=lambda s: s.start[0]
    )
    for seg in vertical_segments:
        print(f"ver: {seg.start} {seg.end}")

    vertical_starts = [seg.start[1] for seg in vertical_segments]

    def crosses_boundary(rect: Rectangle) -> bool:
        left_x = rect.left().start[0]
        right_x = rect.right().start[0]

        print(f"Trying rect {rect.top_left} {rect.bottom_right}")

        horizontal_search_points = [
            (bisect_left(horizontal_starts, left_x), left_x),
            (bisect_left(horizontal_starts, right_x), right_x),
        ]
        print("horizontal")

        for i, cond in horizontal_search_points:
            while i:
                cand = horizontal_segments[i - 1]
                print(f"\t{i - 1} cand start={cand.start} end={cand.end}")
                if cand.end[0] < cond:
                    break
                if rect.intersected_by(cand):
                    return True
                i -= 1

        top_y = rect.top().start[1]
        bottom_y = rect.bottom().start[1]

        vertical_search_points = [
            (bisect_left(vertical_starts, top_y), top_y),
            (bisect_left(vertical_starts, bottom_y), bottom_y),
        ]

        print("vertical")
        for seg in vertical_segments:
            print(f"ver: {seg.start} {seg.end}")

        for i, cond in vertical_search_points:
            while i:
                cand = vertical_segments[i - 1]
                print(f"\t{i - 1}: cand start={cand.start} end={cand.end}")
                if cand.end[1] < cond:
                    break
                if rect.intersected_by(cand):
                    return True
                i -= 1

        return False

    max_area = 0
    points = [(2, 5), (11, 1)]
    for p1 in points:
        for p2 in points[1:]:
            if p1 == p2:
                continue

            # print(f'candidate {p1} {p2}')

            rect = Rectangle(p1, p2)
            if crosses_boundary(rect):
                # print('\tintersects boundary')
                continue

            # print(f'\t new candidate area: {p1} {p2} {rect.area()}')
            max_area = max(max_area, rect.area())

    return max_area


with open("input.txt") as f:
    input_string = f.read().strip()

test_input = """\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"""

# print(p1(parse(input_string)))
print(p2(parse(test_input)))

from __future__ import annotations
from dataclasses import dataclass, field
from itertools import combinations
from copy import deepcopy
import math
import time


@dataclass
class Vec3:
    x: int
    y: int
    z: int

    def abs(self) -> Vec3:
        return Vec3(abs(self.x), abs(self.y), abs(self.z))

    def sum(self) -> int:
        return self.x + self.y + self.z

    def __add__(self: Vec3, other: Vec3) -> Vec3:
        assert isinstance(other, Vec3)
        return Vec3(self.x + other.x, self.y + other.y, self.z + other.z)

    def __iadd__(self: Vec3, other: Vec3) -> Vec3:
        assert isinstance(other, Vec3)
        return self + other


@dataclass
class Moon:
    pos: Vec3
    vel: Vec3
    pos_hist: list[Vec3] = field(default_factory=list)
    vel_hist: list[Vec3] = field(default_factory=list)

    def update_pos(self):
        self.pos += self.vel

    def record_vel(self):
        self.vel_hist.append(deepcopy(self.vel))

    def record_pos(self):
        self.pos_hist.append(deepcopy(self.pos))

    def potential(self) -> int:
        return self.pos.abs().sum()

    def kinetic(self) -> int:
        return self.vel.abs().sum()


def apply_gravity(m1: Moon, m2: Moon):
    if m1.pos.x != m2.pos.x:
        x1, x2 = (1, -1) if m1.pos.x < m2.pos.x else (-1, 1)
        m1.vel.x += x1
        m2.vel.x += x2

    if m1.pos.y != m2.pos.y:
        y1, y2 = (1, -1) if m1.pos.y < m2.pos.y else (-1, 1)
        m1.vel.y += y1
        m2.vel.y += y2

    if m1.pos.z != m2.pos.z:
        z1, z2 = (1, -1) if m1.pos.z < m2.pos.z else (-1, 1)
        m1.vel.z += z1
        m2.vel.z += z2


def parse_one(s: str) -> Moon:
    x, y, z = map(lambda x: int(x[2:].strip()), s[1:-1].split(", "))
    return Moon(Vec3(x, y, z), Vec3(0, 0, 0))


def parse(s: str) -> list[Moon]:
    return list(map(lambda s: parse_one(s), s.split("\n")))


def pairs(iterable):
    return combinations(iterable, 2)


def part1(moons: list[Moon], steps: int):
    for i in range(steps):
        for m1, m2 in pairs(moons):
            apply_gravity(m1, m2)
        for m in moons:
            m.update_pos()

    return sum(map(lambda m: m.potential() * m.kinetic(), moons))


def time_until_repeat(orig_pos: list[int]) -> int:
    # Return the timestep which repeats the initial position
    pos = [p for p in orig_pos]
    vel = [0 for _ in pos]
    t = 0

    while True:
        t += 1
        for i in range(len(pos)):
            acc = len(list(filter(lambda x: x > pos[i], pos))) - len(
                list(filter(lambda x: x < pos[i], pos))
            )
            vel[i] += acc
        for i in range(len(pos)):
            pos[i] += vel[i]

        if all(v == 0 for v in vel) and all(
            pos[i] == orig_pos[i] for i in range(len(pos))
        ):
            return t


def part2(moons: list[Moon]):
    # tbh, i'm not sure if it's safe to assume that the initial state will always be the repeat
    # state or if states are periodic, but this seems to work for my input
    x_repeat = time_until_repeat([m.pos.x for m in moons])
    y_repeat = time_until_repeat([m.pos.y for m in moons])
    z_repeat = time_until_repeat([m.pos.z for m in moons])
    return math.lcm(x_repeat, y_repeat, z_repeat)


input_string = """<x=5, y=13, z=-3>
<x=18, y=-7, z=13>
<x=16, y=3, z=4>
<x=0, y=8, z=8>"""

print(part1(parse(input_string), 1000))
print(part2(parse(input_string)))

from typing import Tuple
from copy import deepcopy
from collections import deque
import math
from scipy.optimize import linprog

from sys import setrecursionlimit

setrecursionlimit(10000)


class MachineDesc:
    def __init__(self, lights: list[bool], wiring: list[list[int]], joltage: list[int]):
        self.lights = lights
        self.wiring = wiring
        self.joltage = joltage

    def print(self):
        print(
            f"Machine: lights {self.lights} wiring {self.wiring} joltage {self.joltage}"
        )

    def min_button_presses_bfs(self) -> int:
        queue = deque()
        queue.append((deepcopy(self.lights), None, 0))
        while queue:
            state, last, path_len = queue.popleft()
            if all(not s for s in state):
                return path_len

            for i in range(len(self.wiring)):
                if last is not None and i == last:
                    continue

                wiring = self.wiring[i]
                new_state = deepcopy(state)
                for w in wiring:
                    new_state[w] = not new_state[w]
                queue.append((new_state, i, path_len + 1))
        assert False

    def min_button_presses_dp(self) -> int:
        IN_PROGRESS = -1
        dp = {}

        target = tuple(False for _ in self.lights)
        dp[target] = 0

        def recurse(state: Tuple[bool, ...]) -> int:
            if state in dp:
                assert dp[state] != IN_PROGRESS
                return dp[state]

            dp[state] = IN_PROGRESS
            min_presses = math.inf

            for w in self.wiring:
                new_state = list(state)
                for i in w:
                    new_state[i] = not new_state[i]

                new_state = tuple(new_state)
                if new_state in dp and dp[new_state] != IN_PROGRESS:
                    min_presses = min(min_presses, 1 + dp[new_state])
                else:
                    min_presses = min(min_presses, 1 + recurse(new_state))

            dp[state] = min_presses
            return min_presses

        print(f"machine: {self.joltage}")
        ret = recurse(tuple(self.lights))
        print(f"machine {self.joltage}: {ret}")
        return ret

    def min_joltage_buttons(self) -> float:
        IN_PROGRESS = -1
        dp = {}
        target = (0 for _ in self.lights)
        dp[target] = 0

        def do_dp(state: Tuple[int, ...]) -> float:
            print(f"trying {state}")
            if state in dp:
                assert dp[state] != IN_PROGRESS
                return dp[state]

            if all(s == 0 for s in state):
                return 0

            if any(s < 0 for s in state):
                return math.inf

            dp[state] = IN_PROGRESS
            min_presses = math.inf

            for w in self.wiring:
                new_state = list(state)
                valid = True
                for i in w:
                    if new_state[i] == 0:
                        valid = False
                        break
                    new_state[i] -= 1

                if not valid:
                    continue

                new_state = tuple(new_state)
                if new_state in dp and dp[new_state] != IN_PROGRESS:
                    min_presses = min(min_presses, 1 + dp[new_state])
                else:
                    min_presses = min(min_presses, 1 + do_dp(new_state))

            dp[state] = min_presses
            return min_presses

        ret = do_dp(tuple(self.joltage))

        print(f"{self.joltage}: {ret}")
        return ret

    def min_joltage_buttons_lp(self) -> float:
        c = [1 for _ in self.wiring]
        A_eq = [[0 for _ in self.wiring] for _ in self.joltage]
        b_eq = self.joltage

        # translate wiring into constraints
        for i, wiring in enumerate(self.wiring):
            for w in wiring:
                A_eq[w][i] += 1

        print(len(c), len(A_eq), len(A_eq[0]))
        res = linprog(c, A_eq=A_eq, b_eq=b_eq, integrality=[1 for _ in self.wiring])
        print(res.message, res.fun)
        return res.fun


def parse(s: str) -> list[MachineDesc]:
    def parse_one(line: str) -> MachineDesc:
        lights_str, rest = line.split(" ", maxsplit=1)

        lights = [c == "#" for c in lights_str[1:-1]]

        wiring_str, joltage_str = rest.rsplit(" ", maxsplit=1)
        joltage = list(map(lambda x: int(x.strip()), joltage_str[1:-1].split(",")))

        wiring = []
        for single in wiring_str.strip().split(" "):
            single = single.strip()
            single_wiring = list(map(lambda x: int(x.strip()), single[1:-1].split(",")))
            wiring.append(single_wiring)

        return MachineDesc(lights, wiring, joltage)

    return list(map(parse_one, s.split("\n")))


def p1(machines: list[MachineDesc]) -> int:
    return sum(m.min_button_presses_dp() for m in machines)


def p2(machines: list[MachineDesc]) -> int:
    return sum(m.min_joltage_buttons_lp() for m in machines)


with open("input.txt") as f:
    input_string = f.read().strip()

test_input = """\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"""

# print(p1(parse(input_string)))
# machines = parse(test_input)
print(p2(parse(input_string)))

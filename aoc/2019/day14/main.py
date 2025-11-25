from typing import Tuple, Iterable
from collections import deque, defaultdict
import math


def stripped(x: Iterable[str]) -> Iterable[str]:
    return map(lambda s: s.strip(), x)


def split(x: Iterable[str], delim: str = " ") -> Iterable[list[str]]:
    return map(lambda s: s.split(delim), x)


def parse_one(line: str) -> Tuple[str, Tuple[int, dict[str, int]]]:
    ingredients, output = stripped(line.split("=>"))
    quantity, key = output.split()
    ingredients_dict = dict(
        map(
            lambda ingr: (ingr[1], int(ingr[0])),
            split(stripped(ingredients.split(","))),
        )
    )
    return (key, (int(quantity), ingredients_dict))


def parse(input: str) -> dict[str, Tuple[int, dict[str, int]]]:
    return dict(map(parse_one, input.split("\n")))


def fuel_to_ore(recipes: dict[str, Tuple[int, dict[str, int]]], quantity: int) -> int:
    ore_count = 0
    residual = defaultdict(int)
    to_visit = deque([(quantity, "FUEL")])

    parents = defaultdict(set)
    for parent, (_, children) in recipes.items():
        for child in children:
            parents[child].add(parent)

    # these are waiting for all their parents to be visited before they can be visited
    waiting = defaultdict(int)
    while to_visit or waiting:
        # print(to_visit, waiting)
        if not to_visit:
            # update to_visit with any waiting children whose parents have all been visited.
            to_visit = deque(
                [
                    (waiting[child], child)
                    for child in parents
                    if len(parents[child]) == 0
                ]
            )
            # remove all the waiting children who are about to be visited
            waiting = defaultdict(
                int, filter(lambda item: len(parents[item[0]]) > 0, waiting.items())
            )
            # remove all the children from the parents dict that we're about to visit
            parents = {
                child: parent_set
                for (child, parent_set) in parents.items()
                if len(parent_set) > 0
            }

        assert to_visit
        # print(to_visit, parents)
        (desired_quantity, output) = to_visit.popleft()
        (recipe_quantity, recipe) = recipes[output]

        # how many recipes do we need to make to get at least the quantity we want?
        num_recipes = desired_quantity // recipe_quantity
        if desired_quantity % recipe_quantity:
            num_recipes += 1

        # save any extra we don't actually require, which can be reused for other ingredients.
        residual[output] = (num_recipes * recipe_quantity) - desired_quantity

        for ingredient, quantity in recipe.items():
            assert output in parents[ingredient]

            required_quantity = num_recipes * quantity
            if ingredient == "ORE":
                ore_count += required_quantity
                continue

            # check to see if we have any extra ingredients lying around from previous recipes
            if residual[ingredient]:
                required_quantity -= min(residual[ingredient], required_quantity)
                residual[ingredient] -= min(residual[ingredient], required_quantity)

            if required_quantity > 0:
                waiting[ingredient] += required_quantity

            parents[ingredient].remove(output)
    return ore_count


def part1(recipes: dict[str, Tuple[int, dict[str, int]]]):
    return fuel_to_ore(recipes, 1)


def part2(recipes: dict[str, Tuple[int, dict[str, int]]]):
    target_ore = 1 * 10**12

    # just binary search with a theoretical max of target_ore.
    # this assumes that 1 ore can't generate more than 1 fuel
    # (i.e., 1 trillion ore can't produce more than 1 trillion fuel)
    #
    # With target_ore = 1 trillion, this loop will run at most
    # 40 times (log2(1trillion))
    min_fuel = 1
    max_fuel = target_ore
    count = 0
    while max_fuel > min_fuel:
        fuel_count = (max_fuel + min_fuel) // 2

        num_ore = fuel_to_ore(recipes, fuel_count)
        if num_ore == target_ore:
            # we've used exactly the number of ore and (assuming fuel_to_ore is optimal) that means
            # we can't do better.
            return fuel_count
        if num_ore > target_ore:
            max_fuel = fuel_count  # I'm off by one if I -1 here? Maybe it's because fuel_count is
            # rounded down during division? idk.
        else:
            count = fuel_count
            min_fuel = fuel_count + 1
    return count


input_string = """171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"""

with open("input.txt") as f:
    input_string = f.read().strip()

print(part1(parse(input_string)))
print(part2(parse(input_string)))

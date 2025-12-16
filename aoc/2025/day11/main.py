from collections import defaultdict


def parse(s: str) -> dict[str, list[str]]:
    graph = defaultdict(list)

    def parse_line(line: str):
        in_node, out_nodes = line.split(": ")

        out_nodes.strip()
        out_nodes = out_nodes.split(" ")
        graph[in_node] = out_nodes

    for line in s.split("\n"):
        parse_line(line)

    return graph


def count_paths(graph: dict[str, list[str]], start: str, target: str) -> int:
    cache = dict()

    def count_paths_from(start: str) -> int:
        if start in cache:
            return cache[start]

        if start == target:
            return 1

        cache[start] = 0
        for next_node in graph[start]:
            cache[start] += count_paths_from(next_node)

        return cache[start]

    return count_paths_from(start)


def p2(graph: dict[str, list[str]]) -> int:
    svr_to_fft = count_paths(graph, "svr", "fft")
    fft_to_dac = count_paths(graph, "fft", "dac")
    dac_to_out = count_paths(graph, "dac", "out")

    svr_to_dac = count_paths(graph, "svr", "dac")
    dac_to_fft = count_paths(graph, "dac", "fft")
    fft_to_out = count_paths(graph, "fft", "out")

    print(svr_to_fft, fft_to_dac, dac_to_out, svr_to_dac, dac_to_fft, fft_to_out)
    return (svr_to_fft * fft_to_dac * dac_to_out) + (
        svr_to_dac * dac_to_fft * fft_to_out
    )


with open("input.txt") as f:
    input_string = f.read().strip()

test_input = """\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"""

graph = parse(input_string)
print(count_paths(graph, "you", "out"))
print(p2(graph))

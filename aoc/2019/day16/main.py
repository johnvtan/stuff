from itertools import cycle


class SingleCycle:
    def __init__(self, phase: int):
        assert phase >= 1
        self.phase = phase

    def __iter__(self):
        for ret in [0, 1, 0, -1]:
            for _ in range(self.phase):
                yield ret


class Pattern:
    def __init__(self, phase: int):
        self.phase = phase

    def __iter__(self):
        pat = cycle(SingleCycle(self.phase))
        # skip the first thing
        next(pat)
        return pat


def fft(signal: list[int], times: int) -> list[int]:
    def one_phase(signal: list[int], phase: int) -> int:
        return abs(sum((a * b) for (a, b) in zip(signal, Pattern(phase)))) % 10

    def fft_once(signal: list[int]) -> list[int]:
        return [one_phase(signal, i) for i in range(1, len(signal) + 1)]

    for i in range(times):
        signal = fft_once(signal)

    return signal


def parse(input_string: str) -> list[int]:
    return [int(c) for c in input_string]


def part2(signal: list[int], repeat: int = 10000) -> int:
    """Part 2 exploits the fact that the signal is repeated, and that the pattern for all digits in
    the second half of the output signal are basically all 0s followed by all 1s.
    """
    final_offset = int("".join(map(str, signal[0:7])))

    total_len = len(signal) * repeat
    halfway = total_len // 2

    # this solution only works if the requested offset is in the 2nd half of the signal
    assert final_offset >= halfway

    # Calculate the cycle found in the 2nd half of the output. We start at the end and keep adding
    # in the previous digit of the input signal until we see a repeat of the remained of the sum
    # and the index we're at in the input signal. Once we have a repeat, we know that the cycle
    # is starting again.
    for _ in range(100):
        seen = set()
        summed = 0
        new_signal = []
        for i, n in cycle(enumerate(reversed(signal))):
            if (summed % 10, i) in seen:
                break
            seen.add((summed % 10, i))
            summed += n
            new_signal.append(summed % 10)
        signal = list(reversed(new_signal))

    # To figure out where in the cycle the answer begins, we have to work backwards from the end.
    # We don't know where the cycle starts, but we do know that the last digit is correct. So
    # we need to figure out |final_idx| by calculating the offset from the end of the signal.
    dist_from_end = total_len - final_offset
    final_idx = len(signal) - (dist_from_end % len(signal))

    ret = [0 for _ in range(8)]
    for i in range(8):
        signal_idx = (final_idx + i) % len(signal)
        ret[i] = signal[signal_idx]

    return int("".join(map(str, ret)))


input_string = "02935109699940807407585447034323"
with open("input.txt") as f:
    input_string = f.read().strip()

print(part2(parse(input_string)))

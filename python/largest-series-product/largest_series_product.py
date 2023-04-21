from functools import reduce
from operator import mul


def largest_product(series, size):
    if size < 0:
        raise ValueError("span must not be negative")
    if len(series) < size:
        raise ValueError("span must be smaller than string length")
    if not series.isdecimal() and len(series) > 0:
        raise ValueError("digits input must only contain digits")

    if size == 0:
        return 1

    numbers = [int(d) for d in series]
    windows = zip(*[numbers[i:] for i in range(size)])
    return max(reduce(mul, w) for w in windows)

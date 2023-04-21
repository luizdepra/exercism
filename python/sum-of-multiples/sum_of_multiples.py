def sum_of_multiples(limit, multiples):
    mult_set = set()
    for v in multiples:
        if v == 0:
            continue

        mults = set(range(v, limit, v))
        mult_set = mult_set.union(mults)

    return sum(mult_set)

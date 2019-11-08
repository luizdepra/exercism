def distance(strand_a, strand_b):
    if len(strand_a) != len(strand_b):
        raise ValueError("Both strands should have the same size")

    return len([a for a, b in zip(strand_a, strand_b) if a != b])

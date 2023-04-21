def commands(binary_str):
    result = []
    for n, d in enumerate(binary_str[::-1]):
        match (n, d):
            case (0, "1"):
                result.append("wink")
            case (1, "1"):
                result.append("double blink")
            case (2, "1"):
                result.append("close your eyes")
            case (3, "1"):
                result.append("jump")
            case (4, "1"):
                result = result[::-1]

    return result

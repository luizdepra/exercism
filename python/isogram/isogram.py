def is_isogram(string):
    letters = set()
    for char in string.lower():
        if not char.isalpha():
            continue

        if char in letters:
            return False

        letters.add(char)

    return True

PRESENTS = [
    "twelve Drummers Drumming",
    "eleven Pipers Piping",
    "ten Lords-a-Leaping",
    "nine Ladies Dancing",
    "eight Maids-a-Milking",
    "seven Swans-a-Swimming",
    "six Geese-a-Laying",
    "five Gold Rings",
    "four Calling Birds",
    "three French Hens",
    "two Turtle Doves",
    "a Partridge in a Pear Tree",
]

DAYS = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
]

def recite_presents(index):
    last = PRESENTS[-1]
    if index == 1:
        presents = f"{last}"
    else:
        filtered = PRESENTS[-index:]
        presents = ", ".join(filtered[:index - 1]) + f", and {last}"

    return f"On the {DAYS[index-1]} day of Christmas my true love gave to me: {presents}."


def recite(start_verse, end_verse):
    result = []
    for i in range(start_verse, end_verse + 1):
        result.append(recite_presents(i))

    return result

def get_rounds(number):
    """Tracking Poker Rounds.

     :param number: int - current round number.
     :return: list - current round and the two that follow.
    """

    return [number, number + 1, number + 2]


def concatenate_rounds(rounds_1, rounds_2):
    """Keeping all Rounds in the Same Place.

    :param rounds_1: list - first rounds played.
    :param rounds_2: list - second set of rounds played.
    :return: list - all rounds played.
    """

    return rounds_1 + rounds_2


def list_contains_round(rounds, number):
    """Finding Prior Rounds.

    :param rounds: list - rounds played.
    :param number: int - round number.
    :return:  bool - was the round played?
    """

    return number in rounds


def card_average(hand):
    """Averaging Card Values.

    :param hand: list - cards in hand.
    :return:  float - average value of the cards in the hand.
    """

    return sum(hand) / len(hand)


def approx_average_is_average(hand):
    """Alternate Averages.

    :param hand: list - cards in hand.
    :return: bool - is approximate average the same as true average?
    """

    average = card_average(hand)
    limits_average = (hand[0] + hand[-1]) / 2
    middle_average = hand[len(hand) // 2]

    return average in (limits_average, middle_average)


def average_even_is_average_odd(hand):
    """More Averaging Techniques.

    :param hand: list - cards in hand.
    :return: bool - are even and odd averages equal?
    """

    even = hand[::2]
    odds = hand[1::2]

    return sum(even) / len(even) == sum(odds) / len(odds)


def maybe_double_last(hand):
    """Bonus Round Rules.

    :param hand: list - cards in hand.
    :return: list - hand with Jacks (if present) value doubled.
    """

    if hand[-1] == 11:
        hand[-1] *= 2

    return hand

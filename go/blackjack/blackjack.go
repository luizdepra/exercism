package blackjack

// ParseCard returns the integer value of a card following blackjack ruleset.
func ParseCard(card string) int {
	var value int;

    switch card {
        case "two":
    		value = 2;
        case "three":
    		value = 3;
        case "four":
    		value = 4;
        case "five":
    		value = 5;
        case "six":
    		value = 6;
        case "seven":
    		value = 7;
        case "eight":
    		value = 8;
        case "nine":
    		value = 9;
        case "ten":
    		value = 10;
        case "jack", "queen", "king":
    		value = 10;
        case "ace":
    		value = 11;
        default:
    		value = 0;
    }

    return value;
}

// FirstTurn returns the decision for the first turn, given two cards of the
// player and one card of the dealer.
func FirstTurn(card1, card2, dealerCard string) string {
	playerScore := ParseCard(card1) + ParseCard(card2)
    dealerScore := ParseCard(dealerCard)

    switch {
        case playerScore > 21:
    		return "P";
        case playerScore > 20 && dealerScore < 10:
    		return "W";
        case playerScore <= 11 || (playerScore < 17 && dealerScore >= 7):
    		return "H";
        default:
    		return "S";
    }
}

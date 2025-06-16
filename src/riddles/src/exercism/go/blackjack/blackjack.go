package blackjack

// ParseCard returns the integer value of a card following blackjack ruleset.
func ParseCard(card string) int {
	switch card {
	case "ace":
		return 11
	case "two":
		return 2
	case "three":
		return 3
	case "four":
		return 4
	case "five":
		return 5
	case "six":
		return 6
	case "seven":
		return 7
	case "eight":
		return 8
	case "nine":
		return 9
	case "ten", "jack", "queen", "king":
		return 10
	default:
		return 0
	}
}

// FirstTurn returns the decision for the first turn, given two cards of the
// player and one card of the dealer.
func FirstTurn(card1, card2, dealerCard string) string {
	sum := ParseCard(card1) + ParseCard(card2)
	card1Num := ParseCard(card1)
	card2Num := ParseCard(card2)
	dealerCardNum := ParseCard(dealerCard)
	switch {
	case card1Num == 11 && card2Num == 11:
		return "P"
	case sum == 21 && (dealerCardNum != 11 && dealerCardNum != 10):
		return "W"
	case sum == 21:
		return "S"
	case sum >= 17 && sum <= 20:
		return "S"
		// The order of the case matters.
	case sum >= 12 && sum <= 16 && dealerCardNum >= 7:
		return "H"
	case sum >= 12 && sum <= 16:
		return "S"
	case sum <= 11:
		return "H"
	default:
		return ""
	}

}

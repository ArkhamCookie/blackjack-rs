use crate::Card;

pub(crate) fn check_blackjack(hand: &Vec<Card>) -> bool {
	let mut score = 0;

	for card in hand {
		score += Card::value(card);
	}

	if score == 21 {
		return true;
	}

	false
}

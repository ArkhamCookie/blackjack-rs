use crate::{Card, Rank};

#[derive(PartialEq)]
pub(crate) enum AceEvents {
	BustNone,
	BustAces,
}

/// Check if hand has blackjack
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

/// Check for aces in a hand
/// (assumes Blackjack and safe has already been handled)
pub(crate) fn check_aces(hand: &Vec<Card>) -> AceEvents {
	let mut score = 0;
	let mut aces: u8 = 0;

	for card in hand {
		if card.rank == Rank::Ace {
			aces += 1;
		}
		score += Card::value(card);
	}

	if score > 21 && aces == 0 {
		return AceEvents::BustNone;
	}

	AceEvents::BustAces
}

/// Handles aces in a hand
/// (assumes Blackjack and BustNone have been handled)
pub(crate) fn handle_aces(hand: &Vec<Card>) -> u8 {
	let mut score = 0;
	let mut aces: u8 = 0;

	for card in hand {
		if card.rank == Rank::Ace {
			aces += 1;
		}
		score += Card::value(card);
	}

	if aces == 0 || score <= 21 {
		return score;
	}

	for _ in 0..aces {
		score -= 10;
		if score <= 21 {
			break;
		}
	}

	score
}

#[cfg(test)]
mod tests {
	use crate::card::{Card, Rank, Suit};
	use crate::blackjack::check_blackjack;

	#[test]
	fn check_blackjack_equal_test() {
		let mut hand = Vec::new();
		hand.push(Card::new(Suit::Spades, Rank::Jack));
		hand.push(Card::new(Suit::Spades, Rank::Six));
		hand.push(Card::new(Suit::Spades, Rank::Five));

		assert_eq!(check_blackjack(&hand), true);
	}

	#[test]
	fn check_blackjack_under_test() {
		let mut hand = Vec::new();
		hand.push(Card::new(Suit::Spades, Rank::Jack));
		hand.push(Card::new(Suit::Spades, Rank::Jack));

		assert_eq!(check_blackjack(&hand), false);
	}

	#[test]
	fn check_blackjack_over_test() {
		let mut hand = Vec::new();
		hand.push(Card::new(Suit::Spades, Rank::Jack));
		hand.push(Card::new(Suit::Spades, Rank::Jack));
		hand.push(Card::new(Suit::Spades, Rank::Jack));

		assert_eq!(check_blackjack(&hand), false);
	}
}

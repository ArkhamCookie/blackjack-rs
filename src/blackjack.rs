use crate::{Card, Rank};

#[derive(PartialEq)]
pub(crate) enum AceEvents {
	Safe,
	BustNone,
	BustAces,
}

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
/// (assumes Blackjack has already been handled)
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
	} else if aces > 0 {
		return AceEvents::BustAces;
	} else {
		return AceEvents::Safe;
	}
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

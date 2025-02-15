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

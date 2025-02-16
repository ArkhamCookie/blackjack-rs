use std::cmp::Ordering;

use crate::{Card, Rank};

#[derive(PartialEq)]
enum AceEvents {
	BustNone,
	BustAces,
}

#[derive(Debug, PartialEq)]
pub(crate) enum GameEvents {
	Safe,
	Blackjack,
	Bust,
}

/// Check if hand has blackjack
pub(crate) fn check_blackjack(hand: &Vec<Card>) -> GameEvents {
	let mut score = 0;

	for card in hand {
		score += Card::value(card);
	}

	if score < 21 {
		return GameEvents::Safe;
	} else if score == 21 {
		return GameEvents::Blackjack;
	}

	match check_aces(hand) {
		AceEvents::BustNone => GameEvents::Bust,
		AceEvents::BustAces => {
			let ace_score = handle_aces(hand);

			match ace_score.cmp(&21) {
				Ordering::Less => GameEvents::Safe,
				Ordering::Equal => GameEvents::Blackjack,
				Ordering::Greater => GameEvents::Bust,
			}
		}
	}
}

/// Check for aces in a hand
/// (assumes Blackjack and safe has already been handled)
fn check_aces(hand: &Vec<Card>) -> AceEvents {
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
fn handle_aces(hand: &Vec<Card>) -> u8 {
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
	use crate::blackjack::{check_blackjack, GameEvents};
	use crate::card::{Card, Rank, Suit};

	#[test]
	fn check_blackjack_equal_test() {
		let mut hand = Vec::new();
		hand.push(Card::new(Suit::Spades, Rank::Ten));
		hand.push(Card::new(Suit::Spades, Rank::Six));
		hand.push(Card::new(Suit::Spades, Rank::Five));

		assert_eq!(check_blackjack(&hand), GameEvents::Blackjack);
	}

	#[test]
	fn check_blackjack_under_test() {
		let mut hand = Vec::new();
		hand.push(Card::new(Suit::Spades, Rank::Ten));
		hand.push(Card::new(Suit::Spades, Rank::Ten));

		assert_eq!(check_blackjack(&hand), GameEvents::Safe);
	}

	#[test]
	fn check_blackjack_over_test() {
		let mut hand = Vec::new();
		hand.push(Card::new(Suit::Spades, Rank::Ten));
		hand.push(Card::new(Suit::Spades, Rank::Ten));
		hand.push(Card::new(Suit::Spades, Rank::Ten));

		assert_eq!(check_blackjack(&hand), GameEvents::Bust);
	}
}

use std::cmp::Ordering;

use crate::blackjack::{check_hand, GameEvents};
use crate::card::Card;

/// Compare two hands' score.
/// Returns:
/// 1 if `hand_one` wins,
/// 2 if `hand_two` wins,
/// 3 if a tie,
pub fn compare_hands(hand_one: &Vec<Card>, hand_two: &Vec<Card>) -> u8 {
	let hand_one_event = check_hand(hand_one);
	let mut hand_one_safe = true;
	let hand_two_event = check_hand(hand_two);
	let mut hand_two_safe = true;

	match hand_one_event {
		GameEvents::Bust => hand_one_safe = false,
		_ => {}
	}
	match hand_two_event {
		GameEvents::Bust => hand_two_safe = false,
		_ => {}
	}

	if hand_one_safe && hand_two_safe {
		let mut hand_one_score: u8 = 0;
		let mut hand_two_score: u8 = 0;

		for card in hand_one {
			hand_one_score += Card::value(card);
		}
		for card in hand_two {
			hand_two_score += Card::value(card);
		}

		match hand_one_score.cmp(&hand_two_score) {
			Ordering::Less => 2,
			Ordering::Equal => 3,
			Ordering::Greater => 1,
		}
	} else if hand_one_safe && !hand_two_safe {
		1
	} else if !hand_one_safe && hand_two_safe {
		2
	} else {
		3
	}
}

#[cfg(test)]
mod tests {
	use crate::card::{Card, Rank, Suit};
	use crate::game::compare_hands;

	#[test]
	fn both_bust() {
		let mut hand_one = Vec::new();
		hand_one.push(Card::new(Suit::Spades, Rank::Ten));
		hand_one.push(Card::new(Suit::Spades, Rank::Ten));
		hand_one.push(Card::new(Suit::Spades, Rank::Ten));

		let mut hand_two = Vec::new();
		hand_two.push(Card::new(Suit::Spades, Rank::Ten));
		hand_two.push(Card::new(Suit::Spades, Rank::Ten));
		hand_two.push(Card::new(Suit::Spades, Rank::Ten));

		assert_eq!(compare_hands(&hand_one, &hand_two), 3);
	}

	#[test]
	fn both_tie() {
		let mut hand_one = Vec::new();
		hand_one.push(Card::new(Suit::Spades, Rank::Ten));
		hand_one.push(Card::new(Suit::Spades, Rank::Ten));

		let mut hand_two = Vec::new();
		hand_two.push(Card::new(Suit::Spades, Rank::Ten));
		hand_two.push(Card::new(Suit::Spades, Rank::Ten));

		assert_eq!(compare_hands(&hand_one, &hand_two), 3);
	}

	#[test]
	fn hand_one_busts() {
		let mut hand_one = Vec::new();
		hand_one.push(Card::new(Suit::Spades, Rank::Ten));
		hand_one.push(Card::new(Suit::Spades, Rank::Ten));
		hand_one.push(Card::new(Suit::Spades, Rank::Ten));

		let mut hand_two = Vec::new();
		hand_two.push(Card::new(Suit::Spades, Rank::Ten));
		hand_two.push(Card::new(Suit::Spades, Rank::Ten));

		assert_eq!(compare_hands(&hand_one, &hand_two), 2);
	}

	#[test]
	fn hand_two_busts() {
		let mut hand_one = Vec::new();
		hand_one.push(Card::new(Suit::Spades, Rank::Ten));
		hand_one.push(Card::new(Suit::Spades, Rank::Ten));

		let mut hand_two = Vec::new();
		hand_two.push(Card::new(Suit::Spades, Rank::Ten));
		hand_two.push(Card::new(Suit::Spades, Rank::Ten));
		hand_two.push(Card::new(Suit::Spades, Rank::Ten));

		assert_eq!(compare_hands(&hand_one, &hand_two), 1);
	}

	#[test]
	fn hand_one_wins() {
		let mut hand_one = Vec::new();
		hand_one.push(Card::new(Suit::Spades, Rank::Ten));
		hand_one.push(Card::new(Suit::Spades, Rank::Ten));

		let mut hand_two = Vec::new();
		hand_two.push(Card::new(Suit::Spades, Rank::Ten));
		hand_two.push(Card::new(Suit::Spades, Rank::Nine));

		assert_eq!(compare_hands(&hand_one, &hand_two), 1);
	}

	#[test]
	fn hand_two_wins() {
		let mut hand_one = Vec::new();
		hand_one.push(Card::new(Suit::Spades, Rank::Ten));
		hand_one.push(Card::new(Suit::Spades, Rank::Nine));

		let mut hand_two = Vec::new();
		hand_two.push(Card::new(Suit::Spades, Rank::Ten));
		hand_two.push(Card::new(Suit::Spades, Rank::Ten));

		assert_eq!(compare_hands(&hand_one, &hand_two), 2);
	}
}

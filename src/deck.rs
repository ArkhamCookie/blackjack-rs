use crate::card::{Card, Rank, Suit};

use rand::seq::SliceRandom;
use rand::thread_rng;

use strum::IntoEnumIterator;

#[derive(Debug)]
pub(crate) struct Deck {
	pub(crate) cards: Vec<Card>,
}

impl Deck {
	pub(crate) fn new(mut deck_count: u8) -> Self {
		let mut cards = Vec::new();

		while deck_count > 0 {
			for suit in Suit::iter() {
				for rank in Rank::iter() {
					let card = Card::new(suit, rank);
					cards.push(card);
				}
			}
			deck_count -= 1;
		}

		Deck { cards }
	}

	pub(crate) fn shuffle(deck_count: u8) -> Self {
		let deck = Deck::new(deck_count);
		let mut cards = deck.cards;
		cards.shuffle(&mut thread_rng());

		Deck { cards }
	}
}

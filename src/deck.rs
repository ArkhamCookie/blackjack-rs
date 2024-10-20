use crate::card::{Card, Suit, Rank};

use strum::IntoEnumIterator;

pub(crate) struct Deck {
	cards: Vec<Card>,
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
			};
			deck_count -= 1;
		}

		Deck { cards }
	}
}
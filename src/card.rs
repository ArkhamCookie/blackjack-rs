use std::fmt;

use strum::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter)]
pub(crate) enum Suit {
	Clubs,
	Diamonds,
	Hearts,
	Spades,
}

impl Suit {
	fn unicode(suit: Suit) -> String {
		match suit {
			Suit::Clubs => String::from("♣"),
			Suit::Diamonds => String::from("♦"),
			Suit::Hearts => String::from("♥"),
			Suit::Spades => String::from("♠"),
		}
	}
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub(crate) enum Rank {
	Ace,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Card {
	pub(crate) suit: Suit,
	pub(crate) rank: Rank,
}

impl Card {
	pub(crate) fn new(suit: Suit, rank: Rank) -> Self {
		Self { suit, rank }
	}

	pub(crate) fn value(card: &Card) -> u8 {
		match card.rank {
			Rank::Ace => 11,
			Rank::Two => 2,
			Rank::Three => 3,
			Rank::Four => 4,
			Rank::Five => 5,
			Rank::Six => 6,
			Rank::Seven => 7,
			Rank::Eight => 8,
			Rank::Nine => 9,
			Rank::Ten => 10,
			Rank::Jack => 10,
			Rank::Queen => 10,
			Rank::King => 10,
		}
	}
}

impl fmt::Display for Card {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let rank = match self.rank {
			Rank::Ace => String::from("A"),
			Rank::Jack => String::from("J"),
			Rank::Queen => String::from("Q"),
			Rank::King => String::from("K"),
			_ => Card::value(self).to_string()
		};
		let suit_unicode = Suit::unicode(self.suit);

		if self.rank == Rank::Ten {
			return write!(f, "┌────────┐\n│ {}   {} │\n│        │\n│        │\n│ {}   {} │\n└────────┘", rank, suit_unicode, suit_unicode, rank)
		}

		write!(f, "┌────────┐\n│ {}    {} │\n│        │\n│        │\n│ {}    {} │\n└────────┘", rank, suit_unicode, suit_unicode, rank)
	}
}

#[cfg(test)]
mod tests {
	use crate::deck::Deck;

	#[test]
	fn display_card_test() {
		let deck = Deck::shuffle(1);
		let card = deck.cards[0];

		println!("{}", card)
	}
}

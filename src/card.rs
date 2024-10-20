use strum::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter)]
pub(crate) enum Suit {
	Clubs,
	Diamonds,
	Hearts,
	Spades,
}

#[derive(Clone, Copy, Debug, EnumIter)]
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

#[derive(Debug)]
pub(crate) struct Card {
	suit: Suit,
	rank: Rank,
}

impl Card {
	pub(crate) fn new(suit: Suit, rank: Rank) -> Self {
		Self {
			suit,
			rank,
		}
	}
}
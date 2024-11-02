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

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
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

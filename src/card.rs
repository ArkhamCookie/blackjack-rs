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

	pub(crate) fn unicode(card: &Card) -> String {
		match card.suit {
			Suit::Clubs => {
				match card.rank {
					Rank::Ace => String::from("🃑"),
					Rank::Two => String::from("🃒"),
					Rank::Three => String::from("🃓"),
					Rank::Four => String::from("🃔"),
					Rank::Five => String::from("🃕"),
					Rank::Six => String::from("🃖"),
					Rank::Seven => String::from("🃗"),
					Rank::Eight => String::from("🃘"),
					Rank::Nine => String::from("🃙"),
					Rank::Ten => String::from("🃚"),
					Rank::Jack => String::from("🃛"),
					Rank::Queen => String::from("🃝"),
					Rank::King => String::from("🃞"),
				}
			},
			Suit::Diamonds => {
				match card.rank {
					Rank::Ace => String::from("🃁"),
					Rank::Two => String::from("🃂"),
					Rank::Three => String::from("🃃"),
					Rank::Four => String::from("🃄"),
					Rank::Five => String::from("🃅"),
					Rank::Six => String::from("🃆"),
					Rank::Seven => String::from("🃇"),
					Rank::Eight => String::from("🃈"),
					Rank::Nine => String::from("🃉"),
					Rank::Ten => String::from("🃊"),
					Rank::Jack => String::from("🃋"),
					Rank::Queen => String::from("🃍"),
					Rank::King => String::from("🃎"),
				}
			},
			Suit::Hearts => {
				match card.rank {
					Rank::Ace => String::from("🂱"),
					Rank::Two => String::from("🂲"),
					Rank::Three => String::from("🂳"),
					Rank::Four => String::from("🂴"),
					Rank::Five => String::from("🂵"),
					Rank::Six => String::from("🂶"),
					Rank::Seven => String::from("🂷"),
					Rank::Eight => String::from("🂸"),
					Rank::Nine => String::from("🂹"),
					Rank::Ten => String::from("🂺"),
					Rank::Jack => String::from("🂻"),
					Rank::Queen => String::from("🂽"),
					Rank::King => String::from("🂾"),
				}
			},
			Suit::Spades => {
				match card.rank {
					Rank::Ace => String::from("🂡"),
					Rank::Two => String::from("🂢"),
					Rank::Three => String::from("🂣"),
					Rank::Four => String::from("🂤"),
					Rank::Five => String::from("🂥"),
					Rank::Six => String::from("🂦"),
					Rank::Seven => String::from("🂧"),
					Rank::Eight => String::from("🂨"),
					Rank::Nine => String::from("🂩"),
					Rank::Ten => String::from("🂪"),
					Rank::Jack => String::from("🂫"),
					Rank::Queen => String::from("🂭"),
					Rank::King => String::from("🂮"),
				}
			},
		}
	}
}

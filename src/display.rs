use crate::Card;

/// Display given hand in terminal
pub(crate) fn display_hand(hand: &Vec<Card>) {
	for card in hand {
		println!("{}", card);
	}
}

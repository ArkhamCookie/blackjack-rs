use crate::card::Card;

/// Display given hand in terminal
pub fn display_hand(hand: &Vec<Card>) {
	for card in hand {
		println!("{}", card);
	}
}

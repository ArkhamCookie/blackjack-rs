use crate::Card;

pub(crate) fn display_hand(hand: &Vec<Card>) {
	for card in hand {
		println!("{}", card);
	}
}

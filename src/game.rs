use crate::card::Card;
use crate::deck::Deck;

pub(crate) fn deal() -> (Vec<Card>, Vec<Card>) {
	let deck = Deck::shuffle(1);
	let mut cards = deck.cards;
	let mut dealer_hand = Vec::new();
	let mut player_hand = Vec::new();

	// Deal 1 card to player
	player_hand.push(cards[0]);
	cards.remove(0);

	// Deal 1 card to dealer
	dealer_hand.push(cards[0]);
	cards.remove(0);

	// Deal 1 card to player
	player_hand.push(cards[0]);
	cards.remove(0);

	// Deal 1 card to dealer
	dealer_hand.push(cards[0]);
	cards.remove(0);

	(dealer_hand, player_hand)
}


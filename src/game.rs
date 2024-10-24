use crate::card::Card;
use crate::deck::Deck;

pub(crate) fn deal(deck: Deck) -> (Vec<Card>, Vec<Card>) {
	let mut cards = deck.cards;
	let mut dealer_hand = Vec::new();
	let mut player_hand = Vec::new();

	// Deal 1 card to player (face up)
	player_hand.push(cards[0]);
	cards.remove(0);

	// Deal 1 card to dealer (face down)
	dealer_hand.push(cards[0]);
	cards.remove(0);

	// Deal 1 card to player (face up)
	player_hand.push(cards[0]);
	cards.remove(0);

	// Deal 1 card to dealer (face up)
	dealer_hand.push(cards[0]);
	cards.remove(0);

	(dealer_hand, player_hand)
}


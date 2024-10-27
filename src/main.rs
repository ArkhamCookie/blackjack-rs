use deck::Deck;

mod card;
mod deck;
mod game;

fn main() {
    let deck = Deck::shuffle(1);
    let mut cards = deck.cards;
    let mut player_hand = Vec::new();
    let mut dealer_hand = Vec::new();

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
}

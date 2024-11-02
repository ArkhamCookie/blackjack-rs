use std::cmp::Ordering;

use card::Card;
use deck::Deck;

use inquire::Select;

mod card;
mod deck;

fn main() {
    let deck = Deck::shuffle(1);
    let mut cards = deck.cards;
    let mut player_hand = Vec::new();
    let mut dealer_hand = Vec::new();

    // [Setup] Deal 1 card to player (face up)
    player_hand.push(cards[0]);
    cards.remove(0);

    // [Setup] Deal 1 card to dealer (face down)
    dealer_hand.push(cards[0]);
    cards.remove(0);

    // [Setup] Deal 1 card to player (face up)
    player_hand.push(cards[0]);
    cards.remove(0);

    // [Setup] Deal 1 card to dealer (face up)
    dealer_hand.push(cards[0]);
    cards.remove(0);

    // Basic display
    println!("Dealer's Hand: Face Down, {:?}", dealer_hand[1].rank);

    println!("Your Hand: {:?}, {:?},", player_hand[0].rank, player_hand[1].rank);

    // Get player's action
    'player_action: loop {
        let options = vec!["Hit", "Stay"];
        let answer = Select::new("Hit or stay?", options).prompt().expect("issue getting user action");

        if answer == "Stay" {
            break
        }

        player_hand.push(cards[0]);
        cards.remove(0);
        
        let mut score: u8 = 0;

        print!("Your hand: ");
        for card in &player_hand {
            score += Card::value(card);
            print!("{:?}, ", card.rank);

            // Check for blackjack or bust
            match score.cmp(&21) {
                Ordering::Less => (),
                Ordering::Equal => {
                    println!("\nBlackjack!");
                    return
                }
                Ordering::Greater => {
                    println!("\nBusted!");
                    return
                }
            }
        }
        println!();
        // println!("{}", score);
    }

    'dealer_action: loop {
        let mut score: u8 = 0;

        print!("Dealer hand: ");
        for card in &dealer_hand {
            score += Card::value(card);
            print!("{:?}, ", card.rank);

            // Check for blackjack or bust
            match score.cmp(&21) {
                Ordering::Less => (),
                Ordering::Equal => {
                    println!("\nBlackjack!");
                    break 'dealer_action
                }
                Ordering::Greater => {
                    println!("\nBusted!");
                    break 'dealer_action
                }
            }
        }
        println!();
        // println!("{}", score);

        if score >= 17 {
            println!("Dealer stays.");
            break 'dealer_action
        }

        dealer_hand.push(cards[0]);
        cards.remove(0);
    }
}

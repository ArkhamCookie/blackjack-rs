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

    // Display starting hand
    println!("Dealer's Hand: 🂠, {}", Card::unicode(&dealer_hand[1]));
    println!("Your Hand: {}, {}", Card::unicode(&player_hand[0]), Card::unicode(&player_hand[1]));

    // Setup score vars for comparing
    let mut player_score: u8 = 0;
    let mut dealer_score: u8;

	// Get player's action
	loop {
		let options = vec!["Hit", "Stay"];
		let answer = Select::new("Hit or stay?", options)
			.prompt()
			.expect("issue getting user action");

		if answer == "Stay" {
			break;
		}

		player_hand.push(cards[0]);
		cards.remove(0);

		let mut score: u8 = 0;

		print!("Your hand: ");
		for card in &player_hand {
			score += Card::value(card);
			print!("{}, ", Card::unicode(card));

			// Check for blackjack or bust
			match score.cmp(&21) {
				Ordering::Less => (),
				Ordering::Equal => {
					println!("\nBlackjack!");
					return;
				}
				Ordering::Greater => {
					println!("\nBusted!");
					return;
				}
			}

		}
        player_score = score;

		println!();
		// println!("{}", score);
	}

	'dealer_action: loop {
		let mut score: u8 = 0;

	    print!("Dealer hand: ");
		for card in &dealer_hand {
			score += Card::value(card);
			print!("{}, ", Card::unicode(card));

			// Check for blackjack or bust
			match score.cmp(&21) {
				Ordering::Less => (),
				Ordering::Equal => {
					println!("\nDealer gets blackjack!");
					return;
				}
				Ordering::Greater => {
					println!("\nDealer busts!");
					return;
				}
			}
		}
        dealer_score = score;

		println!();
		// println!("{}", score);

		if score >= 17 {
			println!("Dealer stays.");
			break 'dealer_action;
		}

		dealer_hand.push(cards[0]);
		cards.remove(0);
	}

    // Get winner based on score
    match player_score.cmp(&dealer_score) {
        Ordering::Less => println!("Dealer wins!"),
        Ordering::Greater => println!("Player wins!"),
        _ => panic!("issue getting score"),
    }
}

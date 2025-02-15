use std::cmp::Ordering;

use card::{Card, Rank};
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
	// println!("Dealer's Hand: 🂠, {}", Card::unicode(&dealer_hand[1]));
	println!("Dealer's Hand:\n┌────────┐\n│        │\n│        │\n│        │\n│        │\n└────────┘\n{}",dealer_hand[1]);
	println!(
		"Your Hand: \n{}\n{}",
		&player_hand[0],
		&player_hand[1],
	);

	// Setup score vars for comparing
	let mut player_score: u8 = 0;
	let mut dealer_score: u8;
	let mut score: u8 = 0;

	for card in &player_hand {
		score += Card::value(card);

		if score == 21 {
			println!("\nBlackjack!");
			return;
		}
	}

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

		score = 0;

		println!("Your hand: ");
		'player_action: for card in &player_hand {
			score += Card::value(card);
			print!("{}\n", card);

			// Check for blackjack or bust
			match score.cmp(&21) {
				Ordering::Less => (),
				Ordering::Equal => {
					println!("Blackjack!");
					return;
				}
				Ordering::Greater => {
					// Check and handle if player has an ace
					let mut ace_score = score;
					for card in &player_hand {
						if card.rank == Rank::Ace {
							ace_score -= 10;

							match ace_score.cmp(&21) {
								Ordering::Less => {
									continue 'player_action;
								}
								Ordering::Equal => {
									println!("Blackjack!");
									return;
								}
								Ordering::Greater => (),
							}
						}
					}
					println!("Busted!");
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

		println!("Dealer hand: ");
		for card in &dealer_hand {
			score += Card::value(card);
			print!("{}\n", card);

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
		Ordering::Equal => println!("Dealer & Player tie!"),
		Ordering::Greater => println!("Player wins!"),
	}
}

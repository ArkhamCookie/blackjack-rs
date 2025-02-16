use crate::blackjack::{check_hand, GameEvents};
use crate::card::Card;
use crate::deck::Deck;
use crate::display::display_hand;
use game::compare_hands;

use inquire::Select;

mod blackjack;
mod card;
mod deck;
mod display;
mod game;

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
	println!("Dealer's Hand:\n┌────────┐\n│        │\n│        │\n│        │\n│        │\n└────────┘\n{}",dealer_hand[1]);
	println!("Your Hand: \n{}\n{}", &player_hand[0], &player_hand[1]);

	// Setup score vars for comparing
	let mut score: u8 = 0;

	for card in &player_hand {
		score += Card::value(card);

		if score == 21 {
			println!("Your hand:");
			display_hand(&player_hand);
			println!("Blackjack!");
			return;
		}
	}

	// Get player's action
	loop {
		// Get player's choice
		let options = vec!["Hit", "Stay"];
		let answer = Select::new("Hit or stay?", options)
			.prompt()
			.expect("issue getting user action");

		if answer == "Stay" {
			break;
		}

		// Deal card
		player_hand.push(cards[0]);
		cards.remove(0);

		score = 0;

		if answer == "Hit" {
			println!("Your hand:");
			display_hand(&player_hand);

			for card in &player_hand {
				score += Card::value(card);
			}

			// Get what hand is
			let event = check_hand(&player_hand);

			match event {
				GameEvents::Safe => continue,
				GameEvents::Blackjack => {
					println!("Blackjack!");
					return;
				}
				GameEvents::Bust => {
					println!("Busted!");
					return;
				}
			}
		}
	}

	// Handle dealer's actions
	loop {
		println!("Dealer hand:");
		display_hand(&dealer_hand);

		let mut score: u8 = 0;

		for card in &dealer_hand {
			score += Card::value(card);
		}

		let event = check_hand(&dealer_hand);

		match event {
			GameEvents::Safe => {
				if score >= 17 {
					println!("Dealer stays.");
					break;
				}
			}
			GameEvents::Blackjack => {
				println!("Dealer gets Blackjack");
				return;
			}
			GameEvents::Bust => {
				println!("Dealer busted!");
				return;
			}
		}

		dealer_hand.push(cards[0]);
		cards.remove(0);
	}

	// Get winner based on score
	match compare_hands(&player_hand, &dealer_hand) {
		1 => println!("Player Wins!"),
		2 => println!("Dealer Wins!"),
		3 => println!("Player & Dealer Tie!"),
		_ => eprintln!("issue getting winner"),
	}
}

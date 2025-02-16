use std::cmp::Ordering;

use crate::blackjack::{check_blackjack, GameEvents};
use crate::card::{Card, Rank};
use crate::deck::Deck;
use crate::display::display_hand;

use inquire::Select;

mod blackjack;
mod card;
mod deck;
mod display;

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
	let player_score: u8;
	let mut dealer_score: u8;
	let mut score: u8 = 0;

	for card in &player_hand {
		score += Card::value(card);

		if score == 21 {
			println!("Your hand:");
			display_hand(&player_hand);
			println!("Blackjack!")
		}
	}

	// Get player's action
	'player_action: loop {
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

		while answer == "Hit" {
			println!("Your hand:");
			display_hand(&player_hand);

			for card in &player_hand {
				score += Card::value(card);
			}

			let event = check_blackjack(&player_hand);

			match event {
				GameEvents::Safe => continue 'player_action,
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
	player_score = score;

	'dealer_action: loop {
		println!("Dealer hand:");
		display_hand(&dealer_hand);

		let mut score: u8 = 0;

		for card in &dealer_hand {
			score += Card::value(card);
		}
		dealer_score = score;
		let event = check_blackjack(&dealer_hand);

		match event {
			GameEvents::Safe => {
				if score >= 17 {
					println!("Dealer stays.");
					break 'dealer_action;
				}
			}
			GameEvents::Blackjack => {
				println!("Dealer gets Blackjack");
				return;
			}
			GameEvents::Bust => {
				println!("Dealer busted!");
			}
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

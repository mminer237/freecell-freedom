use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(EnumIter)]
pub enum Suit {
	Spades,
	Clubs,
	Hearts,
	Diamonds,
}

pub struct Card {
	pub suit: Suit,
	pub number: u16,
}

#[derive(Default)]
pub struct Stack {
	pub cards: Vec<Card>
}

#[derive(Default)]
pub struct Foundation {
	cards: Vec<Card>
}

#[derive(Default)]
pub struct Game {
	pub free_cells: [Option<Card>; 4],
	pub foundations: [Foundation; 4],
	pub stacks: [Stack; 8],
}

struct GameSeed(ChaCha8Rng);
impl GameSeed {
	fn new(seed: u64) -> Self {
		GameSeed(ChaCha8Rng::seed_from_u64(seed))
	}

	fn get_next_number(&mut self, max: usize) -> usize {
		self.0.gen_range(1..max)
	}
}

impl Game {
	pub fn new(seed_number: u64) -> Self {
		let mut game_seed = GameSeed::new(seed_number);
		let mut deck = Vec::new();
		for number in 1..13u16 {
			for suit in Suit::iter() {
				deck.push(Card{
					suit,
					number
				});
			}
		}

		let mut stacks: [Stack; 8] = Default::default();
		for deck_index in (0..deck.len()).rev() {
			let stack_sizes: [usize; 8] = [7, 7, 7, 7, 6, 6, 6, 6];
			for stack_list_index in 0..stack_sizes.len() {
				let stack = &mut stacks[stack_list_index];
				for stack_index in 0..stack_sizes[stack_list_index] {
					let next_number = game_seed.get_next_number(deck_index);
					stack.cards[stack_index] = deck.remove(next_number);
				}
			}
		}

		Self {
			stacks,
			..Default::default()
		}
	}
}

pub fn add(left: usize, right: usize) -> usize {
	left + right
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}

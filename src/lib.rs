use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Clone, Copy, Debug, EnumIter, PartialEq)]
pub enum Suit {
	Spades,
	Clubs,
	Hearts,
	Diamonds,
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
	pub suit: Suit,
	pub number: u16,
}

#[derive(Debug, Default)]
pub struct Stack {
	pub cards: Vec<Card>
}

#[derive(Debug, Default)]
pub struct FreeCell {
	card: Option<Card>
}

#[derive(Debug, Default)]
pub struct Foundation {
	cards: Vec<Card>
}

pub trait Cell {
	fn last_card(&self) -> Option<&Card>;
}

impl Cell for FreeCell {
	fn last_card(&self) -> Option<&Card> {
		self.card.as_ref()
	}
}

impl Cell for Foundation {
	fn last_card(&self) -> Option<&Card> {
		self.cards.last()
	}
}

#[derive(Debug, Default)]
pub struct Game {
	pub seed_number: u64,
	pub free_cells: [FreeCell; 4],
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
		for number in 1..13u16+1 {
			for suit in Suit::iter() {
				deck.push(Card{
					suit,
					number
				});
			}
		}

		let mut stacks: [Stack; 8] = Default::default();
		let stack_sizes: [usize; 8] = [7, 7, 7, 7, 6, 6, 6, 6];
		for stack_list_index in 0..stack_sizes.len() {
			let stack = &mut stacks[stack_list_index];
			for _ in 0..stack_sizes[stack_list_index] {
				if deck.len() > 1 {
					let next_number = game_seed.get_next_number(deck.len());
					stack.cards.push(deck.remove(next_number));
				}
				else {
					stack.cards.push(deck.remove(0));
				}
			}
		}

		Self {
			seed_number,
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

mod card_view;

use std::{alloc::Layout, cmp::max, cmp::min, thread::sleep, time::Duration, default, rc::Rc};

use card_view::CardView;
use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use freecell_freedom::{Game, Suit, Cell, Card, Stack};
use rand::Rng;
use rust_i18n::t;
rust_i18n::i18n!("locales");
use cursive::{theme::{BorderStyle, Palette}, views::{Panel, ScrollView, PaddedView}, Cursive, XY, View};
use cursive::traits::With;
use cursive::views::{Button, LinearLayout, TextView};

fn main() {
	let mut siv = cursive::default();

	siv.set_theme(cursive::theme::Theme {
		shadow: false,
		borders: BorderStyle::Simple,
		palette: Palette::default().with(|palette| {
			use cursive::theme::BaseColor::*;
			use cursive::theme::Color::TerminalDefault;
			use cursive::theme::PaletteColor::*;

			palette[Background] = TerminalDefault;
			palette[View] = TerminalDefault;

			palette[Primary] = White.dark();
			palette[TitlePrimary] = Red.light();
			palette[Secondary] = Red.light();
			palette[Highlight] = Red.dark();
		}),
	});

	siv.add_global_callback('q', |s| s.quit());

	siv.add_layer(
		LinearLayout::vertical()
			.child(TextView::new(t!("title")))
			.child(Button::new(t!("new_game"), |s| {
				s.pop_layer();
				new_random_game(s)
			}))
			.child(Button::new(t!("quit"), |s| s.quit()))
	);

	siv.run();
}

fn new_random_game(siv: &mut Cursive) {
	let mut game = Game::new(rand::thread_rng().gen());
	draw_game(&mut game, siv);
}

fn draw_game(game: &mut Game, siv: &mut Cursive) {
	let card_style = Rc::new(get_card_style(siv.screen_size()));
	// eprintln!("{:?}", card_style);
	// sleep(Duration::new(2, 0)); // TEMP
	siv.add_layer(
		LinearLayout::vertical()
			.child(LinearLayout::horizontal()
				.child(Button::new(t!("quit"), |s| s.quit()))
			)
			.child(LinearLayout::horizontal()
				.child(get_cells(game.free_cells.iter().map(|x| x as &dyn Cell), card_style.clone()))
				.child(TextView::new(if card_style.spacing > 0 {" ".repeat(card_style.spacing * 7)} else { "".to_string() }))
				.child(get_cells(game.foundations.iter().map(|x| x as &dyn Cell), card_style.clone()))
			)
			.child(LinearLayout::horizontal()
				.child(
					ScrollView::new(get_stacks(&game.stacks, card_style))
				)
			)
	);

}

#[derive(Debug, PartialEq)]
enum CardBorder {
	Full,
	Embeded,
	None
}

#[derive(Debug)]
pub struct CardStyle<T> {
	x: T,
	y: T,
	border: CardBorder,
	spacing: T
}

fn get_card_style(screen_size: XY<usize>) -> CardStyle<usize> {
	let border = if screen_size.x >= 39 {
		if screen_size.y >= 33 { CardBorder::Full } else { CardBorder::Embeded }
	}
	else {
		CardBorder::None
	};
	let mut max_width = (screen_size.x - 8) / 8;
	if max_width % 2 == 0 {
		max_width -= 1; // Ensure card are an odd width
	}
	let mut max_height = if screen_size.y >= 33 {
		(screen_size.y - 27) / 2
	}
	else {
		if screen_size.y > 20 {
			(screen_size.y - 18) / 2
		}
		else {
			if border == CardBorder::None {
				1
			}
			else {
				3
			}
		}
	};

	if max_width / 5 / 2 <= max_height / 7 {
		max_height = max_width * 7 / 5 / 2;
	}
	else {
		max_width = max_height * 5 * 2 / 7;
	}

	let spacing = min((screen_size.x - 1 - max_width * 8) / 7, 4);

	CardStyle { x: max_width, y: max_height, border, spacing }
}

fn get_cells<'a>(cells: impl Iterator<Item = &'a dyn Cell>, card_style: Rc<CardStyle<usize>>) -> LinearLayout {
	let mut layout = LinearLayout::horizontal();
	let mut cards: Vec<&Card> = Default::default();
	for cell in cells {
		layout.add_child(CardView::new(cell.last_card().map(|x| *x), card_style.clone()));
	}
	layout
}

fn get_stacks(stacks: &[Stack; 8], card_style: Rc<CardStyle<usize>>) -> LinearLayout {
	let mut layout = LinearLayout::horizontal();
	if let Some((first_stack, most_stacks)) = stacks.split_first() {
		layout.add_child(PaddedView::lrtb(0, 0, 0, 0, get_stack(first_stack, card_style.clone())));
		for stack in most_stacks {
			layout.add_child(PaddedView::lrtb(card_style.spacing, 0, 0, 0, get_stack(stack, card_style.clone())));
		}
	}
	layout
}

fn get_stack(stack: &Stack, card_style: Rc<CardStyle<usize>>) -> LinearLayout {
    let mut stack_layout = LinearLayout::vertical();
    if let Some((last_card, partial_cards)) = stack.cards.split_last() {
		for card in partial_cards {
			stack_layout.add_child(render_partial_card(card, card_style.clone()));
		}
		stack_layout.add_child(CardView::new(Some(*last_card), card_style.clone()));
	}
    stack_layout
}

fn number_symbol(number: &u16, upside_down: bool) -> String {
    if !upside_down {
		match number {
			1 => "A".to_string(),
			11 => "J".to_string(),
			12 => "Q".to_string(),
			13 => "K".to_string(),
			_  => number.to_string()
		}
	}
	else {
		match number {
			1 => "∀".to_string(),
			2 => "↊".to_string(),
			3 => "↋".to_string(),
			4 => "ߤ".to_string(),
			6 => "9".to_string(),
			7 => "𝘓".to_string(),
			9 => "6".to_string(),
			10 => "0⇂".to_string(),
			11 => "ᒋ".to_string(),
			12 => "Ꝺ".to_string(),
			13 => "ꓘ".to_string(),
			_  => number.to_string()
		}
	}
}

fn suit_symbol(suit: &Suit) -> &'static str {
	#[cfg(target_os = "windows")]
	match suit {
		Suit::Spades => "♠",
		Suit::Clubs => "♣",
		Suit::Hearts => "♥",
		Suit::Diamonds => "♦"
	}
	#[cfg(not(target_os = "windows"))]
	match suit {
		Suit::Spades => "♠️",
		Suit::Clubs => "♣️",
		Suit::Hearts => "♥️",
		Suit::Diamonds => "♦️"
	}
}

fn render_partial_card(card: &Card, card_style: Rc<CardStyle<usize>>) -> TextView {
	let mut card_text = suit_symbol(&card.suit).to_string() + &number_symbol(&card.number, false);
	#[cfg(target_os = "windows")]
	const EMOJI_LENGTH: &'static usize = &3;
	#[cfg(not(target_os = "windows"))]
	const EMOJI_LENGTH: &'static usize = &6;
	if card_text.len() < EMOJI_LENGTH + 2 {
		if card_style.border == CardBorder::Embeded {
			card_text += "─";
		}
		else {
			card_text += " ";
		}
	}
	if card_style.border == CardBorder::None {
		if card_style.x > 3 {
			TextView::new(" ".repeat(card_style.x - 3) + &card_text)
		}
		else {
			TextView::new(card_text)
		}
	} else {
		TextView::new(
			"┌".to_string() +
			&(if card_style.border == CardBorder::Full {
				"─".repeat(card_style.x - 2)
			}
			else {
				(if card_style.x > 5 {
					"─".repeat(card_style.x - 5)
				} else { "".to_owned() }) +
				&card_text
			}) +
			&"┐".to_string() +
			&(if card_style.border == CardBorder::Full {
				"\n│".to_owned() +
				&(if card_style.x > 5 { " ".repeat(card_style.x - 5) } else { "".to_string() }) + &card_text +
				"│"
			} else { "".to_string() })
		)
	}
}

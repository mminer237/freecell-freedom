use std::rc::Rc;

use cursive::{View, theme::{Style, ColorStyle}, Vec2, XY};
use freecell_freedom::Card;

use crate::{CardStyle, CardBorder, suit_symbol, number_symbol};

#[derive(Debug)]
pub struct CardView {
	card: Option<Card>,
	card_style: Rc<CardStyle<usize>>
}
impl CardView {
	pub fn new(card: Option<Card>, card_style: Rc<CardStyle<usize>>) -> Self {
		Self{card, card_style}
	}
}
impl View for CardView {
	fn draw(&self, printer: &cursive::Printer) {
		if printer.size.x == 0 {
			return;
		}
		let xy: usize = self.card_style.x;

		let style = if printer.focused {
			ColorStyle::highlight()
		} else {
			ColorStyle::primary()
		};
		/* TODO: Make some cards red */

		let border_size = if self.card_style.border == CardBorder::None { 0 } else { 1 };

		printer.with_color(style, |printer| {
			/* Print card border */
			if self.card_style.border != CardBorder::None {
				printer.print_box(Vec2::new(0, 0), printer.size, false);
			}

			/* Print card body */
			if let Some(card) = self.card {
				printer.print((self.card_style.x - border_size - 3, border_size), 
					&(suit_symbol(&card.suit).to_owned() + &number_symbol(&card.number, false)));
				if self.card_style.y - border_size * 2 - 2 > 0 {
					for (row, line) in card_art(&card, XY{x: self.card_style.x - border_size * 2, y: self.card_style.y - border_size * 2 - 2}).lines().enumerate() {
						printer.print((border_size, row + border_size + 1), line);
					}
				}
				if self.card_style.y - border_size * 2 > 1 {
					printer.print((border_size, self.card_style.y - border_size * 2), 
						&(number_symbol(&card.number, true) + suit_symbol(&card.suit) + &" ".repeat(self.card_style.x - border_size * 2 - 3)));
				}
			}
			else {
				for row in border_size..self.card_style.y - border_size {
					printer.print((border_size, row), &"▒".repeat(self.card_style.x - border_size * 2));
				}
			};
		});
	}

	fn layout(&mut self, _: cursive::Vec2) {}

	fn needs_relayout(&self) -> bool {
		true
	}

	fn required_size(&mut self, constraint: cursive::Vec2) -> cursive::Vec2 {
		let _ = constraint;
		cursive::Vec2::new(self.card_style.x, self.card_style.y)
	}

	fn on_event(&mut self, _: cursive::event::Event) -> cursive::event::EventResult {
		cursive::event::EventResult::Ignored
	}

	fn call_on_any<'a>(&mut self, _: &cursive::view::Selector<'_>, _: cursive::event::AnyCb<'a>) {}

	fn focus_view(
		&mut self,
		_: &cursive::view::Selector<'_>,
	) -> Result<cursive::event::EventResult, cursive::view::ViewNotFound> {
		Err(cursive::view::ViewNotFound)
	}

	fn take_focus(
		&mut self,
		source: cursive::direction::Direction,
	) -> Result<cursive::event::EventResult, cursive::view::CannotFocus> {
		let _ = source;

		Err(cursive::view::CannotFocus)
	}

	fn important_area(&self, view_size: cursive::Vec2) -> cursive::Rect {
		cursive::Rect::from_size((0, 0), view_size)
	}
}

fn card_art(card: &Card, size: XY<usize>) -> String {
	let symbol = suit_symbol(&card.suit);
	let art = match card.number {
		1 => {
			(" ".repeat(size.x) + "\n").repeat((size.y - 1) / 2) +
			&" ".repeat((size.x - 1) / 2) + symbol + &" ".repeat((size.x - 1) / 2) + "\n" +
			&(" ".repeat(size.x) + "\n").repeat((size.y - 1) / 2 - 1) + &(" ".repeat(size.x))
		},
		2 => {
			let mut padding_lines = [(size.y - 2) / 3, (size.y - 2) / 3, (size.y - 2) / 3];
			if padding_lines[1] * 3 < size.y - 2 {
				padding_lines[1] += size.y - 2 - padding_lines[1] * 3;
			}
			(" ".repeat(size.x) + "\n").repeat(padding_lines[0]) +
			&" ".repeat((size.x - 1) / 2) + symbol + &" ".repeat((size.x - 1) / 2) + "\n" +
			&(" ".repeat(size.x) + "\n").repeat(padding_lines[1]) +
			&" ".repeat((size.x - 1) / 2) + symbol + &" ".repeat((size.x - 1) / 2) + "\n" +
			&(" ".repeat(size.x) + "\n").repeat(padding_lines[2] - 1) + &(" ".repeat(size.x))
		},
		3 => {
			" ".repeat((size.x - 1) / 2) + symbol + &" ".repeat((size.x - 1) / 2) + "\n" +
			&(" ".repeat(size.x) + "\n").repeat((size.y - 3) / 2) +
			&" ".repeat((size.x - 1) / 2) + symbol + &" ".repeat((size.x - 1) / 2) + "\n" +
			&(" ".repeat(size.x) + "\n").repeat((size.y - 3) / 2) +
			&" ".repeat((size.x - 1) / 2) + symbol + &" ".repeat((size.x - 1) / 2)
		},
		11 => {
			(" ".repeat(size.x) + "\n").repeat((size.y - 1) / 2) +
			&" ".repeat((size.x - 1) / 2) + "👲" + &" ".repeat((size.x - 2) / 2) + "\n" +
			&(" ".repeat(size.x) + "\n").repeat((size.y - 1) / 2 - 1) + &(" ".repeat(size.x))
		},
		12 => {
			(" ".repeat(size.x) + "\n").repeat((size.y - 1) / 2) +
			&" ".repeat((size.x - 1) / 2) + "👸" + &" ".repeat((size.x - 2) / 2) + "\n" +
			&(" ".repeat(size.x) + "\n").repeat((size.y - 1) / 2 - 1) + &(" ".repeat(size.x))
		},
		13 => {
			(" ".repeat(size.x) + "\n").repeat((size.y - 1) / 2) +
			&" ".repeat((size.x - 1) / 2) + "🤴" + &" ".repeat((size.x - 2) / 2) + "\n" +
			&(" ".repeat(size.x) + "\n").repeat((size.y - 1) / 2 - 1) + &(" ".repeat(size.x))
		},
		_ => {
			(" ".repeat(size.x) + "\n").repeat(size.y - 1) + &(" ".repeat(size.x))
		},
	};
	let art_lines = art.lines().count();
	art +
	&(if art_lines < size.y { ("\n".to_owned() + &" ".repeat(size.x)).repeat(size.y - art_lines) } else { "".to_string() })
}

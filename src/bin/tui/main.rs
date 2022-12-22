use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use freecell_freedom::{Game, Suit};
use rand::Rng;
use rust_i18n::t;
rust_i18n::i18n!("locales");
use cursive::theme::{BorderStyle, Palette};
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
				new_random_game()
			}))
			.child(Button::new(t!("quit"), |s| s.quit())),
	);

	siv.run();
}

fn new_random_game() {
	let mut game = Game::new(rand::thread_rng().gen());
	
}

use crossterm::event::Event;
use crossterm::event::{ read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers };
use std::io::Error;
mod terminal;
use terminal::{ Position, Size, Terminal };

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent { code, modifiers, .. }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye. \r\n")?;
        } else {
            Self::draw_rows()?;
            Self::draw_welcome_message()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for curr_row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if curr_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let welcome_message = format!("{NAME} editor -- version {VERSION}");

        #[allow(clippy::integer_division)]
        let message_half_len: usize = welcome_message.len() / 2;
        let Size { height, width } = Terminal::size()?;

        // handle overflow if the message size is too big
        let x = if message_half_len > (u16::MAX as usize) {
            0
        } else {
            width / 2 - message_half_len
        };

        let y = height / 3;

        Terminal::move_cursor_to(Position { x, y })?;
        Terminal::print(&welcome_message)?;
        Ok(())
    }
}

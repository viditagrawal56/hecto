use std::io::Error;

use super::terminal::{ Position, Size, Terminal };

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
pub struct View;

impl View {
    pub fn render() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for curr_row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if curr_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Self::draw_welcome_message()?;
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

        Terminal::move_caret_to(Position { col: x, row: y })?;
        Terminal::print(&welcome_message)?;
        Ok(())
    }
}

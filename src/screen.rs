use std::io;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, 
               disable_raw_mode, Clear, ClearType},
    cursor::{Hide, Show, MoveTo},
    style::Print,
};

pub struct Screen; // namespace for static methods

impl Screen {
    pub fn enable_raw() -> io::Result<()> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, Hide)?;
        Ok(())
    }

    pub fn disable_raw() -> io::Result<()> {
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen, Show)?;
        Ok(())
    }

    pub fn clear_screen() -> io::Result<()> {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        Ok(())
    }

    pub fn draw_lines(lines: &[String]) -> io::Result<()> {
        //let mut stdout = io::stdout();
        for (i, lines) in lines.iter().enumerate() {
            execute!(
                io::stdout(), 
                MoveTo(0, i as u16), 
                Print(format!("{:>4} | {}", i + 1, lines))
            )?;
        }
    Ok(())
    }
}

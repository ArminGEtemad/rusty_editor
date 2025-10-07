use crate::{document::Document, screen::Screen};
use crossterm::{cursor::{Show, MoveTo}, event::{self, Event, KeyCode, KeyEvent, KeyModifiers}, style::Print, execute};
use std::io::{self};

const HORIZONTAL_OFFSET: u16 = 6; // supporting under 10k lines for now
                                  // because Print(format!("{:>4} | {}", i + 1, lines))

pub struct Editor {
    doc: Document,
    cursor_x: usize,
    cursor_y: usize,
    dirty: bool, // if file has been changed
    filename: String,
}

impl Editor {
    pub fn new(doc: Document, filename: String) -> Self {
        Self {
            doc,
            cursor_x: 0,
            cursor_y: 0,
            dirty: false,
            filename,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        Screen::enable_raw()?;

        let _guard = scopeguard::guard((), |_| {
            let _ = Screen::disable_raw();
        });

        loop {
            Screen::clear_screen()?;
            self.draw()?;
            if self.handle_input()? {
                break;
            }
        }
        Screen::disable_raw()
    }

    fn draw(&self) -> io::Result<()> {
        Screen::draw_lines(&self.doc.lines())?;
        crossterm::execute!(
            io::stdout(),
            crossterm::cursor::MoveTo(self.cursor_x as u16 + HORIZONTAL_OFFSET, self.cursor_y as u16),
            Show, // the cursor becomes visible
        )?;
        Ok(())
    }

    fn clamp_cursor_x(&mut self) { // getting to the end of the next line if prev line is longer
        if let Some(line) = self.doc.lines.get(self.cursor_y) {
            self.cursor_x = self.cursor_x.min(line.len());
        } else {
            self.cursor_x = 0;
        }
    }

    fn save_to_file(&mut self) -> io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(&self.filename)?;
        for line in &self.doc.lines {
            writeln!(file, "{}", line)?;
        }
        self.dirty = false;
        Ok(())
    }
    
    fn show_message(&self, msg: &str) -> io::Result<()> {
        let y = self.doc.lines.len() as u16 + 1;
        execute!(
            io::stdout(),
            MoveTo(0, y),
            Print(msg),
        ) 
    }

    fn handle_input(&mut self) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') if modifiers.contains(KeyModifiers::CONTROL) => {
                        // Stopping the user to quit if unsaved changes first
                        if self.dirty {
                            self.show_message("Usaved Changes. Ctrl+S to save or Ctrl+Q to force Quit!")?;
                            std::thread::sleep(std::time::Duration::from_secs(3)); 
                            self.dirty = false; // if Ctrl+Q is pressed again can quit
                            return  Ok(false);
                        } else {
                            return Ok(true);
                        }
                    }
                    KeyCode::Char('s') if modifiers.contains(KeyModifiers::CONTROL) => {
                        self.save_to_file()?;
                    }
                    KeyCode::Up => {
                        self.cursor_y = self.cursor_y.saturating_sub(1);
                        self.clamp_cursor_x();
                    }
                    KeyCode::Down => {
                        if self.cursor_y < self.doc.lines.len().saturating_sub(1) {
                            self.cursor_y += 1;
                            self.clamp_cursor_x();
                        }
                    }
                    KeyCode::Left => {
                        if self.cursor_x > 0 {
                            self.cursor_x -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if let Some(line) = self.doc.lines.get(self.cursor_y) {
                            if self.cursor_x < line.len() {
                                self.cursor_x += 1;
                            }
                        }
                    }
                    KeyCode::Char(c) => {
                        if let Some(line) = self.doc.lines.get_mut(self.cursor_y) {
                            if self.cursor_x > line.len() {
                                self.cursor_x = line.len();
                            }
                            line.insert(self.cursor_x, c);
                            self.cursor_x += 1;
                            self.dirty = true;
                        }
                    }
                    KeyCode::Backspace => {
                        if self.cursor_x > 0 {
                            if let Some(line) = self.doc.lines.get_mut(self.cursor_y) {
                                if self.cursor_x <= line.len() {
                                    line.remove(self.cursor_x - 1);
                                    self.cursor_x -= 1;
                                    self.dirty = true;
                                }
                            }
                        } else if self.cursor_y > 0 {
                            // merging lines
                            let current = self.doc.lines.remove(self.cursor_y);
                            self.cursor_y -= 1;

                            if let Some(prev) = self.doc.lines.get_mut(self.cursor_y) {
                                self.cursor_x = prev.len();
                                prev.push_str(&current);
                            }
                            self.dirty = true;
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(line) = self.doc.lines.get_mut(self.cursor_y) {
                            let new_line = line.split_off(self.cursor_x);
                            self.cursor_y += 1;
                            self.cursor_x = 0;
                            self.doc.lines.insert(self.cursor_y, new_line);
                            self.dirty = true;
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(false)
    }
}

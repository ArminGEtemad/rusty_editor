use crate::{document::Document, screen::Screen};
use crossterm::{cursor::Show, event::{self, Event, KeyCode, KeyEvent, KeyModifiers}};
use std::io::{self};

const HORIZONTAL_OFFSET: u16 = 6; // supporting under 10k lines for now
                                  // because Print(format!("{:>4} | {}", i + 1, lines))

pub struct Editor {
    doc: Document,
    cursor_x: usize,
    cursor_y: usize,
}

impl Editor {
    pub fn new(doc: Document) -> Self {
        Self {
            doc,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        Screen::enable_raw()?;
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

    fn handle_input(&mut self) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') if modifiers.contains(KeyModifiers::CONTROL)=> {
                        return Ok(true);
                    } 
                    KeyCode::Up => {
                        if self.cursor_y > 0 {
                            self.cursor_y -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.cursor_y < self.doc.lines.len().saturating_sub(1) {
                            self.cursor_y += 1;
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
                            if self.cursor_x < line.len() {
                                line.insert(self.cursor_x, c);
                                self.cursor_x += 1;
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if self.cursor_x > 0 {
                            if let Some(line) = self.doc.lines.get_mut(self.cursor_y) {
                                if self.cursor_x < line.len() {
                                    line.remove(self.cursor_x - 1);
                                    self.cursor_x -= 1;
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
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(line) = self.doc.lines.get_mut(self.cursor_y) {
                            let new_line = line.split_off(self.cursor_x);
                            self.cursor_y += 1;
                            self.cursor_x = 0;
                            self.doc.lines.insert(self.cursor_y, new_line);
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(false)
    }
}

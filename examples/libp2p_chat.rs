//! # [Ratatui] Tabs example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Position, Rect},
    style::{palette::tailwind, Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Padding, Paragraph, Tabs, Widget},
    DefaultTerminal, Frame,
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

#[derive(Default)]
struct App {
    /// Current value of the input box
    input: String,
    /// Position of cursor in the editor area.
    character_index: usize,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,
    state: AppState,
    selected_tab: SelectedTab,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    #[default]
    Normal,
    Editing,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "Tab 1")]
    Tab1,
    #[strum(to_string = "Tab 2")]
    Tab2,
    #[strum(to_string = "Tab 3")]
    Tab3,
    #[strum(to_string = "Tab 4")]
    Tab4,
}

impl App {
    const fn new() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            character_index: 0,
            state: AppState::Running,
            selected_tab: SelectedTab::Tab1,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn submit_message(&mut self) {
        self.messages.push(self.input.clone());
        self.input.clear();
        self.reset_cursor();
    }

    fn draw(&self, frame: &mut Frame) {
        //setup frame
        let vertical = Layout::vertical([
            Constraint::Fill(80), //messages_area
            Constraint::Min(3),   //input_area
            Constraint::Max(1),   //help_area
        ]);
        let [messages_area, input_area, help_area] = vertical.areas(frame.area());
        let horizontal = Layout::vertical([
            Constraint::Fill(3), //title_area
            Constraint::Fill(3), //tabs_area
        ]);
        let [title_area, tabs_area] = horizontal.areas(frame.area());

        //detect input_mode
        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    " ".into(),
                    "Esc".bold(),
                    " <QUIT> ".into(),
                    "e".bold(),
                    " <EDIT>".bold(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    " ".into(),
                    "Esc".bold(),
                    " <NORMAL> ".into(),
                    "<Enter>".bold(),
                    " <POST>".into(),
                ],
                Style::default(),
            ),
        };

        //title_area stub
        //create a Text element
        let text = Text::from(Line::from("           Title Area")).patch_style(style.clone());

        //create Paragraph with Text element content
        let help_message = Paragraph::new(text);

        //render to frame
        frame.render_widget(help_message, title_area);

        //tabs_area stub
        //create a Text element
        let text = Text::from(Line::from("           Tabs Area")).patch_style(style.clone());

        //create Paragraph with Text element content
        let help_message = Paragraph::new(text);

        //render to frame
        frame.render_widget(help_message, tabs_area);

        //create a Text element
        let text = Text::from(Line::from(msg)).patch_style(style);

        //create Paragraph with Text element content
        let help_message = Paragraph::new(text);

        //render to frame
        frame.render_widget(help_message, help_area);

        //
        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("Input"));

        //render to frame
        frame.render_widget(input, input_area);

        //
        match self.input_mode {
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            InputMode::Normal => {}

            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Editing => frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                input_area.x + self.character_index as u16 + 1,
                // Move one line down, from the border to the input line
                input_area.y + 1,
            )),
        }

        //
        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = Line::from(Span::raw(format!("{i}: {m}")));
                ListItem::new(content)
            })
            .collect();
        let messages = List::new(messages).block(Block::bordered().title("Messages"));

        //render to frame
        frame.render_widget(messages, messages_area);
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.state == AppState::Running {
            //terminal.draw(|frame| frame.render_widget(&self, frame.area())
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            match self.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        self.input_mode = InputMode::Editing;
                    }
                    //KeyCode::Char('q') => {
                    //    return Ok(());
                    //}
                    KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                    KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                    KeyCode::Char('q') | KeyCode::Esc => self.quit(),

                    _ => {}
                },
                InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => self.submit_message(),
                    KeyCode::Char(to_insert) => self.enter_char(to_insert),
                    KeyCode::Backspace => self.delete_char(),
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
                    KeyCode::Esc => self.input_mode = InputMode::Normal,
                    _ => {}
                },
                InputMode::Editing => {}
            }
        }
        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        self.selected_tab.render(inner_area, buf);
        render_footer(footer_area, buf);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        //           //text        //background color of widget
        //                         //Color::Reset bkgrnd of terminal
        let style = (Color::Magenta, Color::Reset);
        //let style = (self.selected_tab.palette().c500, Color::Reset);
        //text       //background of selected tab
        //                                 //Color::Reset bkgrnd of terminal
        let highlight_style = (Color::White, Color::Reset);
        //let highlight_style = (self.selected_tab.palette().c500, Color::Reset);
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .style(style)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "Ratatui Tabs Example".magenta().bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("◄ ► to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // in a real app these might be separate widgets
        match self {
            Self::Tab1 => self.render_tab0(area, buf),
            Self::Tab2 => self.render_tab1(area, buf),
            Self::Tab3 => self.render_tab2(area, buf),
            Self::Tab4 => self.render_tab3(area, buf),
        }
    }
}

impl SelectedTab {
    /// Return tab's name as a styled `Line`
    fn title(self) -> Line<'static> {
        format!("  {self}  ")
            //text color of not selected tab
            //.fg(self.palette().c50)
            .fg(Color::Magenta)
            //color of bckgrnd of not selected tab
            .bg(Color::Reset)
            .into()
    }

    fn render_tab0(self, area: Rect, buf: &mut Buffer) {

        //Paragraph::new("render_tab0:Hello, World!")
        //    .block(self.block())
        //    .render(area, buf);
    }

    fn render_tab1(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("render_tab1:Welcome to the Ratatui tabs example!")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab2(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("render_tab2:Look! I'm different than others!")
            .block(self.block())
            .render(area, buf);
    }

    fn render_tab3(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(
            "render_tab3:I know, these are some basic changes. But I think you got the main idea.",
        )
        .block(self.block())
        .render(area, buf);
    }

    /// A block surrounding the tab's content
    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c900)
    }
    /// <https://docs.rs/ratatui/latest/ratatui/style/palette/tailwind/index.html>
    /// <https://docs.rs/ratatui/latest/src/ratatui/style/palette/tailwind.rs.html#594-606>
    /// Magnolia
    /// pub const PURPLE: Palette = Palette {
    ///     c50: Color::from_u32(0xfaf5ff),  //https://www.htmlcsscolor.com/hex/FAF5FF
    ///     c100: Color::from_u32(0xf3e8ff),
    ///     c200: Color::from_u32(0xe9d5ff),
    ///     c300: Color::from_u32(0xd8b4fe),
    ///     c400: Color::from_u32(0xc084fc),
    ///     c500: Color::from_u32(0xa855f7),
    ///     c600: Color::from_u32(0x9333ea),
    ///     c700: Color::from_u32(0x7e22ce),
    ///     c800: Color::from_u32(0x6b21a8),
    ///     c900: Color::from_u32(0x581c87),
    ///     c950: Color::from_u32(0x3b0764),
    /// };
    const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Tab1 => tailwind::PURPLE,
            Self::Tab2 => tailwind::PURPLE,
            Self::Tab3 => tailwind::PURPLE,
            Self::Tab4 => tailwind::PURPLE,
        }
    }
}

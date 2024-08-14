use std::io::stdout;

use ratatui::{prelude::CrosstermBackend, Terminal};

fn main() {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    let alt = alternate_screen_wrapper::AlternateScreen::enter().unwrap();
    let input: s_text_input_f::Blocks = vec![s_text_input_f::Block::Paragraph(vec![
        s_text_input_f::ParagraphItem::Text("head ".into()),
        s_text_input_f::ParagraphItem::Placeholder,
        s_text_input_f::ParagraphItem::Text(" tail ".into()),
        s_text_input_f::ParagraphItem::Placeholder,
    ])];
    let a = ratatui_inputs::get_input(input, &mut |text| {
        terminal
            .draw(|f| f.render_widget(ratatui::widgets::Paragraph::new(text), f.area()))
            .map(|_| ())
    })
    .unwrap()
    .unwrap();

    drop(alt);
    dbg!(a);
}

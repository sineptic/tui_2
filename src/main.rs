use std::io::stdout;

use ratatui::{prelude::CrosstermBackend, Terminal};
use s_text_input_f::Block;
use s_text_input_f_parser::CorrectParagraph;

fn main() {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    let alt = alternate_screen_wrapper::AlternateScreen::enter().unwrap();
    let CorrectParagraph {
        input: first_paragraph,
        answer: first_answer,
    } = s_text_input_f_parser::parse_paragraph("1 start  1 finish").unwrap();
    let CorrectParagraph {
        input: second_paragraph,
        answer: second_answer,
    } = s_text_input_f_parser::parse_paragraph("2 start `` 2.1 finish `b` 2.2 finish").unwrap();
    let input_blocks: s_text_input_f::Blocks = vec![
        s_text_input_f::Block::Paragraph(first_paragraph),
        s_text_input_f::Block::Paragraph(second_paragraph),
        s_text_input_f::Block::OneOf(vec![
            "first variant".to_string(),
            "second variant".to_string(),
            "third variant".to_string(),
        ]),
    ];
    let (result_1, user_answer) = ratatui_inputs::get_input(input_blocks.clone(), &mut |text| {
        terminal
            .draw(|f| f.render_widget(ratatui::widgets::Paragraph::new(text), f.area()))
            .map(|_| ())
    })
    .transpose()
    .unwrap()
    .unwrap();
    let correct_answer = vec![first_answer, second_answer, vec!["1".to_string()]];
    let mut b = s_text_input_f::to_asnwered(input_blocks, user_answer, correct_answer)
        .into_iter()
        .map(Block::Answered)
        .collect::<Vec<_>>();
    b.push(s_text_input_f::Block::Paragraph(vec![
        s_text_input_f::ParagraphItem::Placeholder,
    ]));

    ratatui_inputs::get_input(b, &mut |text| {
        terminal
            .draw(|f| f.render_widget(ratatui::widgets::Paragraph::new(text), f.area()))
            .map(|_| ())
    })
    .transpose()
    .unwrap();

    drop(alt);
    dbg!(result_1);
}

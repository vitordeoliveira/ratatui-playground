use ratatui::{style::Stylize, widgets::Paragraph, Frame};

pub fn ui(f: &mut Frame) {
    let area = f.size();
    f.render_widget(
        Paragraph::new("Hello Ratatui! (press 'q' to quit)")
            .white()
            .on_blue(),
        area,
    );
}

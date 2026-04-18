use crate::app::{App, AppState};
use crate::models::task::Task;
use ratatui::prelude::CrosstermBackend;
use ratatui::{
    Terminal,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::Paragraph,
    widgets::{Block, Borders, Cell, Row, Table},
};

use std::io;

pub fn make_rows(tasks: &Vec<Task>) -> Vec<Row<'static>> {
    let mut rows = Vec::new();
    for (i, task) in tasks.iter().enumerate() {
        let status: String;
        let color: Color;
        if task.status {
            status = String::from("Done");
            color = Color::Green;
        } else {
            status = String::from("Note done");
            color = Color::Red;
        }
        rows.push(Row::new(vec![
            Cell::from(i.to_string()),
            Cell::from(task.name.clone()),
            Cell::from(status).style(Style::default().fg(color)),
        ]));
    }
    rows
}

pub fn draw(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) {
    let _ = terminal.draw(|f| {
        let size = f.size();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(70),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
            ])
            .split(size);

        let table = Table::new(
            make_rows(&mut app.tasks),
            [
                Constraint::Length(10),
                Constraint::Length(70),
                Constraint::Length(20),
            ],
        )
        .header(
            Row::new(vec!["ID", "Name", "Status"]).style(ratatui::style::Style::default().bold()),
        )
        .block(Block::default().title("TODO List").borders(Borders::ALL))
        .widths(&[
            Constraint::Length(10),
            Constraint::Length(70),
            Constraint::Length(20),
        ]);

        f.render_widget(table, layout[0]);

        //render input
        let render_input = match app.state {
            AppState::Notshing => None,
            AppState::AddNewTask => Some("Add Task"),
            AppState::Complete => Some("Complete Task"),
            AppState::Remove => Some("Remove Task"),
        };

        if let Some(title) = render_input {
            let widget = Paragraph::new(app.input.clone())
                .block(Block::default().title(title).borders(Borders::ALL));

            f.render_widget(widget, layout[1]);
        }

        //controlls
        let text =
            format!("[Ctrl+q] quit | [Ctrl+a] add new | [Ctrl+r] remove | [Ctrl + c] complete");

        let paragraph =
            Paragraph::new(text).block(Block::default().title("Controll").borders(Borders::ALL));

        f.render_widget(paragraph, layout[2]);
    });
}

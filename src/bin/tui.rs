use advent_of_code_2025::*;
use std::{io, time::Instant};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Padding, Paragraph},
};

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(draw)?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break Ok(());
            }
        }
    }
}

fn draw(frame: &mut Frame) {
    let main_layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ]);
    let block_layout = Layout::vertical([Constraint::Length(20); 3]);
    let [title_area, main_area, footer_area] = main_layout.areas(frame.area());
    let areas: Vec<Vec<Rect>> = block_layout
        .split(main_area)
        .iter()
        .map(|&area| {
            Layout::horizontal([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area)
            .to_vec()
        })
        .collect();
    frame.render_widget(
        Paragraph::new("Advent of Code 2025").alignment(ratatui::layout::Alignment::Center),
        title_area,
    );
    frame.render_widget(
        Paragraph::new("https://github.com/wjholden/Advent-of-Code-2025")
            .alignment(ratatui::layout::Alignment::Center),
        footer_area,
    );
    let solutions = solutions();
    let mut day = 0;
    for row in 0..3 {
        for col in 0..4 {
            let message = match solutions.get(day) {
                Some((part1, part2, duration)) => {
                    format!("Part 1: {part1}\nPart 2: {part2}\n{duration:?}")
                }
                None => "Coming soon!".to_owned(),
            };
            day += 1;
            frame.render_widget(
                Paragraph::new(message).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .padding(Padding::new(1, 1, 1, 1))
                        .title(format!("Day {day}")),
                ),
                areas[row][col],
            );
        }
    }
}

fn solutions() -> Vec<(usize, usize, std::time::Duration)> {
    vec![
        {
            let start = Instant::now();
            let d = day01::Puzzle::new(day01::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day02::Puzzle::new(day02::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day03::Puzzle::new(day03::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day04::Puzzle::new(day04::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day05::Puzzle::new(day05::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day06::Puzzle::new(day06::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day07::Puzzle::new(day07::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day08::Puzzle::new(day08::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
    ]
}

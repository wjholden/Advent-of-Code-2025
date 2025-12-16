use advent_of_code_2025::*;
use std::{
    io,
    time::{Duration, Instant},
};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Padding, Paragraph},
};

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    println!("Solving all 12 days...");
    let solutions = solutions();
    terminal.clear()?;
    loop {
        terminal.draw(|frame| draw(frame, &solutions))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break Ok(());
            }
        }
    }
}

fn draw(frame: &mut Frame, solutions: &[(usize, usize, Duration)]) {
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
        Paragraph::new("Advent of Code 2025")
            .alignment(ratatui::layout::Alignment::Center)
            .style(
                Style::default()
                    .fg(ratatui::style::Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        title_area,
    );
    frame.render_widget(
        Paragraph::new("https://github.com/wjholden/Advent-of-Code-2025")
            .alignment(ratatui::layout::Alignment::Center),
        footer_area,
    );
    let mut day = 0;
    for row in 0..3 {
        for col in 0..4 {
            let message = match solutions.get(day) {
                Some((part1, 0, duration)) => {
                    format!("Part 1:  {part1}\nPart 2:  Merry Christmas!\n{duration:?}")
                }
                Some((part1, part2, duration)) => {
                    format!("Part 1:  {part1}\nPart 2:  {part2}\nRuntime: {duration:?}")
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
        {
            let start = Instant::now();
            let d = day09::Puzzle::new(day09::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day10::Puzzle::new(day10::PUZZLE).solve();
            (d.part1 as usize, d.part2 as usize, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day11::Puzzle::new(day11::PUZZLE).solve();
            (d.part1, d.part2, start.elapsed())
        },
        {
            let start = Instant::now();
            let d = day12::Puzzle::new(day12::PUZZLE).solve();
            (d.part1, 0, start.elapsed())
        },
    ]
}

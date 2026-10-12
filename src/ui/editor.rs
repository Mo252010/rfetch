use crate::config::{self, Config, Module, ModuleType};
use crate::theme;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::{DefaultTerminal, Frame};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    General,
    Theme,
    Modules,
}

impl Tab {
    const ALL: [Tab; 3] = [Tab::General, Tab::Theme, Tab::Modules];
    fn title(&self) -> &'static str {
        match self {
            Tab::General => " General ",
            Tab::Theme => " Theme ",
            Tab::Modules => " Modules ",
        }
    }
}

enum Mode {
    Normal,
    Input(InputKind),
    Picker,
}

#[derive(Clone, Copy)]
enum InputKind {
    Title,
    ModuleLabel,
    ModuleMounts,
}

struct Editor {
    config: Config,
    tab: Tab,
    general_idx: usize,
    theme_idx: usize,
    module_idx: usize,
    mode: Mode,
    input: String,
    picker_idx: usize,
    dirty: bool,
    status: String,
}

const GENERAL_FIELDS: usize = 5;
const THEME_FIELDS: usize = 5;

impl Editor {
    fn new(config: Config) -> Self {
        Self {
            config,
            tab: Tab::General,
            general_idx: 0,
            theme_idx: 0,
            module_idx: 0,
            mode: Mode::Normal,
            input: String::new(),
            picker_idx: 1,
            dirty: false,
            status: "arrows navigate \u{2022} space toggles \u{2022} s saves \u{2022} q quits".into(),
        }
    }

    fn touch(&mut self, msg: &str) {
        self.dirty = true;
        self.status = msg.to_string();
    }
}

pub fn run(terminal: &mut DefaultTerminal, base: &Config) -> Result<()> {
    let mut ed = Editor::new(base.clone());
    loop {
        terminal.draw(|frame| draw(frame, &ed))?;
        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        if handle_key(&mut ed, key.code, key.modifiers) {
            return Ok(());
        }
    }
}

fn handle_key(ed: &mut Editor, code: KeyCode, modifiers: KeyModifiers) -> bool {
    if let Mode::Input(kind) = ed.mode {
        return handle_input(ed, kind, code);
    }
    if matches!(ed.mode, Mode::Picker) {
        return handle_picker(ed, code);
    }

    match code {
        KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => return true,
        KeyCode::Char('q') => return true,
        KeyCode::Tab => next_tab(ed, true),
        KeyCode::BackTab => next_tab(ed, false),
        KeyCode::Char('1') => ed.tab = Tab::General,
        KeyCode::Char('2') => ed.tab = Tab::Theme,
        KeyCode::Char('3') => ed.tab = Tab::Modules,
        KeyCode::Char('s') => do_save(ed),
        KeyCode::Char('r') => {
            ed.config = Config::default();
            ed.touch("reset to defaults");
        }
        KeyCode::Up => move_sel(ed, -1),
        KeyCode::Down => move_sel(ed, 1),
        KeyCode::Left => adjust(ed, false),
        KeyCode::Right => adjust(ed, true),
        KeyCode::Enter | KeyCode::Char(' ') => activate(ed),
        KeyCode::Char('a') if ed.tab == Tab::Modules => {
            ed.mode = Mode::Picker;
            ed.picker_idx = 1;
        }
        KeyCode::Char('d') if ed.tab == Tab::Modules => delete_module(ed),
        KeyCode::Char('K') if ed.tab == Tab::Modules => move_module(ed, -1),
        KeyCode::Char('J') if ed.tab == Tab::Modules => move_module(ed, 1),
        KeyCode::Char('n') if ed.tab == Tab::Modules => begin_input(ed, InputKind::ModuleLabel),
        KeyCode::Char('m') if ed.tab == Tab::Modules => begin_input(ed, InputKind::ModuleMounts),
        _ => {}
    }
    false
}

fn do_save(ed: &mut Editor) {
    match config::save(&ed.config) {
        Ok(()) => {
            ed.dirty = false;
            ed.status = format!("saved to {}", config::config_path().display());
        }
        Err(e) => ed.status = format!("save failed: {e}"),
    }
}

fn next_tab(ed: &mut Editor, forward: bool) {
    let i = Tab::ALL.iter().position(|t| *t == ed.tab).unwrap_or(0);
    let n = Tab::ALL.len();
    ed.tab = Tab::ALL[if forward { (i + 1) % n } else { (i + n - 1) % n }];
}

fn move_sel(ed: &mut Editor, delta: isize) {
    match ed.tab {
        Tab::General => ed.general_idx = wrap_add(ed.general_idx, delta, GENERAL_FIELDS),
        Tab::Theme => ed.theme_idx = wrap_add(ed.theme_idx, delta, THEME_FIELDS),
        Tab::Modules => {
            if !ed.config.modules.is_empty() {
                ed.module_idx = wrap_add(ed.module_idx, delta, ed.config.modules.len());
            }
        }
    }
}

fn wrap_add(idx: usize, delta: isize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    ((idx as isize + delta).rem_euclid(len as isize)) as usize
}

fn adjust(ed: &mut Editor, forward: bool) {
    match ed.tab {
        Tab::General => adjust_general(ed, forward),
        Tab::Theme => adjust_theme(ed, forward),
        Tab::Modules => adjust_module_width(ed, forward),
    }
}

fn adjust_general(ed: &mut Editor, forward: bool) {
    match ed.general_idx {
        0 => {
            let step: i64 = 100;
            let v = ed.config.general.refresh_ms as i64 + if forward { step } else { -step };
            ed.config.general.refresh_ms = v.max(100) as u64;
            ed.touch("refresh interval changed");
        }
        1 => {
            ed.config.general.border = !ed.config.general.border;
            ed.touch("border toggled");
        }
        2 => {
            ed.config.general.rounded = !ed.config.general.rounded;
            ed.touch("rounded toggled");
        }
        3 => {
            ed.config.general.show_logo = !ed.config.general.show_logo;
            ed.touch("logo toggled");
        }
        _ => {}
    }
}

fn adjust_theme(ed: &mut Editor, forward: bool) {
    let field = match ed.theme_idx {
        0 => &mut ed.config.theme.accent,
        1 => &mut ed.config.theme.label,
        2 => &mut ed.config.theme.value,
        3 => &mut ed.config.theme.title,
        _ => &mut ed.config.theme.border,
    };
    *field = theme::cycle(field, forward);
    ed.touch("theme color changed");
}

fn adjust_module_width(ed: &mut Editor, forward: bool) {
    if let Some(m) = ed.config.modules.get_mut(ed.module_idx) {
        let w = m.bar_width as i64 + if forward { 2 } else { -2 };
        m.bar_width = w.clamp(4, 80) as u16;
        ed.touch("bar width changed");
    }
}

fn activate(ed: &mut Editor) {
    match ed.tab {
        Tab::General => match ed.general_idx {
            1 => {
                ed.config.general.border = !ed.config.general.border;
                ed.touch("border toggled");
            }
            2 => {
                ed.config.general.rounded = !ed.config.general.rounded;
                ed.touch("rounded toggled");
            }
            3 => {
                ed.config.general.show_logo = !ed.config.general.show_logo;
                ed.touch("logo toggled");
            }
            4 => begin_input(ed, InputKind::Title),
            _ => {}
        },
        Tab::Theme => adjust_theme(ed, true),
        Tab::Modules => {
            if let Some(m) = ed.config.modules.get_mut(ed.module_idx) {
                m.enabled = !m.enabled;
                ed.touch("module toggled");
            }
        }
    }
}

fn delete_module(ed: &mut Editor) {
    if ed.module_idx < ed.config.modules.len() {
        ed.config.modules.remove(ed.module_idx);
        if ed.module_idx >= ed.config.modules.len() && ed.module_idx > 0 {
            ed.module_idx -= 1;
        }
        ed.touch("module removed");
    }
}

fn move_module(ed: &mut Editor, delta: isize) {
    let len = ed.config.modules.len();
    if len < 2 {
        return;
    }
    let target = wrap_add(ed.module_idx, delta, len);
    if target != ed.module_idx {
        ed.config.modules.swap(ed.module_idx, target);
        ed.module_idx = target;
        ed.touch("module moved");
    }
}

fn begin_input(ed: &mut Editor, kind: InputKind) {
    ed.input = match kind {
        InputKind::Title => ed.config.general.title.clone(),
        InputKind::ModuleLabel => ed
            .config
            .modules
            .get(ed.module_idx)
            .and_then(|m| m.label.clone())
            .unwrap_or_default(),
        InputKind::ModuleMounts => ed
            .config
            .modules
            .get(ed.module_idx)
            .map(|m| m.mounts.join(","))
            .unwrap_or_default(),
    };
    ed.mode = Mode::Input(kind);
    ed.status = "editing \u{2022} enter confirms \u{2022} esc cancels \u{2022} ctrl+u clears".into();
}

fn handle_input(ed: &mut Editor, kind: InputKind, code: KeyCode) -> bool {
    match code {
        KeyCode::Esc => {
            ed.mode = Mode::Normal;
            ed.status = "edit cancelled".into();
        }
        KeyCode::Enter => {
            commit_input(ed, kind);
            ed.mode = Mode::Normal;
        }
        KeyCode::Backspace => {
            ed.input.pop();
        }
        KeyCode::Char(c) => ed.input.push(c),
        _ => {}
    }
    false
}

fn commit_input(ed: &mut Editor, kind: InputKind) {
    match kind {
        InputKind::Title => {
            ed.config.general.title = ed.input.trim().to_string();
            ed.touch("title updated");
        }
        InputKind::ModuleLabel => {
            if let Some(m) = ed.config.modules.get_mut(ed.module_idx) {
                let t = ed.input.trim();
                m.label = if t.is_empty() { None } else { Some(t.to_string()) };
            }
            ed.touch("label updated");
        }
        InputKind::ModuleMounts => {
            if let Some(m) = ed.config.modules.get_mut(ed.module_idx) {
                m.mounts = ed
                    .input
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            ed.touch("mounts updated");
        }
    }
}

fn handle_picker(ed: &mut Editor, code: KeyCode) -> bool {
    let kinds = ModuleType::ALL;
    match code {
        KeyCode::Esc => {
            ed.mode = Mode::Normal;
        }
        KeyCode::Up => ed.picker_idx = wrap_add(ed.picker_idx, -1, kinds.len()),
        KeyCode::Down => ed.picker_idx = wrap_add(ed.picker_idx, 1, kinds.len()),
        KeyCode::Enter => {
            let kind = kinds[ed.picker_idx];
            ed.config.modules.push(Module::new(kind));
            ed.module_idx = ed.config.modules.len() - 1;
            ed.mode = Mode::Normal;
            ed.touch("module added");
        }
        _ => {}
    }
    false
}

fn draw(frame: &mut Frame, ed: &Editor) {
    let area = frame.area();
    let accent = theme::parse(&ed.config.theme.accent);
    let dim = theme::parse(&ed.config.theme.border);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(2),
        ])
        .split(area);

    let dirty = if ed.dirty { " \u{25cf} unsaved" } else { "" };
    let title = format!(" rfetch config editor{} ", dirty);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(dim))
        .title(Span::styled(title, Style::default().fg(accent)));
    frame.render_widget(block, chunks[0]);

    let tab_line = build_tabs(ed, accent, dim);
    frame.render_widget(
        Paragraph::new(tab_line),
        chunks[0].inner(Margin {
            vertical: 0,
            horizontal: 1,
        }),
    );

    match ed.tab {
        Tab::General => draw_general(frame, ed, chunks[1], accent, dim),
        Tab::Theme => draw_theme(frame, ed, chunks[1], accent, dim),
        Tab::Modules => draw_modules(frame, ed, chunks[1], accent, dim),
    }

    let help = if matches!(ed.mode, Mode::Input(_)) {
        format!(" {} ", ed.input_display())
    } else {
        format!(" {} ", ed.status)
    };
    let help_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(dim))
        .title(Span::styled(" status ", Style::default().fg(dim)));
    frame.render_widget(
        Paragraph::new(help).style(Style::default().fg(Color::Gray)).block(help_block),
        chunks[2],
    );

    if matches!(ed.mode, Mode::Picker) {
        draw_picker(frame, ed, area, accent, dim);
    }
}

impl Editor {
    fn input_display(&self) -> String {
        format!("edit: {}\u{2588}", self.input)
    }
}

fn build_tabs(ed: &Editor, accent: Color, dim: Color) -> Line<'static> {
    let mut spans: Vec<Span> = Vec::new();
    for tab in Tab::ALL.iter() {
        let active = *tab == ed.tab;
        let style = if active {
            Style::default().fg(Color::Black).bg(accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(dim)
        };
        spans.push(Span::styled(tab.title().to_string(), style));
        spans.push(Span::raw(" "));
    }
    Line::from(spans)
}

fn field_line(selected: bool, label: &str, value: String, value_color: Color, accent: Color) -> Line<'static> {
    let marker = if selected { "\u{25b6} " } else { "  " };
    let label_style = if selected {
        Style::default().fg(accent).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };
    let row_style = if selected {
        Style::default().bg(Color::Rgb(30, 30, 40))
    } else {
        Style::default()
    };
    Line::from(vec![
        Span::styled(marker.to_string(), label_style),
        Span::styled(format!("{:<16}", label), label_style),
        Span::styled(value, Style::default().fg(value_color)),
    ])
    .style(row_style)
}

fn draw_general(frame: &mut Frame, ed: &Editor, area: Rect, accent: Color, dim: Color) {
    let g = &ed.config.general;
    let rows: Vec<(String, Color)> = vec![
        (format!("{} ms", g.refresh_ms), Color::White),
        (on_off(g.border), bool_color(g.border)),
        (on_off(g.rounded), bool_color(g.rounded)),
        (on_off(g.show_logo), bool_color(g.show_logo)),
        (
            if g.title.is_empty() {
                "<none>".into()
            } else {
                g.title.clone()
            },
            Color::White,
        ),
    ];
    let labels = ["Refresh", "Border", "Rounded", "Logo", "Title"];
    let mut lines = Vec::new();
    for (i, (label, (value, color))) in labels.iter().zip(rows).enumerate() {
        lines.push(field_line(ed.general_idx == i, label, value, color, accent));
    }
    frame.render_widget(
        Paragraph::new(lines).block(section_block(" general ", dim)),
        area,
    );
}

fn draw_theme(frame: &mut Frame, ed: &Editor, area: Rect, accent: Color, dim: Color) {
    let t = &ed.config.theme;
    let rows = [
        ("Accent", &t.accent),
        ("Label", &t.label),
        ("Value", &t.value),
        ("Title", &t.title),
        ("Border", &t.border),
    ];
    let mut lines = Vec::new();
    for (i, (label, value)) in rows.iter().enumerate() {
        let swatch = format!("  \u{2588}\u{2588}  {}", value);
        lines.push(field_line(
            ed.theme_idx == i,
            label,
            swatch,
            theme::parse(value),
            accent,
        ));
    }
    frame.render_widget(
        Paragraph::new(lines).block(section_block(" left/right cycles palette ", dim)),
        area,
    );
}

fn draw_modules(frame: &mut Frame, ed: &Editor, area: Rect, accent: Color, dim: Color) {
    if ed.config.modules.is_empty() {
        frame.render_widget(
            Paragraph::new("  no modules \u{2022} press 'a' to add")
                .style(Style::default().fg(Color::Gray))
                .block(section_block(" modules ", dim)),
            area,
        );
        return;
    }
    let mut lines = Vec::new();
    for (i, m) in ed.config.modules.iter().enumerate() {
        let selected = i == ed.module_idx;
        let check = if m.enabled { "[\u{2713}]" } else { "[ ]" };
        let check_style = if m.enabled {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        let label = m.label.clone().unwrap_or_else(|| "-".into());
        let mounts = if m.mounts.is_empty() {
            String::new()
        } else {
            format!("  mounts:{}", m.mounts.join(","))
        };
        let marker = if selected { "\u{25b6} " } else { "  " };
        let name_style = if selected {
            Style::default().fg(accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        let row_style = if selected {
            Style::default().bg(Color::Rgb(30, 30, 40))
        } else {
            Style::default()
        };
        lines.push(
            Line::from(vec![
                Span::styled(marker.to_string(), name_style),
                Span::styled(format!("{} ", check), check_style),
                Span::styled(format!("{:<9}", m.kind.as_str()), name_style),
                Span::styled(format!("width {:<3}", m.bar_width), Style::default().fg(Color::DarkGray)),
                Span::styled(format!("  label:{}", label), Style::default().fg(Color::White)),
                Span::styled(mounts, Style::default().fg(Color::DarkGray)),
            ])
            .style(row_style),
        );
    }
    frame.render_widget(
        Paragraph::new(lines).block(section_block(" a add  d delete  space toggle  J/K move  n label  m mounts ", dim)),
        area,
    );
}

fn draw_picker(frame: &mut Frame, ed: &Editor, area: Rect, accent: Color, dim: Color) {
    let width = 30.min(area.width);
    let height = (ModuleType::ALL.len() as u16 + 2).min(area.height);
    let popup = centered_rect(width, height, area);
    frame.render_widget(Clear, popup);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(accent))
        .title(Span::styled(" add module ", Style::default().fg(accent)));

    let mut lines = Vec::new();
    for (i, kind) in ModuleType::ALL.iter().enumerate() {
        let selected = i == ed.picker_idx;
        let style = if selected {
            Style::default().fg(Color::Black).bg(accent)
        } else {
            Style::default().fg(Color::Gray)
        };
        lines.push(Line::from(Span::styled(format!(" {} ", kind.as_str()), style)));
    }
    frame.render_widget(Paragraph::new(lines).block(block), popup);
    let _ = dim;
}

fn section_block(title: &str, dim: Color) -> Block<'static> {
    Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(dim))
        .title(Span::styled(format!(" {} ", title.trim()), Style::default().fg(dim)))
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect {
        x,
        y,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}

fn on_off(v: bool) -> String {
    if v { "[on]".into() } else { "[off]".into() }
}

fn bool_color(v: bool) -> Color {
    if v { Color::Green } else { Color::DarkGray }
}

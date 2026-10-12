use crate::collector::{Collector, Snapshot};
use crate::config::{Config, Module, ModuleType};
use crate::logo;
use crate::theme;
use crate::ui::widgets as w;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use ratatui::DefaultTerminal;
use std::time::Duration;

pub enum Action {
    Quit,
    OpenEditor,
}

pub fn run(terminal: &mut DefaultTerminal, config: &Config, collector: &mut Collector) -> Result<Action> {
    let interval = Duration::from_millis(config.general.refresh_ms.max(100));
    loop {
        let snapshot = collector.refresh();
        terminal.draw(|frame| render(frame, config, &snapshot))?;

        if event::poll(interval)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(Action::Quit),
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(Action::Quit);
                }
                KeyCode::Char('e') => return Ok(Action::OpenEditor),
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame, config: &Config, snap: &Snapshot) {
    let area = frame.area();
    let accent = theme::parse(&config.theme.accent);
    let border_color = theme::parse(&config.theme.border);

    let art = logo::ascii();
    let (logo_area, info_area) = if config.general.show_logo && area.width > 40 {
        let logo_w = art.iter().map(|l| l.chars().count()).max().unwrap_or(20) as u16 + 2;
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(logo_w), Constraint::Min(20)])
            .split(area);
        (Some(chunks[0]), chunks[1])
    } else {
        (None, area)
    };

    if let Some(logo_area) = logo_area {
        let logo = art
            .iter()
            .map(|l| Line::from(Span::styled(*l, Style::default().fg(accent))))
            .collect::<Vec<_>>();
        let para = Paragraph::new(logo).wrap(Wrap { trim: false });
        frame.render_widget(para, center_vertically(logo_area, art.len() as u16));
    }

    let mut block = Block::default().borders(if config.general.border {
        Borders::ALL
    } else {
        Borders::NONE
    });
    if config.general.rounded {
        block = block.border_type(BorderType::Rounded);
    }
    block = block.border_style(Style::default().fg(border_color));
    let title = if config.general.title.is_empty() {
        " rfetch ".to_string()
    } else {
        format!(" {} ", config.general.title)
    };
    block = block.title(Span::styled(title, Style::default().fg(theme::parse(&config.theme.title))));

    let inner = block.inner(info_area);
    let lines = build_lines(config, snap, inner.width);
    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(para, info_area);
}

/// Number of terminal rows the rendered view needs, so the live view can be
/// shown in an inline viewport just tall enough for its content.
pub fn content_height(config: &Config, snap: &Snapshot, width: u16) -> u16 {
    let border = if config.general.border { 2 } else { 0 };
    let inner_w = width.saturating_sub(border).max(20);
    let info = build_lines(config, snap, inner_w).len() as u16;
    let logo = if config.general.show_logo && width > 40 {
        logo::ascii().len() as u16
    } else {
        0
    };
    info.max(logo) + border
}

pub fn plain_text(config: &Config, snap: &Snapshot) -> String {
    let mut out = String::new();
    if config.general.show_logo {
        for line in logo::ascii() {
            out.push_str(line.trim_end());
            out.push('\n');
        }
        out.push('\n');
    }
    for line in build_lines(config, snap, 80) {
        let text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
        out.push_str(text.trim_end());
        out.push('\n');
    }
    out
}

fn center_vertically(area: Rect, height: u16) -> Rect {
    if area.height <= height {
        return area;
    }
    let y = area.y + (area.height - height) / 2;
    Rect {
        x: area.x,
        y,
        width: area.width,
        height,
    }
}

fn build_lines(config: &Config, snap: &Snapshot, width: u16) -> Vec<Line<'static>> {
    let label_c = theme::parse(&config.theme.label);
    let value_c = theme::parse(&config.theme.value);
    let accent = theme::parse(&config.theme.accent);
    let dim = theme::parse(&config.theme.border);

    let label_style = w::key_style(label_c);
    let value_style = w::value_style(value_c);
    let accent_style = w::accent_style(accent);
    let filled = Style::default().fg(accent);
    let empty = Style::default().fg(dim);

    let key_w = 9usize;
    let mut lines: Vec<Line<'static>> = Vec::new();
    let avail = width.saturating_sub(key_w as u16 + 3).max(10) as usize;

    let keyed = |key: &str| Span::styled(format!("{:<width$}", key, width = key_w), label_style);

    for module in &config.modules {
        if !module.enabled {
            continue;
        }
        let bar_w = (module.bar_width as usize).min(avail);
        match module.kind {
            ModuleType::Break => lines.push(Line::from("")),
            ModuleType::Header => {
                let user = Span::styled(snap.username.clone(), label_style);
                let at = Span::styled("@", value_style);
                let host = Span::styled(snap.hostname.clone(), accent_style);
                lines.push(Line::from(vec![user, at, host]));
                let rule = "\u{2500}".repeat(avail.min(40));
                lines.push(Line::from(Span::styled(rule, Style::default().fg(dim))));
                lines.push(Line::from(""));
            }
            ModuleType::Cpu => {
                let freq = if snap.cpu.freq_mhz > 0 {
                    format!(" @ {:.2} GHz", snap.cpu.freq_mhz as f64 / 1000.0)
                } else {
                    String::new()
                };
                lines.push(Line::from(vec![
                    keyed(&module.label_or("CPU")),
                    Span::styled(
                        format!("{} ({} cores){}", snap.cpu.brand, snap.cpu.cores, freq),
                        value_style,
                    ),
                ]));
                let mut spans = w::gauge(snap.cpu.usage as f64 / 100.0, bar_w, filled, empty);
                spans.push(Span::styled(
                    format!(" {:>5.1}%", snap.cpu.usage),
                    accent_style,
                ));
                lines.push(Line::from(
                    std::iter::once(keyed("")).chain(spans).collect::<Vec<_>>(),
                ));
                if snap.cpu.per_core.len() > 1 {
                    let spark = w::sparkline(&snap.cpu.per_core, avail);
                    lines.push(Line::from(vec![
                        keyed("cores"),
                        Span::styled(spark, Style::default().fg(accent)),
                    ]));
                }
            }
            ModuleType::Memory => {
                let frac = ratio(snap.mem.used, snap.mem.total);
                let mut spans = w::gauge(frac, bar_w, filled, empty);
                spans.push(Span::styled(
                    format!(
                        " {} / {} ({:.0}%)",
                        w::format_bytes(snap.mem.used),
                        w::format_bytes(snap.mem.total),
                        frac * 100.0
                    ),
                    value_style,
                ));
                lines.push(Line::from(
                    std::iter::once(keyed(&module.label_or("Memory")))
                        .chain(spans)
                        .collect::<Vec<_>>(),
                ));
            }
            ModuleType::Swap => {
                if snap.mem.swap_total == 0 {
                    continue;
                }
                let frac = ratio(snap.mem.swap_used, snap.mem.swap_total);
                let mut spans = w::gauge(frac, bar_w, filled, empty);
                spans.push(Span::styled(
                    format!(
                        " {} / {} ({:.0}%)",
                        w::format_bytes(snap.mem.swap_used),
                        w::format_bytes(snap.mem.swap_total),
                        frac * 100.0
                    ),
                    value_style,
                ));
                lines.push(Line::from(
                    std::iter::once(keyed(&module.label_or("Swap")))
                        .chain(spans)
                        .collect::<Vec<_>>(),
                ));
            }
            ModuleType::Gpu => {
                let label = module.label_or("GPU");
                if snap.gpus.is_empty() {
                    lines.push(Line::from(vec![
                        keyed(&label),
                        Span::styled("not detected", value_style),
                    ]));
                    continue;
                }
                for (i, gpu) in snap.gpus.iter().enumerate() {
                    let k = if i == 0 { label.clone() } else { format!("{}+{}", label, i) };
                    let mut header = vec![
                        keyed(&k),
                        Span::styled(gpu.name.clone(), value_style),
                    ];
                    if !gpu.live {
                        header.push(Span::styled("  (driver unavailable)", Style::default().fg(dim)));
                    }
                    lines.push(Line::from(header));
                    if let Some(usage) = gpu.usage {
                        let mut spans =
                            w::gauge(usage as f64 / 100.0, bar_w, filled, empty);
                        let temp = gpu
                            .temp
                            .map(|t| format!("  {}C", t))
                            .unwrap_or_default();
                        let freq = match (gpu.freq_mhz, gpu.freq_max_mhz) {
                            (Some(cur), Some(max)) if max > 0 => {
                                format!("  {:.2}/{:.2}GHz", cur as f64 / 1000.0, max as f64 / 1000.0)
                            }
                            (Some(cur), _) => format!("  {:.2}GHz", cur as f64 / 1000.0),
                            _ => String::new(),
                        };
                        spans.push(Span::styled(
                            format!(" {:>3}%{}{}", usage, temp, freq),
                            accent_style,
                        ));
                        lines.push(Line::from(
                            std::iter::once(keyed("")).chain(spans).collect::<Vec<_>>(),
                        ));
                    }
                    if let (Some(used), Some(total)) = (gpu.mem_used, gpu.mem_total) {
                        let frac = ratio(used, total);
                        let mut spans = w::gauge(frac, bar_w, filled, empty);
                        spans.push(Span::styled(
                            format!(
                                " vram {} / {}",
                                w::format_bytes(used),
                                w::format_bytes(total)
                            ),
                            value_style,
                        ));
                        lines.push(Line::from(
                            std::iter::once(keyed("")).chain(spans).collect::<Vec<_>>(),
                        ));
                    }
                }
            }
            ModuleType::Storage => {
                let label = module.label_or("Disk");
                let mut seen = std::collections::HashSet::new();
                let disks: Vec<_> = snap
                    .disks
                    .iter()
                    .filter(|d| {
                        if module.mounts.is_empty() {
                            seen.insert(d.device.clone())
                        } else {
                            mount_selected(module, &d.mount)
                        }
                    })
                    .collect();
                if disks.is_empty() {
                    lines.push(Line::from(vec![
                        keyed(&label),
                        Span::styled("none", value_style),
                    ]));
                    continue;
                }
                for d in disks {
                    let used = d.total.saturating_sub(d.available);
                    let frac = ratio(used, d.total);
                    let mut spans = w::gauge(frac, bar_w, filled, empty);
                    spans.push(Span::styled(
                        format!(
                            " {} [{}] {}/{} ({:.0}%)",
                            d.mount,
                            if d.fs.is_empty() { "?" } else { d.fs.as_str() },
                            w::format_bytes(used),
                            w::format_bytes(d.total),
                            frac * 100.0
                        ),
                        value_style,
                    ));
                    lines.push(Line::from(
                        std::iter::once(keyed(&label))
                            .chain(spans)
                            .collect::<Vec<_>>(),
                    ));
                }
            }
            ModuleType::Uptime => lines.push(Line::from(vec![
                keyed(&module.label_or("Uptime")),
                Span::styled(w::human_uptime(snap.uptime_secs), value_style),
            ])),
            ModuleType::Kernel => lines.push(Line::from(vec![
                keyed(&module.label_or("Kernel")),
                Span::styled(snap.kernel.clone(), value_style),
            ])),
            ModuleType::Os => lines.push(Line::from(vec![
                keyed(&module.label_or("OS")),
                Span::styled(snap.os.clone(), value_style),
            ])),
            ModuleType::Host => lines.push(Line::from(vec![
                keyed(&module.label_or("Host")),
                Span::styled(snap.hostname.clone(), value_style),
            ])),
            ModuleType::Load => {
                let (a, b, c) = snap.load;
                lines.push(Line::from(vec![
                    keyed(&module.label_or("Load")),
                    Span::styled(format!("{:.2} {:.2} {:.2}", a, b, c), value_style),
                ]));
            }
        }
    }
    lines
}

fn ratio(used: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        used as f64 / total as f64
    }
}

fn mount_selected(module: &Module, mount: &str) -> bool {
    if module.mounts.is_empty() {
        return true;
    }
    module
        .mounts
        .iter()
        .any(|m| mount == m || (m.ends_with('/') && mount.starts_with(m)))
}

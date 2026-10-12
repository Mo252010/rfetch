use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

pub const FILLED: char = '\u{2588}'; // full block
pub const EMPTY: char = '\u{2591}'; // light shade

const BLOCKS: [char; 9] = [' ', '\u{2581}', '\u{2582}', '\u{2583}', '\u{2584}', '\u{2585}', '\u{2586}', '\u{2587}', '\u{2588}'];

pub fn gauge(fraction: f64, width: usize, filled: Style, empty: Style) -> Vec<Span<'static>> {
    let width = width.max(1);
    let frac = fraction.clamp(0.0, 1.0);
    let fill = (frac * width as f64).round() as usize;
    let fill = fill.min(width);
    let filled_str = FILLED.to_string().repeat(fill);
    let empty_str = EMPTY.to_string().repeat(width - fill);
    vec![
        Span::styled("[", empty),
        Span::styled(filled_str, filled),
        Span::styled(empty_str, empty),
        Span::styled("]", empty),
    ]
}

pub fn sparkline(values: &[f32], max_width: usize) -> String {
    if values.is_empty() {
        return String::new();
    }
    let take = values.len().min(max_width.max(1));
    values
        .iter()
        .take(take)
        .map(|v| {
            let idx = ((v.clamp(0.0, 100.0) / 100.0) * 8.0).round() as usize;
            BLOCKS[idx.min(8)]
        })
        .collect()
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else if value >= 100.0 {
        format!("{:.0} {}", value, UNITS[unit])
    } else {
        format!("{:.1} {}", value, UNITS[unit])
    }
}

pub fn human_uptime(secs: u64) -> String {
    let days = secs / 86_400;
    let hours = (secs % 86_400) / 3_600;
    let mins = (secs % 3_600) / 60;
    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{}d", days));
    }
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    parts.push(format!("{}m", mins));
    parts.join(" ")
}

pub fn key_style(color: Color) -> Style {
    Style::default().fg(color)
}

pub fn accent_style(color: Color) -> Style {
    Style::default().fg(color).add_modifier(Modifier::BOLD)
}

pub fn value_style(color: Color) -> Style {
    Style::default().fg(color)
}

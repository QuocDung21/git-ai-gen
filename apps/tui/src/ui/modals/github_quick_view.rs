use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use rust_i18n::t;

use crate::app::App;

fn highlight_word(word: &str, theme: &crate::theme::AppTheme) -> Span<'static> {
    if word.chars().all(|c| c.is_numeric()) {
        return Span::styled(word.to_string(), Style::default().fg(theme.orange));
    }
    let keywords = [
        "fn", "let", "pub", "struct", "enum", "impl", "if", "else", "return", "class", "import",
        "const", "var", "function", "for", "while", "match", "use", "mod", "true", "false", "null",
        "nil", "static", "mut", "ref", "self", "Self", "type", "as", "break", "continue", "in",
        "loop", "where", "trait", "crate", "async", "await", "dyn",
    ];
    if keywords.contains(&word) {
        return Span::styled(
            word.to_string(),
            Style::default()
                .fg(theme.purple)
                .add_modifier(Modifier::BOLD),
        );
    }
    let first_char = word.chars().next().unwrap_or(' ');
    let is_capitalized = first_char.is_uppercase();
    let is_numeric_type = [
        "i32", "u32", "i64", "u64", "f32", "f64", "usize", "bool", "char", "str",
    ]
    .contains(&word);
    if is_capitalized || is_numeric_type {
        return Span::styled(word.to_string(), Style::default().fg(theme.yellow));
    }
    Span::styled(word.to_string(), Style::default().fg(theme.fg))
}

fn highlight_line(line: &str, theme: &crate::theme::AppTheme) -> Line<'static> {
    let mut spans = Vec::new();
    let trimmed = line.trim_start();
    if trimmed.starts_with("//")
        || trimmed.starts_with("///")
        || trimmed.starts_with("#")
        || trimmed.starts_with("/*")
        || trimmed.starts_with("* ")
    {
        return Line::from(vec![Span::styled(
            line.to_string(),
            Style::default().fg(theme.border),
        )]);
    }
    let mut current_word = String::new();
    let mut in_string = false;
    let mut string_char = '"';
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if in_string {
            current_word.push(c);
            if c == string_char && (i == 0 || chars[i - 1] != '\\') {
                in_string = false;
                spans.push(Span::styled(
                    current_word.clone(),
                    Style::default().fg(theme.green),
                ));
                current_word.clear();
            }
            i += 1;
            continue;
        }
        if c == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
            if !current_word.is_empty() {
                spans.push(Span::styled(
                    current_word.clone(),
                    Style::default().fg(theme.fg),
                ));
                current_word.clear();
            }
            let rest: String = chars[i..].iter().collect();
            spans.push(Span::styled(rest, Style::default().fg(theme.border)));
            break;
        }
        if c == '"' || c == '\'' {
            if !current_word.is_empty() {
                let word_span = highlight_word(&current_word, theme);
                spans.push(word_span);
                current_word.clear();
            }
            in_string = true;
            string_char = c;
            current_word.push(c);
            i += 1;
            continue;
        }
        if c.is_alphanumeric() || c == '_' {
            current_word.push(c);
        } else {
            if c == '(' {
                if !current_word.is_empty() {
                    spans.push(Span::styled(
                        current_word.clone(),
                        Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
                    ));
                    current_word.clear();
                }
            } else if !current_word.is_empty() {
                let word_span = highlight_word(&current_word, theme);
                spans.push(word_span);
                current_word.clear();
            }
            let style = if c == '(' || c == ')' || c == '{' || c == '}' || c == '[' || c == ']' {
                Style::default().fg(theme.yellow)
            } else if c == '='
                || c == '+'
                || c == '-'
                || c == '*'
                || c == '/'
                || c == '&'
                || c == '|'
                || c == '!'
                || c == '<'
                || c == '>'
                || c == '?'
                || c == ':'
            {
                Style::default().fg(theme.purple)
            } else {
                Style::default().fg(theme.fg)
            };
            spans.push(Span::styled(c.to_string(), style));
        }
        i += 1;
    }
    if !current_word.is_empty() {
        let word_span = highlight_word(&current_word, theme);
        spans.push(word_span);
    }
    Line::from(spans)
}

fn apply_search_highlight(
    spans: Vec<Span<'static>>,
    query: &str,
    theme: &crate::theme::AppTheme,
) -> Vec<Span<'static>> {
    if query.is_empty() {
        return spans;
    }
    let mut new_spans = Vec::new();
    let query_lower = query.to_lowercase();
    for span in spans {
        let text = span.content.to_string();
        let text_lower = text.to_lowercase();
        if text_lower.contains(&query_lower) {
            let mut temp_text = text.clone();
            let mut temp_lower = text_lower.clone();
            while let Some(match_idx) = temp_lower.find(&query_lower) {
                if match_idx > 0 {
                    new_spans.push(Span::styled(temp_text[..match_idx].to_string(), span.style));
                }
                let match_end = match_idx + query.len();
                new_spans.push(Span::styled(
                    temp_text[match_idx..match_end].to_string(),
                    Style::default()
                        .bg(theme.yellow)
                        .fg(theme.select_fg)
                        .add_modifier(Modifier::BOLD),
                ));
                temp_text = temp_text[match_end..].to_string();
                temp_lower = temp_lower[match_end..].to_string();
            }
            if !temp_text.is_empty() {
                new_spans.push(Span::styled(temp_text, span.style));
            }
        } else {
            new_spans.push(span);
        }
    }
    new_spans
}

pub fn render_github_quick_view(f: &mut Frame, app: &App, area: Rect, path: &str, name: &str) {
    let theme = app.theme();
    f.render_widget(Clear, area);

    let temp_dir = if let Some(ref dir) = app.github_temp_dir {
        dir.path()
    } else {
        return;
    };
    let file_path = temp_dir.join(path);
    let content_text = if file_path.exists() {
        std::fs::read_to_string(&file_path)
            .unwrap_or_else(|_| t!("github_file_read_error").to_string())
    } else {
        t!("github_file_not_found").to_string()
    };

    let lines_total: Vec<&str> = content_text.lines().collect();
    let total_count = lines_total.len();
    let page_size = (area.height as usize).saturating_sub(6).max(5);
    let start_idx = app
        .github_quickview_scroll
        .min(total_count.saturating_sub(page_size));
    let visible_lines = lines_total.iter().skip(start_idx).take(page_size);

    let mut content = vec![Line::from("")];

    for (idx, line) in visible_lines.enumerate() {
        let real_line_num = start_idx + idx + 1;
        let line_num_str = format!("{:>4} │ ", real_line_num);
        let expanded = line.replace('\t', "    ");
        let highlighted = highlight_line(&expanded, &theme);
        let filtered_spans =
            apply_search_highlight(highlighted.spans, &app.github_quickview_search, &theme);

        let mut spans = vec![Span::styled(
            line_num_str,
            Style::default().fg(theme.border),
        )];
        spans.extend(filtered_spans);
        content.push(Line::from(spans));
    }

    content.push(Line::from(""));
    if app.github_quickview_searching {
        content.push(Line::from(vec![
            Span::styled(
                t!("github_quickview_search_prefix").to_string(),
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                app.github_quickview_search.clone(),
                Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
            ),
            Span::styled("_", Style::default().fg(theme.green)),
        ]));
    } else {
        let mut spans = vec![Span::styled(
            t!("github_quickview_actions").to_string(),
            Style::default().fg(theme.border),
        )];
        if !app.github_quickview_search.is_empty() {
            spans.push(Span::styled(
                t!("github_quickview_clear_search").to_string(),
                Style::default().fg(theme.purple),
            ));
            spans.push(Span::styled(
                t!(
                    "github_quickview_active_search",
                    search = app.github_quickview_search
                )
                .to_string(),
                Style::default()
                    .fg(theme.green)
                    .add_modifier(Modifier::ITALIC),
            ));
        }
        content.push(Line::from(spans));
    }

    let title_text = t!(
        "github_quickview_title",
        name = name,
        start = start_idx + 1,
        end = (start_idx + page_size).min(total_count),
        total = total_count
    )
    .to_string();

    let block = Block::default()
        .title(Span::styled(
            title_text,
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.cyan))
        .border_type(BorderType::Double)
        .style(Style::default().bg(theme.bg));

    let paragraph = Paragraph::new(content)
        .alignment(ratatui::layout::Alignment::Left)
        .block(block);
    f.render_widget(paragraph, area);
}

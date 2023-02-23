use std::ops::Range;

use colored::{ColoredString, Colorize};

use crate::{file_tree::FileNode, environment::ItemId};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Level {
    Error,
    Warning,
    Info,
}

impl Level {
    fn colorize(&self, text: &str) -> ColoredString {
        match self {
            Level::Error => text.red(),
            Level::Warning => text.yellow(),
            Level::Info => text.blue(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash, Debug)]
pub struct Position {
    file_index: usize,
    start: usize,
    end: usize,
}

impl Position {
    pub fn new(file_index: usize, range: Range<usize>) -> Self {
        Self {
            file_index,
            start: range.start,
            end: range.end,
        }
    }

    pub fn file_index(&self) -> usize {
        self.file_index
    }

    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    pub fn extend(&mut self, position: Position) {
        self.start = self.start.min(position.start);
        self.end = self.end.max(position.end);
    }

    pub fn placeholder() -> Position {
        Self {
            file_index: 0,
            start: 0,
            end: 0,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Element {
    Text(String),
    GeneratedCodeBlock(String),
    SourceCodeBlock(Position),
}

fn expand_text_range_to_include_full_lines(range: Range<usize>, text: &str) -> Range<usize> {
    let mut last_line_before_dr = 0;
    for (index, byte) in text.bytes().enumerate() {
        if byte == ('\n' as u8) {
            last_line_before_dr = index + 1;
        }
        if index >= range.start {
            break;
        }
    }
    let mut first_line_after_dr = text.len();
    for (index, byte) in text.bytes().enumerate() {
        if index < range.end {
            continue;
        } else if byte == ('\r' as u8) || byte == ('\n' as u8) {
            first_line_after_dr = index;
            break;
        }
    }
    last_line_before_dr..first_line_after_dr
}

fn find_line_and_column(index: usize, text: &str) -> (usize, usize) {
    let mut line = 1;
    let mut column = 1;
    for char in (&text[..index]).chars() {
        if char == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}

impl Element {
    pub fn format_colorful(&self, level: Level, files: &FileNode) -> String {
        match self {
            Element::Text(text) => {
                let level_text = match level {
                    Level::Error => "ERROR:",
                    Level::Warning => "WARN:",
                    Level::Info => "INFO:",
                };
                format!("{} {}\n", level.colorize(level_text).bold(), text.bold())
            }
            Element::GeneratedCodeBlock(generated) => {
                let mut result = format!("{}", level.colorize(&format!("> [generated]\n")));
                for line in generated.lines() {
                    result.push_str(&format!("{}{}\n", level.colorize("| "), line));
                }
                result
            }
            Element::SourceCodeBlock(location) => {
                let (path, content) = files.get_file(location.file_index());
                let diagnostic_range = location.range();
                let expanded_range =
                    expand_text_range_to_include_full_lines(diagnostic_range.clone(), content);
                let text = &content[expanded_range.clone()];
                let (line, column) = find_line_and_column(diagnostic_range.start, content);
                let mut result = format!(
                    "{}",
                    level.colorize(&format!("> {}.sr:{}:{}\n", path, line, column))
                );
                let mut position = expanded_range.start;
                for line in text.lines() {
                    result.push_str(&format!("{}{}\n", level.colorize("| "), line));
                    let mut highlight = String::new();
                    for char in line.chars() {
                        if diagnostic_range.contains(&position) {
                            highlight.push('^');
                        } else {
                            highlight.push(' ');
                        }
                        position += char.len_utf8();
                    }
                    if highlight.contains("^") {
                        result.push_str(&format!(
                            "{}{}\n",
                            level.colorize("| "),
                            level.colorize(&highlight)
                        ));
                    }
                }
                result
            }
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Diagnostic {
    elements: Vec<(Level, Element)>,
}

impl Diagnostic {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn with_element(mut self, level: Level, element: Element) -> Self {
        self.elements.push((level, element));
        self
    }

    pub fn with_text(self, level: Level, text: String) -> Self {
        self.with_element(level, Element::Text(text.to_owned()))
    }

    pub fn with_text_info(self, text: String) -> Self {
        self.with_text(Level::Info, text)
    }

    pub fn with_text_warning(self, text: String) -> Self {
        self.with_text(Level::Warning, text)
    }

    pub fn with_text_error(self, text: String) -> Self {
        self.with_text(Level::Error, text)
    }

    pub fn with_generated_code_block(self, level: Level, generated_code_block: String) -> Self {
        self.with_element(
            level,
            Element::GeneratedCodeBlock(generated_code_block.to_owned()),
        )
    }

    pub fn with_generated_code_block_info(self, generated_code_block: String) -> Self {
        self.with_generated_code_block(Level::Info, generated_code_block)
    }

    pub fn with_generated_code_block_warning(self, generated_code_block: String) -> Self {
        self.with_generated_code_block(Level::Warning, generated_code_block)
    }

    pub fn with_generated_code_block_error(self, generated_code_block: String) -> Self {
        self.with_generated_code_block(Level::Error, generated_code_block)
    }

    pub fn with_source_code_block(
        self,
        level: Level,
        source_code_block: impl Into<Position>,
    ) -> Self {
        self.with_element(level, Element::SourceCodeBlock(source_code_block.into()))
    }

    pub fn with_source_code_block_info(self, source_code_block: impl Into<Position>) -> Self {
        self.with_source_code_block(Level::Info, source_code_block)
    }

    pub fn with_source_code_block_warning(self, source_code_block: impl Into<Position>) -> Self {
        self.with_source_code_block(Level::Warning, source_code_block)
    }

    pub fn with_source_code_block_error(self, source_code_block: impl Into<Position>) -> Self {
        self.with_source_code_block(Level::Error, source_code_block)
    }

    pub fn with_item(self, level: Level, item: &ItemId) -> Self {
        todo!()
    }

    pub fn with_item_info(self, item: &ItemId) -> Self {
        Self::with_item(self, Level::Info, item)
    }

    pub fn with_item_warning(self, item: &ItemId) -> Self {
        Self::with_item(self, Level::Warning, item)
    }

    pub fn with_item_error(self, item: &ItemId) -> Self {
        Self::with_item(self, Level::Error, item)
    }
}

impl Diagnostic {
    pub fn format_colorful(&self, files: &FileNode) -> String {
        let mut result = String::new();
        for (level, element) in &self.elements {
            result.push_str(&element.format_colorful(*level, files));
        }
        result
    }
}

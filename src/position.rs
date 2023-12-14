use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Position {
    pub line: usize,
    pub col: usize,
    pub index: usize,
}

impl Position {
    pub fn advance(&mut self, ch: char) {
        self.index += 1;
        self.col += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 0;
        }
    }

    pub fn get_lines_between_as_display(&self, other: &Self, source: &str) -> String {
        let mut lines = source
            .lines()
            .enumerate()
            .map(|(i, line)| format!("{} | {}", i + 1, line))
            .skip(self.line)
            .take(other.line - self.line + 1)
            .collect::<Vec<_>>()
            .join("\n");
        lines.push('\n');
        lines.push_str(&" ".repeat(self.col + 3));
        lines.push_str(&"^".repeat(other.col - self.col + 1));
        lines
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.line + 1, self.col + 1)
    }
}

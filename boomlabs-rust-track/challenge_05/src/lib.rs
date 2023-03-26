use std::fmt;

pub struct Room {
    pub name: String,
    pub north: String,
    pub south: String,
    pub east: String,
    pub west: String,
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_len = self.north.len().max(self.south.len()).max(self.name.len());
        let total_len: usize;
        if max_len % 2 == 0 {
            total_len = max_len + 3;
        } else {
            total_len = max_len + 2;
        }
        write!(
            f,
            "
{}[{}{}{}]
{}|
{}+{}N{}+
{} - |{}{}{}| - {}
{}+{}S{}+
{}|
{}[{}{}{}]",
            " ".repeat(self.west.len() + 3),
            " ".repeat(left_space_len(total_len, self.north.len())),
            self.north,
            " ".repeat(right_space_len(total_len, self.north.len())),
            " ".repeat(left_space_len(total_len, 1) + 1 + self.west.len() + 3),
            " ".repeat(self.west.len() + 3),
            "-".repeat(left_space_len(total_len, 1)),
            "-".repeat(right_space_len(total_len, 1)),
            self.west,
            " ".repeat(left_space_len(total_len, self.name.len())),
            self.name,
            " ".repeat(right_space_len(total_len, self.name.len())),
            self.east,
            " ".repeat(self.west.len() + 3),
            "-".repeat(left_space_len(total_len, 1)),
            "-".repeat(right_space_len(total_len, 1)),
            " ".repeat(left_space_len(total_len, 1) + 1 + self.west.len() + 3),
            " ".repeat(self.west.len() + 3),
            " ".repeat(left_space_len(total_len, self.south.len())),
            self.south,
            " ".repeat(right_space_len(total_len, self.south.len())),
        )
    }
}

fn left_space_len(total_len: usize, text_len: usize) -> usize {
    if text_len % 2 == 0 {
        (total_len - text_len - 1) / 2
    } else {
        (total_len - text_len) / 2
    }
}

fn right_space_len(total_len: usize, text_len: usize) -> usize {
    if text_len % 2 == 0 {
        (total_len - text_len + 1) / 2
    } else {
        (total_len - text_len) / 2
    }
}

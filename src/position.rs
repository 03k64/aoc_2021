#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn neighbours_all(self, height: usize, width: usize) -> Vec<Self> {
        vec![
            self.bottom_left(height),
            self.bottom_right(height, width),
            self.down(height),
            self.left(),
            self.right(width),
            self.top_left(),
            self.top_right(width),
            self.up(),
        ]
        .into_iter()
        .filter_map(|n| n)
        .collect()
    }

    pub fn neighbours_orthogonal(self, height: usize, width: usize) -> Vec<Self> {
        vec![self.down(height), self.left(), self.right(width), self.up()]
            .into_iter()
            .filter_map(|n| n)
            .collect()
    }

    fn bottom_left(self, height: usize) -> Option<Self> {
        if self.x > 0 && self.y < height - 1 {
            Some(Self {
                x: self.x - 1,
                y: self.y + 1,
            })
        } else {
            None
        }
    }

    fn bottom_right(self, height: usize, width: usize) -> Option<Self> {
        if self.x < width - 1 && self.y < height - 1 {
            Some(Self {
                x: self.x + 1,
                y: self.y + 1,
            })
        } else {
            None
        }
    }

    fn down(self, height: usize) -> Option<Self> {
        if self.y < height - 1 {
            Some(Self {
                x: self.x,
                y: self.y + 1,
            })
        } else {
            None
        }
    }

    fn left(self) -> Option<Self> {
        if self.x > 0 {
            Some(Self {
                x: self.x - 1,
                y: self.y,
            })
        } else {
            None
        }
    }

    fn right(self, width: usize) -> Option<Self> {
        if self.x < width - 1 {
            Some(Self {
                x: self.x + 1,
                y: self.y,
            })
        } else {
            None
        }
    }

    fn top_left(self) -> Option<Self> {
        if self.x > 0 && self.y > 0 {
            Some(Self {
                x: self.x - 1,
                y: self.y - 1,
            })
        } else {
            None
        }
    }

    fn top_right(self, width: usize) -> Option<Self> {
        if self.x < width - 1 && self.y > 0 {
            Some(Self {
                x: self.x + 1,
                y: self.y - 1,
            })
        } else {
            None
        }
    }

    fn up(self) -> Option<Self> {
        if self.y > 0 {
            Some(Self {
                x: self.x,
                y: self.y - 1,
            })
        } else {
            None
        }
    }
}

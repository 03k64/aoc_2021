#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn neighbours_all(self) -> Vec<Self> {
        vec![
            self.bottom_left(),
            self.bottom_right(),
            self.down(),
            self.left(),
            self.right(),
            self.top_left(),
            self.top_right(),
            self.up(),
        ]
        .into_iter()
        .filter_map(|n| n)
        .collect()
    }

    pub fn neighbours_orthogonal(self) -> Vec<Self> {
        vec![self.down(), self.left(), self.right(), self.up()]
            .into_iter()
            .filter_map(|n| n)
            .collect()
    }

    fn bottom_left(self) -> Option<Self> {
        if self.x > 0 && self.y < usize::MAX {
            Some(Self {
                x: self.x - 1,
                y: self.y + 1,
            })
        } else {
            None
        }
    }

    fn bottom_right(self) -> Option<Self> {
        if self.x < usize::MAX && self.y < usize::MAX {
            Some(Self {
                x: self.x + 1,
                y: self.y + 1,
            })
        } else {
            None
        }
    }

    fn down(self) -> Option<Self> {
        if self.y < usize::MAX {
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

    fn right(self) -> Option<Self> {
        if self.x < usize::MAX {
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

    fn top_right(self) -> Option<Self> {
        if self.x < usize::MAX && self.y > 0 {
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

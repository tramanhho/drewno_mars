use std::fmt::{Display, Formatter, Error};
use std::ops::Add;
use std::cmp::Ordering;
use std::cmp::{min, max};

pub mod node;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
	start: Position,
	end: Position
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
	row: usize,
	col: usize,
}

impl Span {
	pub fn new(start: usize, end: usize) -> Span {
		Span {
			start: Position { row: 1, col: start },
			end:   Position { row: 1, col: end   }
		}
	}
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[{}, {}]", self.row, self.col)
    }
}

impl<'a, 'b> Add<&'b Span> for &'a Span {
    type Output = Span;

    fn add(self, other: &'b Span) -> Span {
        Span {
            start: min(self.start.clone(), other.start.clone()),
			end: max(self.end.clone(), other.end.clone())
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
		if self.row.cmp(&other.row) == Ordering::Equal {
			self.col.cmp(&other.col)
		} else {
			self.row.cmp(&other.row)
		}
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && 
		self.col == other.col
    }
}

impl Eq for Position {}


impl Span {
	fn correct(&mut self, line_bytes: &Vec<usize>) {
	self.start.col += 1;
	self.end.col += 1;

	let mut line_bytes = line_bytes.into_iter();
	loop {
		let line = match line_bytes.next() {
			Some(x) => x + 2,
			None => break,
		};

		if self.start.col >= line {
			self.start.col -= line;
			self.start.row += 1;
		} else {
			()
		}

		if self.end.col >= line {
			self.end.col -= line;
			self.end.row += 1;
		} else {
			break;
		}
	}
}
}

pub fn line_bytes(input: String) -> Vec<usize> {
	let lines = input.lines();
	lines.map(|line| line.len()).collect::<Vec<usize>>()
}

mod tests {
	use super::Position;
	use std::cmp::Ordering;

	impl Position {
		pub fn test_new(row: usize, col: usize) -> Position {
			Position {
				row: row, 
				col: col 
			}
		}
	}

	struct Test {
		a: Position,
		b: Position,
		result: Ordering
	}

	#[test]
	fn position_ordering() {
		let vecs = vec![
			Test { //diff row/col
				a: Position::test_new(13,  1), 
				b: Position::test_new(10,  5),
				result: Ordering::Greater
			},
			Test { //diff row/col
				a: Position::test_new(5,  15), 
				b: Position::test_new(7,  14),
				result: Ordering::Greater
			},
			Test { //same row, diff col
				a: Position::test_new(10,  9), 
				b: Position::test_new(10,  11),
				result: Ordering::Greater
			},
			Test { //same row, diff col
				a: Position::test_new(5,  1), 
				b: Position::test_new(5,  7),
				result: Ordering::Greater
			},
			Test { //diff row, same col
				a: Position::test_new(1,  2), 
				b: Position::test_new(8,  2),
				result: Ordering::Greater
			},
			Test { //diff row, same col
				a: Position::test_new(15,  7), 
				b: Position::test_new(17,  7),
				result: Ordering::Greater
			},
			Test { //same row/col
				a: Position::test_new(14,  1), 
				b: Position::test_new(14,  1),
				result: Ordering::Greater
			},
		];

		for test in vecs {
			assert_eq!(test.a.cmp(&test.b), test.result);
		}
	}
}
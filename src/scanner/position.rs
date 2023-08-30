pub struct Position {
    row_start: i32,
    col_start: i32,
    row_end: i32,
    col_end: i32
}

impl Position {
    pub fn new(row_start: i32, col_start: i32, row_end: i32, col_end: i32) -> Position {
        Position {
            row_start,
            col_start,
            row_end,
            col_end
        } 
    }

    pub fn start_to_string(&self) -> String {
        format!("[{}, {}]", self.row_start, self.col_start)
    }

    pub fn span_to_string(&self) -> String {
        format!("{} - [{}, {}]", self.start_to_string(), self.row_start, self.col_start)
    }
}
#[derive(Debug)]
pub enum PartialDate {
    Year { year: i32 },
    YearMonth { year: i32, month: u8 },
    FullDate { year: i32, month: u8, day: u8 },
}

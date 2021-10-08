use chrono::{DateTime, Utc};

fn main() {
    println!("Hello, world!");
}
// Value Objects
pub struct ScreeningTime {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

pub struct Screening {
    movie: Movie,
    room: Room,
    time: ScreeningTime,
}

pub struct Year(u32);

pub struct Movie {
    title: String,
    year: Year,
    cast: Vec<String>,
    director: String,
    duration: Duration,
}

pub struct Duration(u32);

pub struct Room {
    name: String,
    seats: Vec<Seat>,
    features: Vec<RoomFeature>,
}

pub struct Seat {
    row: u32,
    number: u32,
    ty: SeatType,
}

pub enum SeatType {
    Standard,
    DBox,
}

pub enum RoomFeature {
    DDD,
}
// Entity
pub struct ScreeningID(u32);
pub struct ScreeningSchedule {
    id: ScreeningID,
    screening: Screening,
    taken_seats: Vec<Seat>,
    available_seats: Vec<Seat>,
}

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

impl ScreeningSchedule {
    pub fn reserve_seats(&self, seats: Vec<SeatLocation>) -> Result<(), String> {
        todo!()
    }
}
pub struct CustomerId(u32);

pub type SeatLocation = (u32, u32);

pub struct ReserveSeatsCommand {
    screening_id: ScreeningID,
    customer_id: CustomerId,
    seats: Vec<SeatLocation>,
}

pub struct Customers {
    customers: Vec<CustomerId>,
}

impl Customers {
    pub fn new(customers: Vec<CustomerId>) -> Customers {
        Customers { customers }
    }

    pub fn get(&self, id: CustomerId) -> Option<CustomerId> {
        todo!()
    }
}
pub struct Screenings {
    screenings: Vec<ScreeningSchedule>,
}

impl Screenings {
    pub fn new(screenings: Vec<ScreeningSchedule>) -> Screenings {
        Screenings { screenings }
    }
    pub fn get(&self, id: ScreeningID) -> Option<ScreeningSchedule> {
        todo!()
    }

    pub fn store(&self, screening_schedule: ScreeningSchedule) {
        todo!()
    }
}
pub struct CommandHandler {
    customers: Customers,
    schedules: Screenings,
}

impl CommandHandler {
    pub fn new(customers: Customers, schedules: Screenings) -> Self {
        Self {
            customers,
            schedules,
        }
    }

    pub fn handle(&self, command: ReserveSeatsCommand) -> Result<(), String> {
        let customer_ = self.customers.get(command.customer_id).unwrap();
        let screening = self.schedules.get(command.screening_id).unwrap();

        screening.reserve_seats(command.seats)?;

        self.schedules.store(screening);

        Ok(())
    }
}

#[test]
fn reservation_test_ok() {
    let handler = CommandHandler::new(Customers::new(vec![CustomerId(1)]), Screenings::new(vec![]));

    let result = handler.handle(ReserveSeatsCommand {
        screening_id: ScreeningID(1),
        customer_id: CustomerId(1),
        seats: vec![],
    });
    assert!(result.is_ok())
}

#[test]
fn reservation_test_not_available() {
    let handler = CommandHandler::new(Customers::new(vec![CustomerId(1)]), Screenings::new(vec![]));

    let result = handler.handle(ReserveSeatsCommand {
        screening_id: ScreeningID(1),
        customer_id: CustomerId(1),
        seats: vec![],
    });
    assert!(result.is_err())
}

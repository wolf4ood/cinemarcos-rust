use std::{collections::HashMap, hash::Hash, panic, vec};

use chrono::{DateTime, Utc};

fn main() {
    println!("Hello, world!");
}

// Value Objects
#[derive(Clone)]
pub struct ScreeningTime {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}
#[derive(Clone)]
pub struct Screening {
    movie: Movie,
    room: Room,
    time: ScreeningTime,
}

#[derive(Clone)]
pub struct Year(u32);

// #[derive(Clone)]
// pub struct Movie {
//     title: String,
//     year: Year,
//     cast: Vec<String>,
//     director: String,
//     duration: Duration,
// }

#[derive(Clone)]
pub struct Movie {
    title: String,
}
#[derive(Clone)]
pub struct Duration(u32);

#[derive(Clone)]
pub struct Room {
    name: String,
    seats: Vec<Seat>,
    features: Vec<RoomFeature>,
}

#[derive(Clone)]
pub struct Seat {
    row: u32,
    number: u32,
    ty: SeatType,
}

#[derive(Clone)]
pub enum SeatType {
    Standard,
    DBox,
}

#[derive(Clone)]
pub enum RoomFeature {
    DDD,
}
// Entity

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct ScreeningID(u32);
pub struct ScreeningSchedule {
    state: Option<ScreeningScheduleState>,
}

#[derive(Clone)]
pub struct ScreeningScheduleState {
    id: ScreeningID,
    screening: Screening,
    taken_seats: Vec<Seat>,
    available_seats: Vec<Seat>,
}
#[derive(Clone)]
pub struct ScreeningCreated {
    id: ScreeningID,
    screening: Screening,
}
#[derive(Clone)]
pub enum ScreeningEvent {
    Created(ScreeningCreated),
    SeatReserved(SeatReserved),
}

impl ScreeningEvent {
    fn aggregate_id(&self) -> ScreeningID {
        match self {
            ScreeningEvent::Created(created) => created.id.clone(),
            ScreeningEvent::SeatReserved(reserved) => reserved.screening_id.clone(),
        }
    }
}

impl ScreeningSchedule {
    pub fn reserve_seats(
        &self,
        customer: CustomerId,
        seats: Vec<SeatLocation>,
    ) -> Result<ScreeningEvent, String> {
        let state = self.state.as_ref().unwrap();

        let mut is_available = true;
        for seat in seats {
            is_available = is_available
                && state.available_seats.iter().any(|available_seat| {
                    available_seat.number == seat.0 && available_seat.row == seat.1
                });
        }

        if is_available {
            // todo logic
            Ok(ScreeningEvent::SeatReserved(SeatReserved {
                customer_id: customer,
                screening_id: state.id.clone(),
                seats: vec![],
            }))
        } else {
            Err(String::from("Seats not available"))
        }
    }

    pub fn apply(&mut self, event: ScreeningEvent) {
        match (self.state.as_ref(), event) {
            (None, ScreeningEvent::Created(ScreeningCreated { id, screening })) => {
                self.state = Some(ScreeningScheduleState {
                    id: id,
                    screening: screening.clone(),
                    taken_seats: vec![],
                    available_seats: screening.room.seats.clone(),
                })
            }
            (None, ScreeningEvent::SeatReserved(_)) => todo!(),
            (Some(_), ScreeningEvent::Created(_)) => panic!("invalid event created without state"),
            (
                Some(_),
                ScreeningEvent::SeatReserved(SeatReserved {
                    seats,
                    screening_id,
                    customer_id,
                }),
            ) => {
                todo!()
            }
        }
    }
}

#[derive(Clone)]
pub struct CustomerId(u32);

pub type SeatLocation = (u32, u32);

pub struct Customers {
    customers: Vec<CustomerId>,
}

impl Customers {
    pub fn new(customers: Vec<CustomerId>) -> Customers {
        Customers { customers }
    }

    pub fn get(&self, id: CustomerId) -> Option<CustomerId> {
        Some(CustomerId(1))
    }
}
pub struct Screenings {
    screenings: HashMap<ScreeningID, ScreeningSchedule>,
}

impl Screenings {
    pub fn new(screenings: HashMap<ScreeningID, ScreeningSchedule>) -> Screenings {
        Screenings { screenings }
    }
    pub fn get(&self, id: ScreeningID) -> Option<&ScreeningSchedule> {
        self.screenings.get(&id)
    }
}

#[derive(Clone)]
pub struct SeatReserved {
    seats: Vec<Seat>,
    screening_id: ScreeningID,
    customer_id: CustomerId,
}
pub struct ReserveSeatsCommand {
    screening_id: ScreeningID,
    customer_id: CustomerId,
    seats: Vec<SeatLocation>,
}
pub struct CommandHandler {
    store: EventStore,
}

pub struct EventStore {
    events: Vec<ScreeningEvent>,
}

impl EventStore {
    pub fn new(events: Vec<ScreeningEvent>) -> Self {
        Self { events }
    }

    pub fn by_aggregate_id(&self, aggregate_id: ScreeningID) -> Vec<ScreeningEvent> {
        self.events
            .iter()
            .filter(|event| event.aggregate_id() == aggregate_id)
            .cloned()
            .collect()
    }
}

pub enum ScreeningCommand {
    ReserveSeat(ReserveSeatsCommand),
}

impl CommandHandler {
    pub fn new(store: EventStore) -> Self {
        Self { store }
    }

    pub fn handle(&self, command: ReserveSeatsCommand) -> Result<(), String> {
        let events = self.store.by_aggregate_id(command.screening_id);

        let mut screening = ScreeningSchedule { state: None };
        for event in events {
            screening.apply(event);
        }
        screening.reserve_seats(command.customer_id, command.seats)?;

        Ok(())
    }
}

fn create_handler_mock() -> CommandHandler {
    CommandHandler::new(EventStore::new(vec![ScreeningEvent::Created(
        ScreeningCreated {
            id: ScreeningID(1),
            screening: Screening {
                movie: Movie {
                    title: "Matrix".to_string(),
                },
                room: Room {
                    name: "A1".to_string(),
                    seats: vec![Seat {
                        row: 1,
                        number: 1,
                        ty: SeatType::Standard,
                    }],
                    features: vec![],
                },
                time: ScreeningTime {
                    start: Utc::now(),
                    end: Utc::now(),
                },
            },
        },
    )]))
}
#[test]
fn reservation_test_ok() {
    let handler = create_handler_mock();
    let result = handler.handle(ReserveSeatsCommand {
        screening_id: ScreeningID(1),
        customer_id: CustomerId(1),
        seats: vec![],
    });
    assert!(result.is_ok())
}

#[test]
fn reservation_test_not_available() {
    let handler = create_handler_mock();
    let result = handler.handle(ReserveSeatsCommand {
        screening_id: ScreeningID(1),
        customer_id: CustomerId(1),
        seats: vec![],
    });
    assert!(result.is_err())
}

#[test]
fn reservation_test_too_late() {
    let handler = create_handler_mock();
    let result = handler.handle(ReserveSeatsCommand {
        screening_id: ScreeningID(1),
        customer_id: CustomerId(1),
        seats: vec![],
    });
    assert!(result.is_err())
}

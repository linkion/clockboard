use iced::{self, Sandbox, Settings};

use chrono::{TimeZone, Utc};
use chrono_tz::Tz;
use chrono_tz::UTC;

fn main() -> iced::Result {
    Board::run(Settings::default())
}

pub struct Clock {
    timezone: Tz,
    
}

impl Clock {
    fn new() -> Clock {
        Clock{
            timezone: UTC,
        }
    }
    fn new_with_tz(timezone: String) -> Clock {
        Clock { timezone: timezone.parse().unwrap() }
    }
    fn currentTime(self) -> String {
        self.timezone.from_utc_datetime(&Utc::now().naive_utc()).time().to_string()
    }
}

pub struct Board {
    clocks: Vec<Clock>,
    debug: bool,
}

impl Sandbox for Board {
    fn theme(&self) -> iced::Theme {
        iced::Theme::default()
    }

    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::default()
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn run(settings: iced::Settings<()>) -> Result<(), iced::Error>
    where
        Self: 'static + Sized,
    {
        <Self as iced::Application>::run(settings)
    }

    type Message = Message;

    fn new() -> Board {
        Board {
            debug: true,
            clocks: vec![Clock::new()],
        }
    }

    fn title(&self) -> String {
        format!("{} - Iced", "ClockBoard")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::AddClock(tz) => self.clocks.push(Clock::new_with_tz(tz)),
            Message::RemoveClock(index) => todo!(),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    AddClock(String),
    RemoveClock(u8),
}
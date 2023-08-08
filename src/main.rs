use iced::theme::{self, Theme};
use iced::widget::{button, column, text, row};
use iced::{Alignment, Color, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    theme: Theme,
    value: i32,
    value2: f64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementInt,
    DecrementInt,
    IncrementFloat,
    DecrementFloat
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { theme: Theme::custom(theme::Palette {
            background: Color::BLACK,
            text: Color::from_rgb(0.8,0.8,0.8),
            primary: Color::from_rgb(0.5, 0.5, 0.3),
            success: Color::from_rgb(0.0, 1.0, 0.0),
            danger:  Color::from_rgb(1.0, 0.0, 0.0),
        }), value: 0, value2: 0.0 }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementInt => {
                self.value += 1;
            }
            Message::DecrementInt => {
                self.value -= 1;
            }
            Message::IncrementFloat => {
                self.value2 += 0.1;
            }
            Message::DecrementFloat => {
                self.value2 -= 0.1;
            }

        }
    }

    fn view(&self) -> Element<Message> {
        row![
            column![
                button("Increment").on_press(Message::IncrementInt),
                text(self.value).size(50),
                button("Decrement").on_press(Message::DecrementInt),
            ]
            .padding(20)
            .align_items(Alignment::Center),
            column![
                button("Inc Float").on_press(Message::IncrementFloat),
                text(self.value2).size(50),
                button("Dec Float").on_press(Message::DecrementFloat)
            ]
            .padding(20)
            .align_items(Alignment::Center)
        ]
        .padding(20)
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

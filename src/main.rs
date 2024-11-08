use iced::widget::{center, text};
use iced::window;
use iced::{Element, Subscription, Task};
use std::time::Duration;

fn main() -> iced::Result {
    iced::daemon(Example::title, Example::update, Example::view)
        .subscription(Example::subscription)
        .run_with(Example::new)
}

#[derive(Debug, Clone)]
#[allow(unused)]
enum Message {
    Tick,
    WindowOpened(window::Id),
}

struct Example {
    counter: usize,
}

impl Example {
    fn new() -> (Self, Task<Message>) {
        let (_, open0) = window::open(window::Settings::default());
        let (_, open1) = window::open(window::Settings::default());
        let tasks = [open0, open1].map(|t| t.map(Message::WindowOpened));
        (Self { counter: 0 }, Task::batch(tasks))
    }

    fn title(&self, window: window::Id) -> String {
        format!("Window {window}")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => self.counter += 1,
            Message::WindowOpened(_) => (),
        }
        Task::none()
    }

    fn view(&self, _window_id: window::Id) -> Element<Message> {
        center(text!("Counter: {:}", self.counter)).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(1)).map(|_| Message::Tick)
    }
}

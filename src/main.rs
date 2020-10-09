use iced::{Application, Command, Element, Sandbox, Length, Container, Settings};
use iced_native::Text;

#[derive(Clone, Debug)]
struct DiscordAnon;

#[derive(Clone, Debug)]
enum Messages {}

impl Sandbox for DiscordAnon {
    type Message = Messages;

    fn new() -> Self {
        DiscordAnon
    }

    fn title(&self) -> String {
        String::from("DiscordAnon")
    }

    fn update(&mut self, message: Self::Message) {
        println!("message: {:?}", message);
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let lbl = Text::new("Hello, world!")
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(lbl)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() {
    DiscordAnon::run(Settings::default())
}

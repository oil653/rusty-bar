use iced::{Color, Element, Font, Length, Subscription, Task, border, theme::{self, Theme}, widget::{container, row, text}};

use iced_layershell::daemon;
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::{LayerShellSettings, StartMode, Settings};
use iced_layershell::to_layer_message;

use std::sync::OnceLock;
use std::time::Duration;

use chrono::Local;


#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
    TimeTrigger,
}

#[derive(Debug, Default)]
struct State {
// GLOBAL VARS
    theme: Option<Theme>,
    radius: i32,
    time_fmt: &'static str,
// LEFT SIDE
    clock: String,
}

impl State {
    fn new(theme: Option<Theme>, radius: i32, time_fmt: &'static str) -> Self {
        Self { 
            theme,
            radius,
            time_fmt,
            ..Default::default() 
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        use Message::*;
        match message {
            TimeTrigger => {
                let now = Local::now();
                self.clock = now.format(self.time_fmt).to_string();
                Task::none()
            }
            _ => {Task::none()}
        }
    }

    fn view(&self, _window: iced::window::Id) -> Element<'_, Message> {
        let clock = text(&self.clock);
        let left = row![clock];

        let middle = row![];
        
        let right = row![];

        container(row![left, middle, right])
        .center(Length::Fill)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();

            container::Style::default()
                .background(palette.background.base.color)
                .border(border::rounded(self.radius))
        })
        .into()
    }

    fn subscription(_state: &State) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(1)).map(|_| Message::TimeTrigger)
    }

    fn style(_: &State, theme: &Theme) -> theme::Style {
        theme::Style {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text
        }
    }
}

fn main() -> iced_layershell::Result {
    // Setting ICED_BACKEND to software will panic, for some reason...
    unsafe {
        std::env::set_var("ICED_BACKEND", "tiny-skia");
    }

    let theme = Some(Theme::CatppuccinMocha);
    let radius = 10;
    let time_fmt = "%H:%M:%S";

    daemon(
        move || {
            State::new(theme.clone(), radius, time_fmt)
        },
        "Rusty Bar", 
        State::update,
        State::view
    )
    .settings(Settings {
        layer_settings: LayerShellSettings {
            layer: iced_layershell::reexport::Layer::Top,
            size: Some((0, 50)),
            start_mode: StartMode::AllScreens,
            anchor: Anchor::Top | Anchor::Left | Anchor::Right,
            exclusive_zone: 50,
            margin: (0, 10, 0, 10),
            ..Default::default()
        },
        fonts: vec![
            std::include_bytes!("assets/fonts/Itim-Regular.ttf").into()
        ],
        default_font: Font::with_name("Itim"),
        ..Default::default()
    })
    .style(State::style)
    .theme(|state: &State, _| {
        // Theme::CatppuccinMocha
        state.theme.clone()
    })
    .subscription(State::subscription)
    .run()
}

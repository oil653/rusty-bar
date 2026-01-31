use iced::{
    Alignment, 
    Color, 
    Element, 
    Font, 
    Length, 
    Padding, 
    Renderer, 
    Subscription, 
    Task, 
    border, 
    theme::{
        self, 
        Theme
    }, 
    widget::{
            Container, Space, container, mouse_area, row, space, svg, text
    }, window,
};

use iced_layershell::{
    daemon,
    reexport::{Anchor, NewLayerShellSettings},
    settings::{
        LayerShellSettings, Settings, StartMode
    },
    to_layer_message
};

use std::{collections::HashMap, time::Duration};
use chrono::Local;

// Weather backend
mod weather;
use weather::prelude::*;
use crate::weather::CurrentWeather;

// The notification of rusty bar to the user (things like errrors, notices, and other messages)
mod notification;
use crate::notification::Notification;

// Contains svgs and other small assets
mod assets;
use crate::assets::ASSETS_WEATHER;

mod windows;
use windows::weather_window;

#[derive(Debug, Clone)]
#[non_exhaustive]
enum WindowType {
    Main,
    Weather
}

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
enum Message {
    NewNotif(Notification),
    NotifRetry(Notification),


    SecondTrigger,


    ParseCurrentWeather,
    CurrentWeatherParsed(Result<CurrentWeather, ParsingError>),

    WeatherWindowMessage(weather_window::Message),
    WeatherWindowToggle,
}

#[derive(Debug, Default)]
struct State {
    window_ids: HashMap<window::Id, WindowType>,
// GLOBAL VARS
    theme: Option<Theme>,
    radius: i32,
    time_fmt: &'static str,
    spacing: u32,
    hpadding: u32,
    notifications: Vec<Notification>,
// LEFT SIDE
    clock: String,
    clock_widget_width: u32,    // SETTING


    first_parse: bool, // Parse initial stuff, it will only be run once, when the program starts

    // None if the location should be parsed from ip address, not a specified position
    tracked_location: Option<Coordinates>,
    weather_current: Option<CurrentWeather>,
    units: Units,

    weather_window_id: Option<window::Id>,
    weather_window_state: weather_window::State,

}

impl State {
    fn new(
        theme: Option<Theme>, 
        radius: i32, 
        spacing: u32, 
        time_fmt: &'static str, 
        clock_widget_width: u32,
        hpadding: u32,
        units: Units
    ) -> Self {
        Self { 
            theme,
            radius,
            time_fmt,
            spacing,
            clock_widget_width,
            hpadding,
            units,
            first_parse: true,
            ..Default::default() 
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        use Message::*;
        match message {
            RemoveWindow(id) => {
                self.window_ids.remove(&id);
                println!("Removing window id {id}");
                Task::none()
            },


            NewNotif(notif) => {
                println!("New notification: {:#?}", notif);
                self.notifications.push(notif);
                Task::none()
            },
            NotifRetry(notif) => {
                notif.retry().expect("{0}")
            },
            



            SecondTrigger => {
                let now = Local::now();
                self.clock = now.format(self.time_fmt).to_string();

                if self.first_parse {
                    self.first_parse = false;
                    Task::done(Message::ParseCurrentWeather)
                } else {
                    Task::none()
                }
            },
            



            ParseCurrentWeather => {
                println!("Parsing current weather");

                use argument::Current;
                Task::perform(get_current(
                    self.tracked_location.clone(), 
                    self.units.clone(),
                    vec![
                        Current::Temperature,
                        Current::IsDay,
                        Current::ApparentTemp,
                        Current::Humidity,
                        Current::WeatherCode,
                        Current::WindDirection,
                        Current::WindSpeed,
                        Current::Precipitation(argument::PrecipitationType::Combined),
                        Current::Precipitation(argument::PrecipitationType::Rain),
                        Current::Precipitation(argument::PrecipitationType::Showers),
                        Current::Precipitation(argument::PrecipitationType::Snowfall)
                    ]
                ), Message::CurrentWeatherParsed)
            },
            CurrentWeatherParsed(result) => {
                match result {
                    Ok(weather) => {
                        self.weather_current = Some(weather);
                        Task::none()
                    },
                    Err(e) => {
                        Task::done(
                            NewNotif(
                                Notification::new_with_retry(
                                    notification::Level::Error, 
                                    e.to_string(), 
                                    Local::now(),
                                    &Message::ParseCurrentWeather
                                )
                            )
                        )
                    }
                }
            },

            WeatherWindowToggle => {
                if let Some(id) = self.weather_window_id {
                    // println!("Closing weather_window with id: {}", id);
                    self.weather_window_id = None;
                    window::close(id)
                } else {
                    let id = window::Id::unique();
                    self.window_ids.insert(id, WindowType::Weather);
                    self.weather_window_id = Some(id);

                    // println!("Opening new weather_window with id: {}", id);
                    
                    Task::done(Message::NewLayerShell { 
                        settings: NewLayerShellSettings { 
                            size: Some((450, 350)),
                            layer: iced_layershell::reexport::Layer::Top,
                            anchor: Anchor::Top | Anchor::Left,
                            margin: Some((10, 0, 0, 30)),
                            keyboard_interactivity: iced_layershell::reexport::KeyboardInteractivity::OnDemand, 
                            output_option: iced_layershell::reexport::OutputOption::LastOutput,
                            ..Default::default()
                        },
                        id
                    })
                }
            },

            _ => {Task::none()}
        }
    }

    fn separator<'a>() -> Container<'a, Message, Theme, Renderer> {
        container(Space::new())
            .width(2)
            .height(30)
            .align_y(Alignment::Center)
            .align_x(Alignment::Center)
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();

                container::Style::default()
                    .background(palette.background.weakest.color)
            })
    }

    /// Returns the type of the window, if it's not found in `self.window_ids` it's assumed to be Main
    fn match_id(&self, id: &window::Id) -> &WindowType {
        match self.window_ids.get(&id) {
            Some(window) => window,
            None => &WindowType::Main
        }
    }

    fn view(&self, id: window::Id) -> Element<'_, Message> {
        use WindowType::*;
        match *self.match_id(&id) {
            Main => self.main_view(),
            Weather => self.weather_window_state.view(&self),
        }
    }

    fn main_view(&self) -> Element<'_, Message> {
        let clock = container(
            text(&self.clock)
            .size(36)
            .center()
            .width(self.clock_widget_width)
            .style(text::primary)
        )
            .padding(Padding::default().horizontal(self.hpadding))
            .width(Length::Shrink)
            .height(Length::Fill)
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();

                container::Style::default()
                    .background(palette.background.weak.color)
                    .border(border::rounded(self.radius))
            })
        ;
        
        let weather_widget = {
            match &self.weather_current {
                Some(weather) => {
                    let svg_handle = iced::widget::svg::Handle::from_memory(
                        ASSETS_WEATHER
                            .get()
                            .unwrap()
                            .get(if weather.is_day.unwrap() {"day"} else {"night"})
                            .unwrap()
                            .get(weather.code.as_ref().unwrap().get_svg_name().as_str())
                            .unwrap()
                            .as_bytes()
                    );

                    container
                    (
                        mouse_area
                        (
                            row!
                            [
                                svg(svg_handle)
                                    .width(36)
                                    .height(36)
                                    .content_fit(iced::ContentFit::Fill),
                                text(weather.temperature.as_ref().unwrap().stringify())
                                    .align_y(Alignment::Center)
                                    .size(36)
                                    .style(text::primary)
                            ]
                            .spacing(5)
                            .align_y(Alignment::Center)
                        )
                        .on_press(Message::WeatherWindowToggle)
                    )
                    .padding(Padding::default().horizontal(self.hpadding))
                    .width(Length::Shrink)
                    .height(Length::Fill)
                    .style(|theme: &Theme| {
                        let palette = theme.extended_palette();

                        container::Style::default()
                            .background(palette.background.weak.color)
                            .border(border::rounded(self.radius))
                    })
                },
                None => {
                    container(space())
                }
            }
        };


        let left = row![
            clock,
            Self::separator(),
            weather_widget,
            Self::separator(),
        ]
            .align_y(Alignment::Center)
            .spacing(self.spacing);


//
        let middle = row![]
            .align_y(Alignment::Center)
            .spacing(self.spacing);
        


//
        let right = row![]
            .align_y(Alignment::Center)
            .spacing(self.spacing);


        container(
            row![
                left,
                space::horizontal(),
                middle,
                space::horizontal(),
                right
            ]
                .align_y(Alignment::Center)
                .padding(5)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_y(Alignment::Center)
        .align_x(Alignment::Center)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();

            container::Style::default()
                .background(palette.background.base.color)
                .border(border::rounded(self.radius))
        })
        .into()
    }

    fn subscription(_state: &State) -> Subscription<Message> {
        Subscription::batch([
            iced::time::every(Duration::from_secs(1)).map(|_| Message::SecondTrigger),
            iced::time::every(Duration::from_mins(15)).map(|_| Message::ParseCurrentWeather)
        ])
    }

    fn style(_: &State, theme: &Theme) -> theme::Style {
        theme::Style {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text
        }
    }
}

fn main() -> iced_layershell::Result {
    assets::load_assets();

    // Setting ICED_BACKEND to software will panic, for some reason...
    unsafe {
        std::env::set_var("ICED_BACKEND", "tiny-skia");
    }

    let theme = Some(Theme::Dracula);
    let radius = 10;
    let time_fmt = "%H:%M:%S";
    let spacing = 4;
    let clock_widget_width = 140;
    let hpadding = 4;
    let units = Units::default();

    daemon(
        move || {
            State::new(
                theme.clone(), 
                radius, 
                spacing, 
                time_fmt, 
                clock_widget_width,
                hpadding,
                units.clone()
            )
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
            margin: (5, 10, 5, 10),
            keyboard_interactivity: iced_layershell::reexport::KeyboardInteractivity::OnDemand,
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
        state.theme.clone()
    })
    .subscription(State::subscription)
    .run()
}

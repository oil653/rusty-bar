use iced::{
    Alignment, Color, Element, Font, Length, Padding, Renderer, Subscription, Task, border, theme::{
        self, 
        Theme
    }, widget::{
        Container, Space, container, mouse_area, row, space, svg, text
    }
};

use iced_layershell::{
    daemon,
    reexport::Anchor,
    settings::{
        LayerShellSettings, 
        StartMode, 
        Settings
    },
    to_layer_message
};

use std::time::Duration;
use chrono::{Local};

mod weather;
use weather::prelude::*;

use crate::notification::Notification;

// The notification of rusty bar to the user (things like errrors, notices, and other messages)
mod notification;

use crate::weather::CurrentWeather;


#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
    NewNotif(Notification),
    NotifRetry(Notification),

    TimeTrigger,

    ParseCurrentWeather,
    CurrentWeatherParsed(Result<CurrentWeather, ParsingError>)
}

#[derive(Debug, Default)]
struct State {
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

    // None if the location should be parsed from ip address, not a specified position
    tracked_location: Option<Coordinates>,
    weather_current: Option<CurrentWeather>,
    units: Units,
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
            },
            ParseCurrentWeather => {
                use argument::Current;

                Task::perform(get_current(
                    self.tracked_location.clone(), 
                    self.units.clone(),
                    vec![
                        Current::ApparentTemp,
                        Current::Humidity,
                        Current::IsDay,
                        Current::Temperature,
                        Current::WeatherCode,
                        Current::WindDirection,
                        Current::WindSpeed,
                        Current::Precipitation(argument::PrecipitationTypes::Combined),
                        Current::Precipitation(argument::PrecipitationTypes::Rain),
                        Current::Precipitation(argument::PrecipitationTypes::Showers),
                        Current::Precipitation(argument::PrecipitationTypes::Snowfall)
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
            NewNotif(notif) => {
                self.notifications.push(notif);
                Task::none()
            },
            NotifRetry(notif) => {
                notif.retry().expect("{0}")
            }
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

    fn view(&self, _window: iced::window::Id) -> Element<'_, Message> {
        let clock = container(
            text(&self.clock)
            .size(36)
            .align_y(Alignment::Center)
            .align_x(Alignment::Center)
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
            }
        );

        
        // let weather_widget = {
        //     match &self.weather_current {
        //         Some(weather) => {
        //             container
        //             (
        //                 mouse_area
        //                 (
        //                     row!
        //                     [
        //                         svg(format!("assets/svgs/weather/{}", weather.code.as_ref().unwrap().get_svg_name(weather.is_day.unwrap())))
        //                         .width(35)
        //                         .height(36),
        //                         text(weather.temperature.as_ref().unwrap().stringify())
        //                     ]
        //                     .spacing(5)
        //                 )
        //             )
        //             .padding(Padding::default().horizontal(self.hpadding))
        //         },
        //         None => 
        //         {
        //             unimplemented!()
        //         }
        //     }
        // };


        let left = row![
            clock,
            Self::separator(),
            // weather_widget
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

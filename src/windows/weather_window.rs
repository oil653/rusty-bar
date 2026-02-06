use iced::{Alignment, Color, Element, Length, Renderer, Task, Theme, border, widget::{Row, button, column, container, row, scrollable::{ Direction, Scrollbar}, scrollable, space, svg, text}};
use iced::widget::text::LineHeight;
use iced_layershell::{
    to_layer_message
};

use crate::ASSETS_WEATHER;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DisplayMode {
    Graph, 
    #[default]
    Cards
}

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum Message {
    DisplayModeChange(DisplayMode)
}

#[derive(Debug, Default)]
pub struct State {
    hourly_display_mode: DisplayMode
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<crate::Message> {
        match message {
            Message::DisplayModeChange(mode) => {
                // println!("DisplayModeChanged to {:#?}", mode);
                self.hourly_display_mode = mode;
                Task::none()
            },
            _ => Task::none()
        }
    }

    pub fn view<'a>(&'a self, state: &'a crate::State) -> Element<'a, crate::Message> {
        let padding = 5;

        // Common svgs
        let wind = svg::Handle::from_memory(
        ASSETS_WEATHER
                    .get()
                    .unwrap()
                    .get("weather")
                    .unwrap()
                    .get("wind")
                    .unwrap()
                    .as_bytes()
        );
        
        let droplet = svg::Handle::from_memory(
        ASSETS_WEATHER
                    .get()
                    .unwrap()
                    .get("weather")
                    .unwrap()
                    .get("droplet")
                    .unwrap()
                    .as_bytes()
        );

        let humidity = svg::Handle::from_memory(
        ASSETS_WEATHER
                    .get()
                    .unwrap()
                    .get("weather")
                    .unwrap()
                    .get("humidity")
                    .unwrap()
                    .as_bytes()
        );

        let refresh = svg::Handle::from_memory(
            ASSETS_WEATHER
                .get()
                .unwrap()
                .get("commons")
                .unwrap()
                .get("refresh")
                .unwrap()
                .as_bytes()
        );

        // Current weather
        let current_height = 120;
        let current = {
            container(match &state.weather_current{
                Some(weather) => {
                    let code_svg_handle = svg::Handle::from_memory(
                            ASSETS_WEATHER
                                .get()
                                .unwrap()
                                .get(if weather.is_day.unwrap() {"day"} else {"night"})
                                .unwrap()
                                .get(weather.code.as_ref().unwrap().get_svg_name().as_str())
                                .unwrap()
                                .as_bytes()
                    );

                    let code_string_size = 
                        if weather.code.as_ref().unwrap().to_string().chars().count() < 15 {
                            32
                        }
                        else if weather.code.as_ref().unwrap().to_string().chars().count() > 23 {
                            28
                        } else {
                            24
                        };

                    container(
                        row![
                            svg(code_svg_handle)
                                .height(current_height - 2 * padding)
                                .width(current_height - 2 * padding)
                                .content_fit(iced::ContentFit::Fill),
                            column![
                                // Upper row
                                row![
                                    column![
                                        text(weather.temperature.as_ref().unwrap().stringify())
                                            .size(60)
                                            .style(text::primary)
                                            .align_y(Alignment::Start)
                                            .line_height(LineHeight::Relative(0.8))
                                        ,
                                        text(format!("Feels like {}", weather.apparent_temperature.as_ref().unwrap().stringify()))
                                            .size(18)
                                            .style(text::secondary)
                                            .line_height(LineHeight::Relative(0.9)),
                                    ],
                                    space::horizontal(),
                                    column![
                                        row![
                                            text(weather.precipitation.as_ref().unwrap().combined_to_string())
                                                .size(18)
                                                .style(text::primary)
                                                .align_y(Alignment::Center),
                                            svg(droplet.clone())
                                                .width(18)
                                                .height(18)
                                        ],
                                        row![
                                            text(weather.humidity.as_ref().unwrap().stringify())
                                                .size(18)
                                                .style(text::primary)
                                                .align_y(Alignment::Center),
                                            svg(humidity.clone())
                                                .width(18)
                                                .height(18)
                                        ],
                                        row![
                                            text(weather.wind.as_ref().unwrap().stringify())
                                                .size(18)
                                                .style(text::primary)
                                                .align_y(Alignment::Center),
                                            svg(wind.clone())
                                                .width(18)
                                                .height(18)
                                        ],
                                    ]
                                    .align_x(Alignment::End)
                                ],
                                space::vertical(),
                                // Lower text
                                text(weather.code.as_ref().unwrap().to_string())
                                    .size(code_string_size)
                                    .style(text::primary)
                            ]
                            .width(Length::Fill)
                            .height(Length::Fill)
                        ]
                        .align_y(Alignment::Start)
                        .spacing(5)
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Alignment::Start)
                    .align_y(Alignment::Center)
                },
                None => {
                    container(
                        space()
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                }
            })
            .width(Length::Fill)
            .height(current_height)
            .align_left(Length::Fill)
            .padding(padding as u16)
            .style(|theme: &Theme| container::Style::default()
                .background(theme.extended_palette().background.strong.color)
                .border(border::rounded(state.radius))
            )
        };

        // Hourly weather
        let mode_switcher = {
            container(
                row![
                    // Cards
                    {button(
                        text("Cards")
                            .center()
                            .size(20)
                            .style(|theme: &Theme| {
                                let palette = theme.extended_palette();

                                text::Style {
                                    color: palette.primary.strong.text.into()
                                }
                            })
                    )
                    .width(55)
                    .height(25)
                    .style(|theme: &Theme, _status: button::Status| {
                        let palette = theme.extended_palette();

                        button::Style {
                            background: if self.hourly_display_mode == DisplayMode::Cards {Some(palette.primary.strong.color.into())} else {Some(palette.secondary.base.color.into())},
                            border: iced::Border { 
                                width: 5.0, 
                                radius: border::Radius { top_left: state.radius as f32, top_right: 0.0, bottom_right: 0.0, bottom_left: state.radius as f32 },
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    })
                    .on_press_maybe({
                        if self.hourly_display_mode != DisplayMode::Cards {
                            Some(crate::Message::WeatherWindowMessage(Message::DisplayModeChange(DisplayMode::Cards)))
                        } else {
                            None
                        }
                    })},
                    // Graph
                    {button(
                        text("Graph")
                            .center()
                            .size(20)
                            .style(|theme: &Theme| {
                                let palette = theme.extended_palette();

                                text::Style {
                                    color: palette.primary.strong.text.into()
                                }
                            })
                    )
                    .width(60)
                    .height(25)
                    .style(|theme: &Theme, _status: button::Status| {
                        let palette = theme.extended_palette();

                        button::Style {
                            background: if self.hourly_display_mode == DisplayMode::Graph {Some(palette.primary.strong.color.into())} else {Some(palette.secondary.base.color.into())},
                            border: iced::Border { 
                                width: 5.0, 
                                radius: border::Radius { top_left: 0.0, top_right: state.radius as f32, bottom_right: state.radius as f32, bottom_left: 0.0 },
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    })
                    .on_press_maybe({
                        if self.hourly_display_mode != DisplayMode::Graph {
                            Some(crate::Message::WeatherWindowMessage(Message::DisplayModeChange(DisplayMode::Graph)))
                        } else {
                            None
                        }
                    })}
                ]
            )
        };

        let hourly_height = 135;
        let hourly_body: container::Container<'_, crate::Message, Theme, Renderer> = { container(
                if state.weather_hourly.is_empty() {
                        let row_thingy: Element<'_, crate::Message, Theme, Renderer> = row![
                            space::horizontal(),
                            text("Refresh hourly weather data")
                                .center()
                                .style(text::primary),
                            svg(refresh.clone())
                                .width(20)
                                .height(20),
                            space::horizontal()
                        ]
                            .spacing(5)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_y(Alignment::Center)
                            .into();

                        button(
                            row_thingy   
                        )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .on_press(crate::Message::ParseHourlyWeather)
                            .style(|_, _| button::Style {
                                background: Some(Color::TRANSPARENT.into()),
                                ..Default::default()
                            })
                            .into()
                } else {
                    match self.hourly_display_mode {
                        DisplayMode::Graph => {
                            <iced::widget::text::Text<'_, Theme, Renderer> as Into<iced::Element<'_, crate::Message, Theme, Renderer>>>::into(text("Unimplemented"))
                        },
                        DisplayMode::Cards => {
                            let mut cards = Vec::new();

                            for hour in &state.weather_hourly {
                                let code_svg_handle = svg::Handle::from_memory(
                                        ASSETS_WEATHER
                                            .get()
                                            .unwrap()
                                            .get(if hour.is_day.unwrap() {"day"} else {"night"})
                                            .unwrap()
                                            .get(hour.code.as_ref().unwrap().get_svg_name().as_str())
                                            .unwrap()
                                            .as_bytes()
                                );

                                cards.push(
                                    container(
                                        column![
                                            text(hour.time.format("%H:00").to_string())
                                                .align_x(Alignment::Center)
                                                .style(text::secondary)
                                                .size(12),
                                            svg(code_svg_handle)
                                                .width(58)
                                                .height(58),
                                            text(hour.temperature.as_ref().unwrap().stringify())
                                                .align_x(Alignment::Center)
                                                .style(text::primary)
                                                .line_height(LineHeight::Relative(0.95))
                                                .size(18),
                                            row![
                                                svg(droplet.clone())
                                                    .width(10)
                                                    .height(10),
                                                text(hour.precipitation.as_ref().unwrap().combined_to_string())
                                                    .align_x(Alignment::Center)
                                                    .style(text::secondary)
                                                    .line_height(LineHeight::Relative(0.9))
                                                    .size(12)
                                            ],
                                            row![
                                                svg(wind.clone())
                                                    .width(10)
                                                    .height(10),
                                                text(hour.wind.as_ref().unwrap().speed_stringify())
                                                    .align_x(Alignment::Center)
                                                    .style(text::secondary)
                                                    .line_height(LineHeight::Relative(0.9))
                                                    .size(12)
                                            ]
                                        ]
                                            .align_x(Alignment::Center)
                                            .spacing(2)
                                    )
                                        .width(65)
                                        .height(Length::Fill)
                                        .align_x(Alignment::Center)
                                        .style(|theme: &Theme| container::Style::default()
                                            .background(theme.extended_palette().background.strong.color)
                                            .border(border::rounded(state.radius))
                                        )
                                    .into()
                                );
                            }

                            scrollable(
                                Row::from_vec(cards)
                                    .height(hourly_height)
                                    .spacing(state.spacing)
                            )
                                .height(Length::Fill)
                                .width(Length::Fill)
                                .auto_scroll(true)
                                .direction(Direction::Horizontal(Scrollbar::new().spacing(5)))
                            .into()
                        }
                    }
                }
            )
            .height(hourly_height)
            .width(Length::Fill)
        };

        container(
            column![
                current,
                mode_switcher,
                hourly_body
            ].spacing(10)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(5)
        .style(|theme: &Theme| container::Style::default()
            .background(theme.extended_palette().background.weakest.color)
            .border(border::rounded(state.radius))
        )
        .into()
    }
}
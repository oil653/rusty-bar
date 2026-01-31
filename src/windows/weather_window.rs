use iced::{Alignment, Element, Length, Theme, border, widget::{column, container, row, space, svg, text}};
use iced::widget::text::LineHeight;
use iced_layershell::{
    to_layer_message
};

use crate::ASSETS_WEATHER;

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum Message {

}

#[derive(Debug, Default)]
pub struct State {

}

impl State {
    pub fn view<'a>(&self, state: &'a crate::State) -> Element<'a, crate::Message> {
        let current_height = 120;
        let current_padding = 5;

        let current = container(match &state.weather_current{
            Some(weather) => {
                let code_svg_handle = iced::widget::svg::Handle::from_memory(
                        ASSETS_WEATHER
                            .get()
                            .unwrap()
                            .get(if weather.is_day.unwrap() {"day"} else {"night"})
                            .unwrap()
                            .get(weather.code.as_ref().unwrap().get_svg_name().as_str())
                            .unwrap()
                            .as_bytes()
                );

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
                            .height(current_height - 2 * current_padding)
                            .width(current_height - 2 * current_padding)
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
                                        svg(droplet)
                                            .width(18)
                                            .height(18)
                                    ],
                                    row![
                                        text(weather.humidity.as_ref().unwrap().stringify())
                                            .size(18)
                                            .style(text::primary)
                                            .align_y(Alignment::Center),
                                        svg(humidity)
                                            .width(18)
                                            .height(18)
                                    ],
                                    row![
                                        text(weather.wind.as_ref().unwrap().stringify())
                                            .size(18)
                                            .style(text::primary)
                                            .align_y(Alignment::Center),
                                        svg(wind)
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
        .padding(current_padding as u16)
        .style(|theme: &Theme| container::Style::default()
            .background(theme.extended_palette().background.strong.color)
            .border(border::rounded(state.radius))
        );

        container(
            column![
                current,
            ]
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
use iced::{Alignment, Element, Length, Theme, border, widget::{container, space}};
use iced_layershell::{
    to_layer_message
};

#[to_layer_message(multi)]
#[derive(Debug, Clone)]
pub enum Message {

}

#[derive(Debug, Default)]
pub struct State {

}

impl State {
    pub fn view<'a>(&self, state: &'a crate::State) -> Element<'a, crate::Message> {
        container(
            space()
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style::default()
            .background(theme.extended_palette().background.base.color)
            .border(border::rounded(state.radius))
        )
        .into()
    }
}
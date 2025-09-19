use iced::widget::{center, column, container, slider, text, vertical_slider};
use iced::{Center, Element, Fill};

#[derive(Debug, Clone)]
pub enum Message {
    SliderChanged(u8),
}

pub struct UiState {
    pub value: u8,
}

impl UiState {
    pub fn new() -> Self {
        UiState { value: 50 }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SliderChanged(value) => {
                self.value = value;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let text = text("hello world :3").size(60);

        let text = column![text,]
            .width(Fill)
            .align_x(Center)
            .spacing(20)
            .padding(20);

        center(text).into()
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
}

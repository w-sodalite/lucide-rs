use iced::widget::button::Status;
use iced::widget::{button, center, scrollable, text, Column, Row};
use iced::{application, Alignment, Border, Element, Length, Size, Task, Theme};
use lucide_rs::Lucide;

const PADDING: u32 = 24;

const ICON_PADDING: u32 = PADDING / 4;

const WINDOW_WIDTH: u32 = 760;

const WINDOW_HEIGHT: u32 = 600;

const ICON_SIZE: u32 = 24;

fn main() -> iced::Result {
    application("Preview", Preview::update, Preview::view)
        .font(Lucide::font_data())
        .window(iced::window::Settings {
            size: Size::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
            resizable: false,
            maximized: false,
            ..Default::default()
        })
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Copy(Lucide),
}

#[derive(Default, Debug)]
struct Preview;

impl Preview {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Copy(code) => iced::clipboard::write(code.name().to_string()),
        }
    }

    fn view(&self) -> Element<Message> {
        let size = (WINDOW_WIDTH - PADDING) / (ICON_SIZE + ICON_PADDING * 2 + PADDING);
        center(
            scrollable(
                Column::with_children(
                    Lucide::ALL
                        .chunks(size as usize)
                        .map(|items| {
                            Row::<'_, Message>::with_children(items.iter().copied().map(icon))
                                .spacing(PADDING)
                                .width(Length::Fill)
                                .align_y(Alignment::Center)
                        })
                        .map(Into::into),
                )
                .spacing(PADDING)
                .padding(PADDING as f32)
                .width(Length::Fill)
                .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .into()
    }
}

fn icon<'a>(code: Lucide) -> Element<'a, Message> {
    button(
        text(code)
            .size(ICON_SIZE)
            .line_height(1.0)
            .font(Lucide::FONT),
    )
    .on_press(Message::Copy(code))
    .style(|theme: &Theme, status| {
        let palette = theme.extended_palette();
        let mut base = button::text(theme, status);
        base.border = Border {
            width: 1.0,
            radius: 5.0.into(),
            color: palette.background.strong.color,
        };
        match status {
            Status::Hovered => {
                base.background = Some(theme.extended_palette().primary.base.color.into());
            }
            _ => {
                base.background = Some(palette.background.weakest.color.into());
            }
        }
        base
    })
    .padding(ICON_PADDING as f32)
    .into()
}

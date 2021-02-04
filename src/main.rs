use iced::{
    button, window, Align, Button, Color, Column, Element, HorizontalAlignment, Length, Sandbox,
    Settings, Text, VerticalAlignment,
};

// Базовая структура счётчика.
// Default - позволяет создать пустой экземпляр типа данных.
#[derive(Default)]
struct Counter {
    value: i32, // Значение счётчика

    // Локальные состояния двух кнопок
    increment_button: button::State, // Инкремент
    decrement_button: button::State, // Декремент
}

// Описываем взаимодействие ПО с пользователем.
// Для этого описываем возможные действия пользователя
// в перечислении Message. (Вероятно, правильнее сказать,
// что мы описываем события, которые могут произойти).
// Атрибут derive позволяет модифицировать поведение типажа.
// Debug - позволяет вывести значение через {:?}
// Clone - позволяет создать T из &T с помощью копии
// Copy - позволяет создавать тип копированием, а не перемещением.
// Сложно? Для понимания - да. Но пока что можно просто писать и не париться :)
#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed, // Нажата кнопки инкремента
    DecrementPressed, // Нажата кнопка дикремента
}

// Реализуем типаж Sandbox для нашей структуры Counter
// Что-то типа абстракных классов в C#.
// Типаж Sandbox имеет сигнатуры определённых методов,
// реализацую которых мы описываем для структуры.
impl Sandbox for Counter {
    // Определяем тип сообщения, которое генерит наше приложение, при нажатии на кнопки.
    type Message = Message;

    // Инициализация приложения.
    // Возвращаем исходное состояние нашего приложения.
    fn new() -> Self {
        Self::default()
    }

    // Заголовок нашего приложения (и соответственно окна)
    // Можно менять динамически :)
    fn title(&self) -> String {
        String::from("Counter - Example by Iced (Cool crate!)")
    }

    // Описываем обновление нашего приложения.
    // Именно update определяет поведение приложения при получении сообщения (ну или при наступлении
    // некого события. Например, тык на кнопку).
    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }



    // Описываем виджеты для отображения в нашей приложухе.
    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(25) // Определяем отступ от внешней границы окна до его элементов
            .align_items(Align::Center) // Определяем выравнивание элементов
            .push(
                Button::new(
                    &mut self.increment_button,
                    Text::new("Increment")
                        .size(25)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                )
                .on_press(Message::IncrementPressed)
                .width(Length::Units(200))
                .height(Length::Units(50)), // Описываем кнопку инкремента и вызываемое ей событие
            )
            .push(
                Text::new(self.value.to_string())
                    .size(50)
                    .color(Color::WHITE),
            ) // Описываем отображаемый в окне текст
            .push(
                Button::new(
                    &mut self.decrement_button,
                    Text::new("Decrement")
                        .size(25)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                )
                .on_press(Message::DecrementPressed)
                .width(Length::Units(200))
                .height(Length::Units(50)), // Описываем кнопку декремента и вызываемое ей событие
            )
            .into()
    }

    fn background_color(&self) -> Color {
        Color::from_rgba(0.0, 0.0, 0.0, 0.45)
    }
}

fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (250, 200),
            min_size: None,
            max_size: None,
            resizable: false,
            decorations: true,
            transparent: true,
            always_on_top: false,
            icon: None,
        },
        flags: (),
        default_font: None,
        default_text_size: 25,
        antialiasing: false,
    };

    Counter::run(settings)
}

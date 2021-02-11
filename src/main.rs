use druid::widget::{Button, Flex, Label};
use druid::{
    theme, AppLauncher, Color, Data, Env, LocalizedString, PlatformError, Widget, WidgetExt,
    WindowDesc,
};

const WINDOW_HEIGHT: f64 = 100.0;
const WINDOW_WIDTH: f64 = 100.0;

// Описываем данные нашего приложения
// Атрибут derive позволяет модифицировать поведение типажа.
#[derive(Clone, Data)]
struct Counter {
    data: i32,
}

// Функаия main должа возвращать нам платформозависимую
// ошибку, в случае, если что-то пошло не так.
fn main() -> Result<(), PlatformError> {
    // Инициализируем наше окно. Структура WindowDesc используется
    // для описания параметров окна, которое мы хотим создать. На вход функции
    // new подаём описание "корневого" виджета нашего интерфейса.
    let main_window = WindowDesc::new(ui_builder())
        .title("Test counter")
        .resizable(false)
        .show_titlebar(false)
        .set_position((1785.0, 31.0)) // hardcode под мой монитор
        .window_size((WINDOW_WIDTH, WINDOW_HEIGHT));
    // Описываем начальное состояние данных приложения
    let initial_state: Counter = Counter { data: 0_i32 };
    // Выполняем первоначальную настроку и запускаем приложение
    // К сожалению, на данный момент нельзя задать значение
    // альфа-канала для придания прозрачности окну (точнее можно,
    // но работать не будет).
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .configure_env(|env, _| {
            env.set(theme::WINDOW_BACKGROUND_COLOR, Color::rgb8(62, 62, 62)) // Задаём цвет заливки нашего окна
        })
        .launch(initial_state)
}

// Задаём параметры нашего "корневого" виджета
fn ui_builder() -> impl Widget<Counter> {
    // Описываем текстовое поле, которое будет отображать значение счётчика
    let label = Label::new(|data: &Counter, _env: &Env| (data.data).to_string())
        .padding(5.0)
        .center();
    // Описываем кнопку, которая реализует инкримент
    // в data должна быть мутабельная ссылка, если нужно
    // изменять некие данные
    let inc_button = Button::new("Increment")
        .on_click(|_ctx, data: &mut Counter, _env: &Env| data.data += 1)
        .padding(5.0);
    // Описываем кнопку, которая реализует декремент
    // в data должна быть мутабельная ссылка, если нужно
    // изменять некие данные
    let dec_button = Button::new("Decrement")
        .on_click(|_ctx, data: &mut Counter, _env: &Env| data.data -= 1)
        .padding(5.0);
    // Описываем "сетку" нашего интерфейса
    Flex::column()
        .with_child(inc_button)
        .with_child(label)
        .with_child(dec_button)
}

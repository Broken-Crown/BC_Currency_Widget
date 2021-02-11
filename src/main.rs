use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, Color, theme};


// Функаия main должа возвращать нам платформозависимую
// ошибку, в случае, если что-то пошло не так.
fn main() -> Result<(), PlatformError>{
    // Инициализируем наше окно. Структура WindowDesc используется
    // для описания параметров окна, которое мы хотим создать. На вход функции
    // new подаём описание "корневого" виджета нашего интерфейса.
    let main_window = WindowDesc::new(ui_builder());
    //Объявляем счётчик
    let data = 0_i32;
    // Выполняем первоначальную настроку и запускаем приложение
    // К сожалению, на данный момент нельзя задать значение
    // альфа-канала для придания прозрачности окну (точнее можно,
    // но работать не будет).
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .configure_env(|env, _| {
            env.set(theme::WINDOW_BACKGROUND_COLOR, Color::rgb8(62, 62, 62)) // Задаём цвет заливки нашего окна
        })
        .launch(data)
}

// Задаём параметры нашего "корневого" виджета
fn ui_builder() -> impl Widget<i32> {
    // Описываем строку, которая будет содержать значение переменной date.
    // LocalizedString позволяет сопоставить значение key с переводом для текущей локали (если проще то
    // позволяет подключить локализацию).
    let text = LocalizedString::new("hello-counter").with_arg("count", |data: &i32, _env| (*data).into());
    // Описываем текстовое поле, которое будет отображать значение строки text
    let label = Label::new(text).padding(5.0).center();
    // Описываем кнопку, которая реализует инкримент
    let inc_button = Button::new("Increment")
        .on_click(|_ctx, data, _env| *data += 1)
        .padding(5.0);
    let dec_button = Button::new("Decrement")
        .on_click(|_ctx, data, _env| *data -= 1)
        .padding(5.0);
    // Описываем "сетку" нашего интерфейса
    Flex::column().with_child(inc_button).with_child(label).with_child(dec_button)

}
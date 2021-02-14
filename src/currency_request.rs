use std::collections::HashMap;

/// URL по которому мы обращаемся к API.
/// На данный момент не планируется добавление
/// возможности настройки URL из интерфейа.
const URL: &str = "https://httpbin.org/ip";

/// Функция, которая возвращает курс конкретной валюты.
///
/// # Аргументы
/// * `currency` - название конкретной валюты.
///
/// **NOTE:** Название валюты должно соответствовать названию соответствующей валюты в ответе API.
/// (В данном случае используется API ЦБ РФ).
///
/// **TODO:** Неплохо было бы отрефакторить эту жуть.
///
/// # Пример
/// ```
/// let mut eur_rub: String = currency_request::get_currency(eur)
/// ```
///
/// # Ошибки
/// Функция возвращает ошибки в случае:
/// * Отсутсвия в ответе API запрашиваемой валюты
/// * Ошибки сервера
/// * Ошибки обработки ответа от сервера
pub fn get_currency(currency: &str) -> String {
    let mut response_json: HashMap<String, String> = send_request(URL);
    let mut response_value = String::new();
    if response_json.contains_key("Error") {
        response_value = match response_json.get("Error") {
            Some(text) => text.to_string(),
            None => "Key not found".to_string(), // Если честно, то я не знаю, как в данном случае может получиться None
        };
        return response_value;
    }
    response_value = match response_json.get(currency) {
        Some(text) => text.to_string(),
        None => "Key not found".to_string(),
    };
    response_value
}

/// Функция выполяет get-запрос по API и возвращает ответ от сервера в виде `HashMap<String, String>`.
///
/// **NOTE:** Функция в разработке! На данный момент работа осуществляется только с API, которые возвращают JSON.
/// ЦБ РФ (на которую нацелено данное ПО) возвращает DataFrame или XML.
///
/// **TODO:** Неплохо было бы отрефакторить эту жуть.
///
/// # Аргументы
/// * `url` - URL, по которому выполняется запрос
///
/// # Пример
/// ```
/// let mut response_json: HashMap<String, String> = send_request(URL);
/// ```
///
/// # Ошибки
/// Функция возвращает ошибки в случае:
/// * Ошибки сервера
/// * Ошибки обработки ответа от сервера
fn send_request(url: &str) -> HashMap<String, String> {
    let response = reqwest::blocking::get(url);
    let mut response_json: HashMap<String, String> = HashMap::new();
    response_json = match response {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<HashMap<String, String>>() {
                    Ok(json) => json,
                    Err(error) => [("Error".to_string(), format!("Parse error: {:?}", error))]
                        .iter()
                        .cloned()
                        .collect(), // Сие "короткое" решение нашёл в документации Rust'а.
                                    // Другого пособа создать HashMap фиксированного размера,
                                    // видимо, нет.
                }
            } else if response.status().is_server_error() {
                [(
                    "Error".to_string(),
                    format!("Server error: {:?}", response.status()),
                )]
                .iter()
                .cloned()
                .collect()
            } else {
                [(
                    "Error".to_string(),
                    format!("Something else happened: {:?}", response.status()),
                )]
                .iter()
                .cloned()
                .collect()
            }
        }
        Err(error) => [(
            "Error".to_string(),
            format!("Something was wrong: {:?}", error),
        )]
        .iter()
        .cloned()
        .collect(),
    };
    return response_json;
}

use rocket::response::content::RawHtml;
use rocket::response::content::RawCss;
use rocket::response::content::RawJavaScript;
use std::fs;
use rocket::get;


#[get("/login")]
pub fn login() -> RawHtml<String> {
    let content = fs::read_to_string("static/login.html").expect("Unable to read file");
    RawHtml(content)
}

#[get("/login.css")]
pub fn login_css() -> RawCss<String> {
    let content = fs::read_to_string("static/css/login.css").expect("Unable to read file");
    RawCss(content)
}

#[get("/login.js")]
pub fn login_js() -> RawJavaScript<&'static str> {
    let content = r#"
// Динамическая загрузка скрипта
const loadAlertScript = () => {
  return new Promise((resolve, reject) => {
    const script = document.createElement('script');
    script.src = 'XP_alert.js';
    script.onload = resolve;
    script.onerror = reject;
    document.head.appendChild(script);
  });
};

// Основной код приложения
const initApp = async () => {
  try {
    // Сначала загружаем скрипт
    await loadAlertScript();
    
    // Затем инициализируем форму
    document.getElementById('loginForm').addEventListener('submit', async function(event) {
      event.preventDefault();
      
      try {
        const response = await fetch('/api/user_login', {
          method: 'POST',
          headers: {'Content-Type': 'application/json'},
          body: JSON.stringify({
            username: event.target.username.value,
            password: event.target.password.value,
            role: event.target.role.value,
          }),
        });

        if (!response.ok) {
          const errorText = await response.text()
          alert(`Ошибка сервера: ${errorText}`);
          return;
        }

        const data = await response.json();
        localStorage.setItem('authToken', data.token);
        window.location.href = '/';
      } catch (error) {
        console.error('Ошибка:', error);
        alert('Ошибка входа. Проверьте учетные данные.');
      }
    });

  } catch (error) {
    console.error('Не удалось загрузить XP_alert.js:', error);
  }
};

// Запускаем приложение
initApp();
    "#;
    RawJavaScript(content)
}

#[get("/loginCheck.js")]
pub fn login_check_js() -> RawJavaScript<&'static str> {
    let content = r#"
        // Проверяем наличие токена в localStorage
        const authToken = localStorage.getItem('authToken');

        if (!authToken) {
          // Если токена нет, перенаправляем на страницу входа
          window.location.href = '/login';
        } else {
          // Отправляем запрос к API для проверки токена
          fetch('/api/check_token', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json'
            },
            body: JSON.stringify({ 
              token: authToken
            })
          })
          .then(response => {
            if (!response.ok) {
              throw new Error('Ответ сети был неудовлетворительным:');
            }
            return response.json();
            window.location.href = '/login';
          })
          .then(data => {
            if (!data.token_valid) {
              // Если токен не валиден, перенаправляем на страницу входа
              window.location.href = '/login';
            }
          })
          .catch(error => {
            console.error('Возникла проблема с операцией выборки:', error);
            // В случае ошибки также перенаправляем на страницу входа
            window.location.href = '/login';
          });
        }
    "#;
    RawJavaScript(content)
}

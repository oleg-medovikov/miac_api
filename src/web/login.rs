use rocket::response::content::RawHtml;
use rocket::response::content::RawCss;
use rocket::response::content::RawJavaScript;
use rocket::get;


#[get("/login")]
pub fn login() -> RawHtml<String> {
    let content = r#"
<!DOCTYPE html>
<html lang="ru">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Вход в систему</title>
  <link rel="stylesheet" href="https://unpkg.com/xp.css">
  <link rel="stylesheet" href="login.css"> <!-- Подключение login.css -->
</head>
<body>
  <div class="login-container" id="loginContainer">
    <div class="window">
      <div class="title-bar" id="titleBar">
        <div class="title-bar-text">Вход в систему</div>
        <div class="title-bar-controls">
          <button aria-label="Minimize" id="minimizeBtn"></button>
          <button aria-label="Maximize" id="maximizeBtn"></button>
          <button aria-label="Close" id="closeBtn"></button>
        </div>
      </div>
      <div class="window-body" id="windowBody">
        <form id="loginForm">
          <div class="field-row">
            <label for="username">Логин:</label>
            <input type="text" id="username" class="full-width"/>
          </div>
          <div class="field-row">
            <label for="password">Пароль:</label>
            <input type="password" id="password" class="full-width"/>
          </div>
          <div class="field-row">
            <label for="role">Роль:</label>
            <select id="role">
              <option>Пользователь</option>
              <option>Администратор</option>
              <option>Гость</option>
            </select>
          </div>
          <button type="submit" class="login-btn">Войти</button>
        </form>
      </div>
    </div>
  </div>

  <script src="/XP_alert.js"></script> <!-- Импорт XP_alert.js -->
  <script>
    document.getElementById('minimizeBtn').addEventListener('click', function() {
      const windowBody = document.getElementById('windowBody');
      windowBody.classList.toggle('collapsed');
    });

    document.getElementById('maximizeBtn').addEventListener('click', function() {
      const loginContainer = document.querySelector('.login-container');
      loginContainer.classList.toggle('fullscreen');
    });

    document.getElementById('closeBtn').addEventListener('click', function() {
      const loginContainer = document.querySelector('.login-container');
      loginContainer.classList.add('hidden');
      // Вызов XP_alert для отображения ошибки
      alert("NET_SendPacket ERROR: NO ERROR");
    });

    const titleBar = document.getElementById('titleBar');
    const loginContainer = document.getElementById('loginContainer');

    titleBar.addEventListener('mousedown', function(e) {
      let offsetX = e.clientX - loginContainer.offsetLeft;
      let offsetY = e.clientY - loginContainer.offsetTop;

      function mouseMoveHandler(e) {
        loginContainer.style.left = `${e.clientX - offsetX}px`;
        loginContainer.style.top = `${e.clientY - offsetY}px`;
      }

      function mouseUpHandler() {
        document.removeEventListener('mousemove', mouseMoveHandler);
        document.removeEventListener('mouseup', mouseUpHandler);
      }

      document.addEventListener('mousemove', mouseMoveHandler);
      document.addEventListener('mouseup', mouseUpHandler);
    });
  </script>

  <script type="module" src="login.js"></script>
</body>
</html>
    "#;
    RawHtml(content.to_string())
}

#[get("/login.css")]
pub fn login_css() -> RawCss<String> {
    let content =r#"
body {
  background: #008080;
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  margin: 0;
  font-family: 'MS Sans Serif', sans-serif;
}

.login-container {
  width: 320px;
  transition: width 0.3s ease, height 0.3s ease;
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.login-container.fullscreen {
  width: 100vw;
  height: 100vh;
  margin: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  top: 0;
  left: 0;
  transform: none;
}

.login-container.fullscreen .window {
  width: 100%;
  height: 100%;
  margin: 0;
}

.login-container.hidden {
  display: none;
}

.window {
  margin: 20px;
  transition: height 0.3s ease;
}

.window-body {
  display: flex;
  flex-direction: column;
  overflow: hidden;
  transition: max-height 0.3s ease;
}

.window-body.collapsed {
  max-height: 0;
  padding: 0;
  border: none;
}

.field-row {
  display: flex;
  align-items: center;
  margin: 10px 0;
}

.field-row label {
  width: 100px;
  margin-right: 10px;
}

.field-row input,
.field-row select {
  flex: 1;
  padding: 3px;
}

.login-btn {
  width: 100%;
  margin-top: 15px;
}

.title-bar {
  cursor: move;
}
    "#; 
    RawCss(content.to_string())
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

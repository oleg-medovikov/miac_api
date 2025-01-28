use rocket::response::content::RawJavaScript;
use rocket::get;

#[get("/XP_alert.js")]
pub fn xp_alert_js() -> RawJavaScript<&'static str> {
    let content = r#"
(() => {
  // Подключаем XP.css
  const xpCSS = document.createElement('link');
  xpCSS.rel = 'stylesheet';
  xpCSS.href = 'https://unpkg.com/xp.css';
  document.head.appendChild(xpCSS);

  // Добавляем кастомные стили
  const style = document.createElement('style');
  style.textContent = `
    .xp-modal { 
      position: fixed; 
      top: 50%; 
      left: 75%; 
      transform: translate(-50%, -50%); 
      z-index: 9999; 
    }
    .xp-modal-overlay { 
      position: fixed; 
      top: 0; 
      left: 0; 
      width: 100%; 
      height: 100%; 
      background: rgba(0,0,0,0.5); 
      display: flex; 
      justify-content: center; 
      align-items: center;
    }
    .xp-error-content {
      display: flex;
      align-items: center;
      gap: 15px;
      padding: 15px !important;
    }
    .xp-error-icon {
      width: 32px;
      height: 32px;
      flex-shrink: 0;
    }
    .xp-status-bar {
      padding: 8px !important;
    }
  `;
  document.head.appendChild(style);

  // Иконка ошибки в base64
  const errorIcon = '/image/error.png'

  // Переопределяем стандартный alert
  window.alert = (message) => {
    // Создаем HTML структуру
    const modal = document.createElement('div');
    modal.className = 'xp-modal-overlay';
    modal.innerHTML = `
      <div class="xp-modal">
        <div class="window" style="width: 300px">
          <div class="title-bar">
            <div class="title-bar-text">Сообщение</div>
            <div class="title-bar-controls">
              <button aria-label="Close" onclick="this.closest('.xp-modal-overlay').remove()"></button>
            </div>
          </div>
          <div class="window-body xp-error-content">
            <img class="xp-error-icon" src="${errorIcon}">
            <p>${message}</p>
          </div>
          <div class="status-bar xp-status-bar">
            <div class="status-bar-field" style="cursor: pointer; width: 100%; text-align: center" 
                 onclick="this.closest('.xp-modal-overlay').remove()">
              OK
            </div>
          </div>
        </div>
      </div>
    `;

    document.body.appendChild(modal);

    // Закрытие по клику вне окна
    modal.addEventListener('click', (e) => {
      if (e.target === modal) modal.remove();
    });
  };
})();
    "#;

    RawJavaScript(content)
}

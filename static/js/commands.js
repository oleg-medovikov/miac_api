// Проверяем наличие токена в localStorage
if (!localStorage.getItem('authToken')) {
  // Если токена нет, перенаправляем на страницу входа
  window.location.href = 'login.html';
} else {
  // Отправляем запрос к API для проверки токена
  fetch(`/check_token`, {
    method: 'GET',
    headers: {
      'Authorization': localStorage.getItem('authToken')
    }
  })
  .then(response => {
    if (!response.ok) {
      throw new Error('Network response was not ok');
    }
    return response.json();
  })
  .then(data => {
    if (!data.token_valid) {
      // Если токен не валиден, перенаправляем на страницу входа
      window.location.href = 'login.html';
    } else { // Если токен валидный - мы вытаскиваем пользователя
      fetch(`/user_get`, { 
        method: 'GET',
        headers: {
          'Authorization': localStorage.getItem('authToken')
        }
      })
      .then(response => {
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        return response.json(); // Преобразуем тело ответа в JSON
      })
      .then(data => {  
        localStorage.setItem('fio', data.fio); 
        localStorage.setItem('groups', data.groups);
        document.getElementById('UserFIO').innerHTML = localStorage.getItem('fio');
        document.getElementById('UserGroups').innerHTML = localStorage.getItem('groups');
      })
      .catch(error => {
        console.error('There has been a problem with your fetch operation:', error);
      });
    }
  })
  .catch(error => {
    console.error('There was a problem with the fetch operation:', error);
  });
}

// Отправляем запрос к API
fetch(`/command_get_all`, {
  method: 'GET',
  headers: {
    'Authorization': localStorage.getItem('authToken')
  }
})
.then(response => response.json()) 
.then(data => {
  // Создаем таблицу команд
  const commandTable = document.createElement('table');
  commandTable.innerHTML = `
    <tr>
      <th>Категория</th>
      <th>Название</th>
      <th>Функция</th>
      <th>Аргументы</th>
      <th>Возврат файлов</th>
      <th>Спросить день</th>
      <th>Описание</th>
      <th>Активный</th>
      <th>Действие</th>
    </tr>
  `;
  // Добавляем строки с данными пользователей
  data.forEach(command => {
    const row = document.createElement('tr');
    row.innerHTML = `
      <td>${command.category}</td>
      <td>${command.name}</td>
      <td>${command.func}</td>   
      <td>${command.arg}</td>
      <td>${command.return_file ? 'Да' : 'Нет'}</td>
      <td>${command.ask_day ? 'Да' : 'Нет'}</td>
      <td>${command.description}</td>
      <td>${command.active ? 'Да' : 'Нет'}</td>
      <td><button class="edit-command" data-guid="${command.guid}">Редактировать</button></td>
    `;
    commandTable.appendChild(row);
  });
  // Отображаем таблицу пользователей
  const commandTableContainer = document.getElementById('commandTableContainer');
  commandTableContainer.innerHTML = ''; // Очищаем предыдущее содержимое
  commandTableContainer.appendChild(commandTable);
  // Добавляем обработчик события для кнопок редактирования
  const editButtons = document.querySelectorAll('.edit-command');
  editButtons.forEach(button => {
    button.addEventListener('click', function() {
      const guid = this.dataset.guid;
      // Находим полный объект пользователя по GUID
      const command = data.find(command => command.guid === guid);
      // Открываем модальное окно с данными пользователя
      openEditCommandModal(command);
    });   
  });
})
.catch(error => {
  console.error('Ошибка при загрузке пользователей:', error);
});

// Объявляем переменную modal в глобальной области видимости
let modal;

// Функция для открытия модального окна
function openEditCommandModal(command) {
  // Получаем модальное окно
  modal = document.getElementById('editCommandModal');

  // Проверяем, существует ли command
  if (command) {
    // Заполняем поля формы данными пользователя
    document.getElementById('guid').value = command.guid || '';
    document.getElementById('category').value = command.category || '';
    document.getElementById('name').value = command.name || '';
    document.getElementById('func').value = command.func || '';
    document.getElementById('arg').value = command.arg || '';
    document.getElementById('return_file').value = command.return_file ? 'true' : 'false';
    document.getElementById('ask_day').value = command.ask_day ? 'true' : 'false';
    document.getElementById('description').value = command.description || '';
    document.getElementById('active').value = command.active ? 'true' : 'false';
    document.getElementById('updateModalButton').style.display = 'block';
    document.getElementById('CommandModalTitle').innerHTML = 'Редактировать'
  } else {
    // Если command не определен, очищаем поля формы
    document.getElementById('guid').value = '';
    document.getElementById('category').value = '';
    document.getElementById('name').value = '';
    document.getElementById('func').value = '';
    document.getElementById('arg').value = '';
    document.getElementById('return_file').value = '';
    document.getElementById('ask_day').value = '';
    document.getElementById('description').value = '';
    document.getElementById('active').value = ''; 
    document.getElementById('createModalButton').style.display = 'block';
    document.getElementById('CommandModalTitle').innerHTML = 'Добавить команду'
  }
  // Отображаем модальное окно
  modal.style.display = 'block';
}

// Получаем кнопку закрытия модального окна
let closeBtn = document.getElementById("closeModalButton");
// Добавляем обработчик события на кнопку закрытия
closeBtn.addEventListener("click", function() {
  // Скрываем модальное окно
  modal.style.display = "none";
});

// Получаем кнопку обновления команды
let okBtn = document.getElementById("updateModalButton");
okBtn.addEventListener("click", function() {
  // Получаем данные из формы
  let guid = document.getElementById('guid').value;
  let category = document.getElementById('category').value;
  let name = document.getElementById('name').value;
  let func = document.getElementById('func').value;
  let arg = document.getElementById('arg').value;
  let return_file = document.getElementById('return_file').value === 'true';
  let ask_day = document.getElementById('ask_day').value === 'true';
  let description = document.getElementById('description').value;
  let active = document.getElementById('active').value === 'true';

  // Создаем объект с данными
  let data = {
    guid: guid,
    category: category,
    name: name, 
    func: func,                 
    arg: arg,
    return_file: return_file,
    description: description,
    ask_day: ask_day,
    active: active
  };

  // Отправляем запрос
  fetch(`/command_update`, {
    method: 'POST',
    headers: {
      'Content-Type':'application/json',
      'Authorization': localStorage.getItem('authToken')
    },
    body: JSON.stringify(data)
  })
  .then(response => response.json())
  .then(data => {
    console.log('Успешно:', data);
    // Здесь можно обработать ответ от сервера, например, обновить интерфейс пользователя
    modal.style.display = "none"; 
    location.reload();
  });
});

// Получаем кнопку открытия для создания пользователя
let addBtn = document.getElementById("addCommandModal");
// Добавляем обработчик события на кнопку закрытия
addBtn.addEventListener("click", function() {
  openEditCommandModal();
});

// Создание нового пользователя
let createBtn = document.getElementById("createModalButton");
createBtn.addEventListener("click", function() {
  // Получаем данные из формы
  let category = document.getElementById('category').value;
  let name = document.getElementById('name').value;
  let func = document.getElementById('func').value;
  let arg = document.getElementById('arg').value;
  let return_file = document.getElementById('return_file').value === 'true';
  let ask_day = document.getElementById('ask_day').value === 'true';
  let description = document.getElementById('description').value;
  let active = document.getElementById('active').value === 'true';

  // Проверяем, заполнены ли все поля
  if (!category || !name || !func || !arg || !return_file || !description || !ask_day) {
    alert('Все поля должны быть заполнены.');
    return; // Прекращаем выполнение функции, если какое-либо поле не заполнено
  }

  // Создаем объект с данными
  let data = {
    category: category,
    name: name,
    func: func,
    arg: arg,
    return_file: return_file,
    description: description,
    ask_day: ask_day,
    active: active
  };

  // Отправляем запрос
  fetch(`/command_create`, {
    method: 'POST',
    headers: {
      'Content-Type':'application/json',
      'Authorization': localStorage.getItem('authToken')
    },
    body: JSON.stringify(data)
  })
  .then(response => response.json())
  .then(data => {
    console.log('Успешно:', data);
    // Здесь можно обработать ответ от сервера, например, обновить интерфейс пользователя
    modal.style.display = "none"; 
    location.reload();
  });
});

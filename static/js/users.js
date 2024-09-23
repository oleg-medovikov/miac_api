// Проверяем наличие токена в localStorage
if (!localStorage.getItem('authToken')) {
  // Если токена нет, перенаправляем на страницу входа
  window.location.href = '/login.html';
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
      window.location.href = '/login.html';
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


fetch(`/user_get_all`, {
  method: 'GET',
  headers: {
    'Authorization': localStorage.getItem('authToken')
  }
})
  .then(response => response.json()) 
  .then(data => {
  // Создаем таблицу пользователей
  const usersTable = document.createElement('table');
  usersTable.innerHTML = `
    <tr> 
      <th>ID в Telegram</th>
      <th>Имя пользователя</th>
      <th>Фамилия</th>
      <th>Группы</th>
      <th>Описание</th>
      <th>Активный</th>
      <th>Действие</th>
    </tr>
  `;
  // Добавляем строки с данными пользователей
  data.forEach(user => {
    const row = document.createElement('tr');
    row.innerHTML = `
      <td>${user.tg_id}</td>
      <td>${user.username}</td>
      <td>${user.fio}</td>
      <td>${user.groups}</td>
      <td>${user.description}</td>
      <td>${user.active ? 'Да' : 'Нет'}</td>
      <td><button class="edit-user" data-guid="${user.guid}">Редактировать</button></td>
    `;
    usersTable.appendChild(row);
  });
  // Отображаем таблицу пользователей
  const usersTableContainer = document.getElementById('usersTableContainer');
  usersTableContainer.innerHTML = ''; // Очищаем предыдущее содержимое
  usersTableContainer.appendChild(usersTable);
  // Добавляем обработчик события для кнопок редактирования
  const editButtons = document.querySelectorAll('.edit-user');
  editButtons.forEach(button => {
    button.addEventListener('click', function() {
      const guid = this.dataset.guid;
      // Находим полный объект пользователя по GUID
      const user = data.find(user => user.guid === guid);
      // Открываем модальное окно с данными пользователя
      openEditUserModal(user);
    });   
  });
 })


  .catch(error => {
    console.error('Ошибка при загрузке пользователей:', error);
  });
  


  // Объявляем переменную modal в глобальной области видимости
var modal;

// Функция для открытия модального окна
function openEditUserModal(user) {
  // Получаем модальное окно
  modal = document.getElementById('editUserModal');

if (user) {
  // Заполняем поля формы данными пользователя
  document.getElementById('guid').value = user.guid || '';
  document.getElementById('tg_id').value = user.tg_id || '';
  document.getElementById('username').value = user.username || '';
  document.getElementById('fio').value = user.fio || '';
  document.getElementById('groups').value = user.groups || '';
  document.getElementById('description').value = user.description || '';
  document.getElementById('active').value = user.active ? 'true' : 'false';
  document.getElementById('updateModalButton').style.display = 'block';
  document.getElementById('UserModalTitle').innerHTML = 'Редактировать'
} else {
  // Если user не определен, очищаем поля формы
  document.getElementById('guid').value = '';
  document.getElementById('tg_id').value = '';
  document.getElementById('username').value = '';
  document.getElementById('fio').value = '';
  document.getElementById('groups').value = '';
  document.getElementById('description').value = '';
  document.getElementById('active').value = ''; 
  document.getElementById('createModalButton').style.display = 'block';
  document.getElementById('UserModalTitle').innerHTML = 'Добавить пользователя'
}
  // Отображаем модальное окно
  modal.style.display = 'block';
}


// Получаем кнопку закрытия модального окна
var closeBtn = document.getElementById("closeModalButton");
  // Добавляем обработчик события на кнопку закрытия
closeBtn.addEventListener("click", function() {
  // Скрываем модальное окно
  modal.style.display = "none";
});


var okBtn = document.getElementById("updateModalButton");
okBtn.addEventListener("click", function() {
 // Получаем данные из формы
  var guid = document.getElementById('guid').value;
  var tg_id = document.getElementById('tg_id').value;
  var username = document.getElementById('username').value;
  var fio = document.getElementById('fio').value;
  var groups = document.getElementById('groups').value;
  var description = document.getElementById('description').value;
  var active = document.getElementById('active').value === 'true';

  // Создаем объект с данными
  var data = {
    guid: guid,
    tg_id: parseInt(tg_id),
    username: username,
    fio: fio,
    groups: groups,
    description: description,
    active: active
  };

  // Отправляем запрос
  fetch(`/user_update`, {
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
var addBtn = document.getElementById("addUserModal");
  // Добавляем обработчик события на кнопку закрытия
addBtn.addEventListener("click", function() {
  openEditUserModal();
});


//Создание нового пользователя
var okBtn = document.getElementById("createModalButton");
okBtn.addEventListener("click", function() {
 // Получаем данные из формы
  var tg_id = document.getElementById('tg_id').value;
  var username = document.getElementById('username').value;
  var fio = document.getElementById('fio').value;
  var groups = document.getElementById('groups').value;
  var description = document.getElementById('description').value;
  var active = document.getElementById('active').value === 'true';

   // Проверяем, заполнены ли все поля
   if (!tg_id || !username || !fio || !groups || !description) {
    alert('Все поля должны быть заполнены.');
    return; // Прекращаем выполнение функции, если какое-либо поле не заполнено
  }

  // Создаем объект с данными
  var data = {
    tg_id: parseInt(tg_id),
    username: username,
    fio: fio,
    groups: groups,
    description: description,
    active: active
  };

  // Отправляем запрос
  fetch(`/user_create`, {
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


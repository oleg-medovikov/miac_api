
document.getElementById('logoutButton').addEventListener('click', function(event) {
  event.preventDefault(); // Предотвращает стандартное действие кнопки (например, переход по ссылке)
  
  if (confirm('Вы уверены, что хотите выйти?')) {
    // Если пользователь нажал "ОК"
    console.log('Выход из системы');

    // Отправляем запрос на сервер для удаления токена
    fetch(`/drop_token`, {
      method: 'POST',
      headers: {
        'Authorization': localStorage.getItem('authToken')
      }
    })
    .then(response => {
      if (!response.ok) {
        throw new Error('Network response was not ok');
      }
      localStorage.removeItem('authToken');
      window.location.href = 'login.html';
    })
    .catch(error => {
      console.error('There has been a problem with your fetch operation:', error);
    });
  } else {
    // Если пользователь нажал "Отменить", ничего не происходит
    console.log('Отмена выхода');
  }
});

function checkAuthToken() {
  if (!localStorage.getItem('authToken')) {
    window.location.href = 'login.html';
    return false;
  }
  return true;
}

function fetchWithToken(url, options = {}) {
  return fetch(url, {
    ...options,
    headers: {
      ...options.headers,
      'Authorization': localStorage.getItem('authToken')
    }
  });
}

function handleUserInfo() {
  fetchWithToken(`check_token`, { method: 'GET' })
    .then(response => {
      if (!response.ok) throw new Error('Token check failed');
      return response.json();
    })
    .then(data => {
      if (!data.token_valid) {
        window.location.href = 'login.html';
      } else {
        return fetchWithToken(`user_get`, { method: 'GET' });
      }
    })
    .then(response => {
      if (!response.ok) throw new Error('Failed to fetch user info');
      return response.json();
    })
    .then(data => {
      localStorage.setItem('fio', data.fio);
      localStorage.setItem('groups', data.groups);
      document.getElementById('UserFIO').innerHTML = localStorage.getItem('fio');
      document.getElementById('UserGroups').innerHTML = localStorage.getItem('groups');
    })
    .catch(error => {
      console.error('Error:', error);
    });
}

function loadDirectories() {
  fetchWithToken(`dir_get_all`, { method: 'GET' })
    .then(response => {
      if (!response.ok) throw new Error('Failed to fetch directories');
      return response.json();
    })
    .then(data => {
      const dirTable = document.createElement('table');
      dirTable.innerHTML = `
        <tr>
          <th>Имя</th>
          <th>Директория</th>
          <th>Описание</th>
          <th>Активный</th>
          <th>Действие</th>
        </tr>
      `;

      data.forEach(dir => {
        const row = document.createElement('tr');
        row.innerHTML = `
          <td>${dir.name}</td>
          <td>${dir.directory}</td>
          <td>${dir.description}</td>
          <td>${dir.active ? 'Да' : 'Нет'}</td>
          <td><button class="edit-dir" data-guid="${dir.guid}">Редактировать</button></td>
        `;
        dirTable.appendChild(row);
      });

      const dirTableContainer = document.getElementById('dirTableContainer');
      if (dirTableContainer) {
        dirTableContainer.innerHTML = '';
        dirTableContainer.appendChild(dirTable);
      }

      const editButtons = document.querySelectorAll('.edit-dir');
      editButtons.forEach(button => {
        button.addEventListener('click', function() {
          const guid = this.dataset.guid;
          const dir = data.find(dir => dir.guid === guid);
          setTimeout(() => openEditDirModal(dir), 100); // Добавляем задержку
        });
      });
    })
    .catch(error => {
      console.error('Error loading directories:', error);
    });
}

function openEditDirModal(dir) {
  const modal = document.getElementById('editDirModal');
  if (!modal) {
    console.error('Modal not found');
    return;
  }

  const guidElement = document.getElementById('guid');
  const nameElement = document.getElementById('name');
  const directoryElement = document.getElementById('directory');
  const descriptionElement = document.getElementById('description');
  const activeElement = document.getElementById('active');

  if (!guidElement || !nameElement || !directoryElement || !descriptionElement || !activeElement) {
    console.error('One of the modal elements not found');
    return;
  }

  if (dir) {
    guidElement.value = dir.guid || '';
    nameElement.value = dir.name || '';
    directoryElement.value = dir.directory || '';
    descriptionElement.value = dir.description || '';
    activeElement.value = dir.active ? 'true' : 'false';
    document.getElementById('updateModalButton').style.display = 'block';
    document.getElementById('dirModalTitle').innerHTML = 'Редактировать';
  } else {
    guidElement.value = '';
    nameElement.value = '';
    directoryElement.value = '';
    descriptionElement.value = '';
    activeElement.value = '';
    document.getElementById('createModalButton').style.display = 'block';
    document.getElementById('dirModalTitle').innerHTML = 'Добавить директорию';
  }

  modal.style.display = 'block';
}

function setupModalButtons() {
  const closeBtn = document.getElementById("closeModalButton");
  if (closeBtn) {
    closeBtn.addEventListener("click", function() {
      const modal = document.getElementById('editDirModal');
      if (modal) modal.style.display = "none";
    });
  }

  const okBtn = document.getElementById("updateModalButton");
  if (okBtn) {
    okBtn.addEventListener("click", function() {
      const guidElement = document.getElementById('guid');
      const nameElement = document.getElementById('name');
      const directoryElement = document.getElementById('directory');
      const descriptionElement = document.getElementById('description');
      const activeElement = document.getElementById('active');

      if (!guidElement || !nameElement || !directoryElement || !descriptionElement || !activeElement) {
        console.error('One of the elements not found');
        return;
      }

      const data = {
        guid: guidElement.value,
        name: nameElement.value,
        directory: directoryElement.value,
        description: descriptionElement.value,
        active: activeElement.value === 'true'
      };

      fetchWithToken(`dir_update`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
      })
      .then(response => {
        if (!response.ok) throw new Error('Update failed');
        return response.json();
      })
      .then(data => {
        console.log('Success:', data);
        const modal = document.getElementById('editDirModal');
        if (modal) modal.style.display = "none";
        location.reload();
      })
      .catch(error => {
        console.error('Error:', error);
      });
    });
  }

  const addBtn = document.getElementById("addDirModal");
  if (addBtn) {
    addBtn.addEventListener("click", function() {
      openEditDirModal();
    });
  }

  const createBtn = document.getElementById("createModalButton");
  if (createBtn) {
    createBtn.addEventListener("click", function() {
      const name = document.getElementById('name').value;
      const directory = document.getElementById('directory').value;
      const description = document.getElementById('description').value;
      const active = document.getElementById('active').value === 'true';

      if (!name || !directory || !description) {
        alert('All fields must be filled.');
        return;
      }

      const data = {
        name: name,
        directory: directory,
        description: description,
        active: active
      };

      fetchWithToken(`dir_create`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
      })
      .then(response => {
        if (!response.ok) throw new Error('Create failed');
        return response.json();
      })
      .then(data => {
        console.log('Success:', data);
        const modal = document.getElementById('editDirModal');
        if (modal) modal.style.display = "none";
        location.reload();
      })
      .catch(error => {
        console.error('Error:', error);
      });
    });
  }
}

function exportTableToPDF() {
  const doc = new jspdf.jsPDF();
  const table = document.getElementById('dirTableContainer');
  if (table) {
    const res = doc.autoTableHtmlToJson(table);
    doc.autoTable(res.columns, res.data);
    doc.save('directories.pdf');
  } else {
    console.error('Table not found');
  }
}

document.addEventListener('DOMContentLoaded', function() {
  if (checkAuthToken()) {
    handleUserInfo();
    loadDirectories();
    setupModalButtons();
  }
});

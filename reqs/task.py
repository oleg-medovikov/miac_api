from .app import app
from typing import Optional
from fastapi import Header, Body

from clas import User, Task, Access
from conf import TOKEN

@app.post("/add_task", tags=["tasks"],)
async def add_task(
        KEY : Optional[set] = Header(None),
        U_ID: Optional[int] = Header(None),
        TASK: Task = Body(None)):
    "Добавим задачу в очередь на выполнение"
    if U_ID is None \
        or TASK is None \
        or KEY != TOKEN:
        return None

    if not await Access.cheak( U_ID, C_ID ):
        return {'mess': 'Недостаточно прав'}

    return await TASK.add()


@app.get("/get_task", tags=["tasks"],)
async def get_task(
        KEY: Optional[str] = Header(None)):
    """Выбрать существующую задачу из очереди
    у которой время начала пустое и начать её выполнять"""
    if KEY != TOKEN:
        return None

    return await Task.get()

@app.post("/stop_task", tags=["tasks"],)
async def stop_task(
        KEY: Optional[str] = Header(None),
        TASK: Task = Body(None)):
    """Закончить выполнять задачу
    проставить время окончания и комментарий"""
    if  KEY != TOKEN:
        return None

    return await Task.stop()

@app.post("/restart_tasks", tags=["tasks"],)
async def restart_tasks(
        KEY: Optional[str] = Header(None)):
    """Обнулить время начала у всех незаконченных задач
    чтобы их можно было заново брать в работу. 
    Делается при рестарте исполнителя, после его падения"""
    if KEY != TOKEN:
        return None

    return await Task.restart()


@app.post("/get_users_list", tags=["tasks"],)
async def get_users_list(
        KEY: Optional[str] = Header(None),
        TASK: Task = Body(None)):
    """Получить для существующей задачи список 
    пользователей для рассылки.
    Он может измениться за время выполнения задачи"""
    if KEY != TOKEN or TASK is None:
        return None

    return await TASK.users()



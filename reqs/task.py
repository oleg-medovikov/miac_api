from .app import app
from typing import Optional
from fastapi import Header, Body

from clas import User, Task, Access
from conf import TOKEN

@app.post("/add_task", tags=["tasks"],)
async def add_task(
        U_ID: Optional[int] = Header(None),
        TASK: Task = Body(None)):
    "Добавим задачу"
    if U_ID is None or TASK is None:
        return None

    if not await Access.cheak( U_ID, C_ID ):
        return {'mess': 'Недостаточно прав'}

    return await TASK.add()


@app.get("/get_task", tags=["tasks"],)
async def get_task(
        Authorization: Optinal[str] = Header(None)):
    """Выбрать существующую задачу
    у которой время начала пустое и начать выполнять"""
    if not Authorization == TOKEN:
        return None

    return await Task.get()

@app.post("/stop_task", tags=["tasks"],)
async def stop_task(
        Authorization: Optinal[str] = Header(None),
        TASK: Task = Body(None)):
    """Закончить выполнять задачу
    проставить время окончания и комментарий"""
    if not Authorization == TOKEN:
        return None

    return await Task.stop()

@app.post("/restart_tasks", tags=["tasks"],)
async def restart_tasks(
        Authorization: Optinal[str] = Header(None)):
    """Обнулить время начала у всех незаконченных задач"""
    if not Authorization == TOKEN:
        return None

    return await Task.stop()


@app.post("/get_users", tags=["tasks"],)
async def get_users(
        Authorization: Optinal[str] = Header(None),
        TASK: Task = Body(None)):
    """Получить для существующей задачи список 
    пользователей для рассылки"""
    if not Authorization == TOKEN:
        return None

    return await Task.users()



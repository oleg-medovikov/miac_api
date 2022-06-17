from .app import app
from typing import Optional
from fastapi import Header, Body

from clas import User
from conf import TOKEN


@app.get("/get_user_by_id", tags=["users"],)
async def get_user_by_id(
        KEY: Optional[str] = Header(None),
        UID: Optional[int] = Header(None)):
    "Получить пользователя по id telegram"
    print( KEY, UID)
    if UID is None or KEY != TOKEN:
        return None

    return await User.get_by_id( UID )

@app.get("/get_all_users", tags=["users"],)
async def get_all_users(
        KEY: Optional[str] = Header(None),
        UID: Optional[int] = Header(None)):
    "Получить всех пользователей из базы"
    print( KEY, UID )
    if UID is None or KEY != TOKEN:
        return None
    if not await User.admin(UID):
        return None

    return await User.all


@app.get("/is_known", tags=["users"],)
async def is_known(
        KEY: Optional[str] = Header(None),
        UID: Optional[int] = Header(None)):
    "Проверить id telegram, наличие в базе"
    if UID is None or KEY != TOKEN:
        return None

    return await User.check(UID)

@app.get("/is_admin", tags=["users"],)
async def is_admin(
        KEY: Optional[str] = Header(None),
        UID: Optional[int] = Header(None)):
    "Проверить id telegram, является ли админом"
    if UID is None or KEY != TOKEN:
        return None

    return await User.admin(UID)

@app.get("/all_users", tags=["users"],)
async def all_users(
        KEY: Optional[str] = Header(None),
        UID: Optional[int] = Header(None)):
    "Получить список всех пользователей"
    if UID is None or KEY != TOKEN:
        return None

    if not await User.admin(UID):
        return {'mess': 'Недостаточно прав'}

    return await User.all()

@app.get("/user_commands", tags=["users"],)
async def user_commands(
        KEY: Optional[str] = Header(None),
        UID: Optional[int] = Header(None)):
    "Получить команды данного пользователя, список кнопочек в боте"
    if UID is None or KEY != TOKEN:
        return None

    if not await User.check(UID):
        return None

    return await User.access( UID )

@app.post("/add_user", tags=["users"],)
async def add_user(
        KEY: Optional[str] = Header(None),
        UID: Optional[int] = Header(None),
        USER: User = Body(None) ):
    """Добавить пользователя или 
    изменить его параметры, если id существует"""
    if UID is None \
        or USER is None \
        or KEY != TOKEN:
        return None
    
    if not await User.admin( UID ):
        return {'mess' : 'Недостаточно прав'}

    return await USER.add()
 
@app.delete("/delete_user", tags=["users"],)
async def delete_user(
        KEY: Optional[str] = Header(None),
        UID: Optional[int] = Header(None),
        DELETEID: Optional[int] = Header(None)):
    """Удалить пользователя по его id,
    использовать в крайнем случае, если срочно надо"""
    if UID is None \
        or DELETEID is None \
        or KEY != TOKEN:
        return None
    
    if not await User.admin( UID ):
        return {'mess' : 'Недостаточно прав'}

    return await USER.delete( DELETEID )

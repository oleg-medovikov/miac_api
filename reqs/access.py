from .app import app
from typing import Optional
from fastapi import Header, Body

from clas import User, Access

@app.post("/add_access", tags=["access"],)
async def add_access(
        KEY : Optional[str] = Header(None),
        U_ID: Optional[int] = Header(None),
        ACCESS: Access = Body(None)):
    """Разрешаем пользователю выполнять
    конкретную команду"""
    if U_ID is None \
        or ACCESS is None \
        or KEY != TOKEN:
        return None

    if not await User.admin( U_ID ):
        return None
    try:
        await ACCESS.add()
    except:
        return {'mess' : 'no'}
    else:
        return {'mess' : 'ok'}


@app.delete("/delete_all_access", tags=["access"],)
async def delete_all_access(
        KEY : Optional[str] = Header(None),
        U_ID: Optional[int] = Header(None)):
    """Удаляем полностью все разрешения, 
    чтобы заново их восстановить из файлика.
    синхронизируем список разрешений с файлом
    """
    if U_ID is None or key != TOKEN:
        return None

    if not await User.admin( U_ID ):
        return None

    return await Access.delete_all()



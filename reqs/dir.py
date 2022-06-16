from .app import app
from typing import Optional
from fastapi import Header, Body

from clas import User, Dir
from conf import TOKEN

@app.get("/get_dir", tags=["dirs"],)
async def get_dir(
        KEY : Optional[str] = Header(None),
        U_ID: Optional[int] = Header(None),
        NAME: Optional[str] = Header(None)):
    "Получить директорию по её названию" 
    if U_ID is None \
        or NAME is None \
        or KEY != TOKEN:
        return None

    if not await User.check( U_ID ):
        return None

    return await Dir.get( NAME )


@app.post("/add_dir", tags=["dirs"],)
async def get_dir(
        KEY : Optional[str] = Header(None),
        U_ID: Optional[int] = Header(None),
        DIR: Dir = Body(None)):
    """Добавить новый объект Dir 
    или изменить старый с таким же названием.
    вместо удаления мы их отключаем 
    через ключ working"""
    if U_ID is None \
        or DIR is None \
        or KEY != TOKEN:
        return None

    if not await User.admin( U_ID ):
        return None

    return await DIR.add()




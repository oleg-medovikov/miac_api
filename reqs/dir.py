from .app import app
from typing import Optional
from fastapi import Header, Body

from clas import User, Dir
from conf import TOKEN

@app.get("/get_dir", tags=["dirs"],)
async def get_dir(
        KEY : Optional[str] = Header(None),
        NAME: Optional[str] = Body(None)):
    "Получить директорию по её названию" 
    if NAME is None \
        or KEY != TOKEN:
        return None

    return await Dir.get( NAME )


@app.post("/add_dir", tags=["dirs"],)
async def add_dir(
        KEY: Optional[str] = Header(None),
        UID: Optional[int] = Header(None),
        DIR: Dir = Body(None)):
    """Добавить новый объект Dir 
    или изменить старый с таким же названием.
    вместо удаления мы их отключаем 
    через ключ working"""
    if UID is None \
        or DIR is None \
        or KEY != TOKEN:
        return None

    if not await User.admin( UID ):
        return None

    return await DIR.add()


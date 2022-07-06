from .app import app
from typing import Optional
from fastapi import Header, Body

from clas import Choice
from conf import TOKEN


@app.post("/add_user_choice", tags=["choice"],)
async def add_user_choice(
        KEY: Optional[str] = Header(None),
        UID: Optional[int] = Header(None),
        CID: Optional[int] = Header(None),
        ):
    """Добавить выбранную пользователем команду,
    это нужно для команд с календарём"""

    if UID is None \
        or CID is None \
        or KEY != TOKEN:
        return None
    
    CHOICE = Choice(u_id = UID, c_id = CID )

    return await CHOICE.add()


@app.delete("/delete_user_choice", tags=["choice"],)
async def delete_user_choice(
        KEY: Optional[str] = Header(None),
        UID: Optional[int] = Header(None)
        ):
    """Удалить выбор пользователя"""

    if UID is None \
        or KEY != TOKEN:
        return None
    
    return await Choice.delete( UID )



@app.get("/get_user_choice", tags=["choice"],)
async def get_user_choice(
        KEY: Optional[str] = Header(None),
        UID: Optional[int] = Header(None)
        ):
    """Получить номер команды, которую 
    выбирал пользователь"""

    if UID is None \
        or KEY != TOKEN:
        return None
    
    return await Choice.get_choice( UID )






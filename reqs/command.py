from .app import app
from typing import Optional
from fastapi import Header, Body

from clas import User, Command
from conf import TOKEN

@app.get("/get_command", tags=["commands"],)
async def get_command(
        KEY: Optional[str] = Header(None),
        UID: Optional[int] = Header(None),
        CID: Optional[int] = Header(None)):
    "Получить объект command по его id"
    if UID is None \
        or CID is None \
        or KEY != TOKEN:
        return None

    if not await User.check(UID):
        return None

    return await Command.get_by_id( CID )


@app.post("/add_command", tags=["commands"],)
async def add_command(
        KEY: Optional[str] = Header(None),
        UID: Optional[int] = Header(None),
        COMMAND: Command = Body(None)):
    """Добавить новую команду 
    или изменить старую, если такой id существует"""

    if UID is None \
        or COMMAND is None \
        or KEY != TOKEN:
        return None

    if not await User.admin( UID ):
        return {'mess' : 'Недостаточно прав'}

    return await COMMAND.add()

@app.get("/all_commands", tags=["commands"],)
async def all_commands(
        KEY: Optional[str] = Header(None),
        UID: Optional[int] = Header(None)):
    "Получить все команды в виде объектов command"
    if UID is None or KEY != TOKEN:
        return None

    if not await User.admin( UID ):
        return {'mess' : 'Недостаточно прав'}

    return await Command.read()

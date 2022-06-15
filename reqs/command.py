from .app import app
from typing import Optional
from fastapi import Header, Body

from clas import User, Command


@app.get("/get_command", tags=["commands"],)
async def get_command(
        U_ID: Optional[int] = Header(None),
        C_ID: Optional[int] = Header(None)):

    if U_ID is None or C_ID is None:
        return None

    if not await User.check(U_ID):
        return None

    return await Command.get_by_id( C_ID )


@app.post("/add_command", tags=["commands"],)
async def add_command(
        U_ID: Optional[int] = Header(None),
        COMMAND: Command = Body(None)):

    if U_ID is None or COMMAND is None:
        return None

    if not await User.admin( U_ID ):
        return {'mess' : 'Недостаточно прав'}

    return await COMMAND.add()

@app.get("/all_commands", tags=["commands"],)
async def all_commands(U_ID: Optional[int] = Header(None)):
    if U_ID is None:
        return None

    if not await User.admin( U_ID ):
        return {'mess' : 'Недостаточно прав'}

    return await Command.read()

from .app import app
from typing import Optional
from fastapi import Header, Body
from fastapi import Request

from clas import User


@app.get("/is_known", tags=["users"],)
async def is_known(U_ID: Optional[int] = Header(None)):
    if U_ID is None:
        return None

    return await User.check(U_ID)

@app.get("/is_admin", tags=["users"],)
async def is_admin(U_ID: Optional[int] = Header(None)):
    if U_ID is None:
        return None

    return await User.admin(U_ID)

@app.get("/all_users", tags=["users"],)
async def all_users(U_ID: Optional[int] = Header(None)):
    if U_ID is None:
        return None

    if not await User.admin(U_ID):
        return {'mess': 'Недостаточно прав'}

    return await User.all()

@app.get("/user_commands", tags=["users"],)
async def all_users(U_ID: Optional[int] = Header(None)):
    if U_ID is None:
        return None

    if not await User.check(U_ID):
        return None

    return await User.access( U_ID )

@app.post("/add_user", tags=["users"],)
async def add_user(
        U_ID: Optional[int] = Header(None),
        USER: User = Body(None) ):

    if U_ID is None or USER is None:
        return None
    
    if not await User.admin( U_ID ):
        return {'mess' : 'Недостаточно прав'}

    return await USER.add()
    



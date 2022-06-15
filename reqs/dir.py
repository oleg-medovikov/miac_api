from .app import app
from typing import Optional
from fastapi import Header, Body

from clas import User, Dir

@app.get("/get_dir", tags=["dirs"],)
async def get_dir(
        U_ID: Optional[int] = Header(None),
        NAME: Optional[str] = Header(None)):

    if U_ID is None or NAME is None:
        return None

    if not await User.check( U_ID ):
        return None

    return await Dir.get( NAME )


@app.post("/add_dir", tags=["dirs"],)
async def get_dir(
        U_ID: Optional[int] = Header(None),
        DIR: Dir = Body(None)):

    if U_ID is None or DIR is None:
        return None

    if not await User.admin( U_ID ):
        return None

    return await DIR.add()




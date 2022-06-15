from .app import app 

from base import POSTGRESS_DB

@app.on_event("startup")
async def startup():
    await POSTGRESS_DB.connect()

@app.on_event("shutdown")
async def shutdown():
    await POSTGRESS_DB.disconnect()

from pydantic import BaseModel, Field

from sqlalchemy import and_
from base import POSTGRESS_DB, t_access

class Access(BaseModel):
    u_id : int
    c_id : int
    comment : str

    async def add(self):
        "Добавление доступа к комманде"
        query = t_access.insert().values(self.__dict__)
        await POSTGRESS_DB.execute(query)

    async def delete_all():
        await POSTGRESS_DB.execute("TRUNCATE TABLE access;")

    async def cheak(U_ID, C_ID):
        "Проверка пользователя на наличие прав на исполнение задачи"
        query = t_access.select().values(and_(
            t_access.c.u_id == U_ID,
            t_access.c.c_id == C_ID
                ))
        res = await POSTGRESS_DB.fetch_one(query)

        if res is None:
            return False
        else return True

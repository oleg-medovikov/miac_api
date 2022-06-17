from pydantic import BaseModel, Field

from sqlalchemy import and_
from base import POSTGRESS_DB, t_access

class Access(BaseModel):
    u_id : int
    c_id : int
    comment : str
    
    async def get_all():
        "Получить все аксесы целиком"
        sql = """
        select a.u_id, u.fio, a.c_id, c.c_name, a.comment from access as a
        join users as u    on(u.u_id = a.u_id)
        join commands as c on(c.c_id = a.c_id)
        order by a.u_id, a.c_id
        """

        return await POSTGRESS_DB.fetch_all(sql)


    async def add(self):
        "Добавление доступа к комманде"
        query = t_access.insert().values(self.__dict__)
        await POSTGRESS_DB.execute(query)

    async def delete_all():
        await POSTGRESS_DB.execute("TRUNCATE TABLE access;")
        return {'mess': 'access deleted'}

    async def cheak(U_ID, C_ID):
        "Проверка пользователя на наличие прав на исполнение задачи"
        query = t_access.select().where(and_(
            t_access.c.u_id == U_ID,
            t_access.c.c_id == C_ID
                ))
        res = await POSTGRESS_DB.fetch_one(query)

        if res is None:
            return False
        else:
            return True

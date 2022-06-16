from pydantic import BaseModel

from base import POSTGRESS_DB, t_choice

class Choice(BaseModel):
    u_id : int
    c_id : int

    async def add(self):
        """Добавляем новый выбор пользователя"""
        query = t_choice.delete(t_choice.c.u_id == self.u_id)
        await POSTGRESS_DB.execute(query)

        query = t_choice.insert().values(self.__dict__)
        await POSTGRESS_DB.execute(query)

    async def delete( U_ID ):
        """Удалить выбор пользователя"""
        query = t_choice.delete(t_choice.c.u_id == U_ID )
        await POSTGRESS_DB.execute(query)

    async def get(U_ID):
        """Получить выбор пользователя"""
        query = t_choice.select(t_choice.c.u_id == U_ID)
        res = await POSTGRESS_DB.fetch_one(query)
        
        if not res  is None:
            return res['c_id']


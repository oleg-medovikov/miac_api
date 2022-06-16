from datetime import date
from uuid import uuid4, UUID
from pydantic import BaseModel
from typing import Optional
from base import POSTGRESS_DB, t_users, t_people, t_access

class User(BaseModel):
    u_id        : int
    first_name  : Optional[str]
    last_name   : Optional[str]
    username    : Optional[str]
    groups      : str
    fio         : str
    description : str

    async def add(self):
        "Добавление пользователя в таблицу пользователей"
        
        query = t_users.select(t_users.c.u_id == self.u_id)
        res = await POSTGRESS_DB.fetch_one(query)

        if not res is None:
            query = t_users.delete(t_users.c.u_id == self.u_id)
            await POSTGRESS_DB.execute(query)

        query = t_users.insert().values(self.__dict__)
        await POSTGRESS_DB.execute(query)

        return {'mess' : 'ok'}

    async def delete(U_ID):
        "Удаление пользователя"
        query = t_users.delete(t_users.c.u_id == U_ID)
        await POSTGRESS_DB.execute(query)


    async def add_people(self):
        "Добавим людей которые писали боту"
        query = t_people.select(t_people.c.u_id == self.u_id)
        
        res = await POSTGRESS_DB.fetch_one(query)

        if res is None:
            MAN = self.__dict__
            MAN.pop('groups')
            MAN.pop('fio')
            MAN.pop('description')
            query = t_people.insert().values(MAN)
            await POSTGRESS_DB.execute(query)
    
    async def get_by_id(U_ID):
        "Взять пользователя по id"
        query = t_users.select(t_users.c.u_id == U_ID )

        res = await POSTGRESS_DB.fetch_one(query)

        if not res is None:
            return User(**res)
        else:
            return {'mess' : 'Нет такого пользователя'}


    async def check(U_ID) -> bool:
        "Проверка пользователя на наличие в базе"
        query = t_users.select(t_users.c.u_id == U_ID )

        res = await POSTGRESS_DB.fetch_one(query)

        if not res is None:
            return True
        else:
            return False

    async def admin(U_ID) -> bool:
        "Проверка на администратора"
        query = t_users.select(t_users.c.u_id == U_ID)

        res = await POSTGRESS_DB.fetch_one(query)

        if res is None:
            return False

        if res['groups'] == 'admin':
            return True
        else:
            return False

    async def access(U_ID):
        "Возвращает список доступных комманд"
        sql = f"""
        select 
            a.c_id,
            c.c_name,
            c.asc_day 
        from access as a
            join commands as c on (a.c_id = c.c_id) 
                where a.u_id = { U_ID }
        """
        return await POSTGRESS_DB.fetch_all(sql)

    async def all():
        sql = """select * from users"""

        return await POSTGRESS_DB.fetch_all(sql)


from pydantic import BaseModel, Field

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

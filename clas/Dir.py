from pydantic import BaseModel

from base import POSTGRESS_DB, t_dirs

class Dir(BaseModel):
    d_id        : int
    d_name      : str
    directory   : str
    description : str
    working     : bool


    async def get(NAME) -> str:
        "Получить директорию по имени"
        query = t_dirs.select(t_dirs.c.d_name == NAME)
        res = await POSTGRESS_DB.fetch_one(query)

        if not res is None and res['working']:
            return res['directory']
        else:
            return ''

    async def add(self):
        "Добавляем новую директорию"
        query = t_dirs.select(t_dirs.c.d_id == self.d_id)
        res = await POSTGRESS_DB.fetch_one(query)

        if not res is None:
            query = t_dirs.delete(t_dirs.c.d_id == self.d_id)
            await POSTGRESS_DB.execute(query)

        query = t_dirs.insert().values(self.__dict__)
        await POSTGRESS_DB.execute(query)

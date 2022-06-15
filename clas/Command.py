from datetime import date
from uuid import uuid4, UUID
from pydantic import BaseModel, Field



from base import POSTGRESS_DB, t_commands
from func import *

class Command(BaseModel):
    c_id : int
    c_category : str
    c_name : str
    c_procedure : str
    c_arg : str
    return_file : bool
    asc_day : bool

    async def add(self):
        "добавление новой команды"
        query = t_commands.select(t_commands.c.c_id == self.c_id)

        res = await POSTGRESS_DB.fetch_one(query)
        
        if not res is None:
            query = t_commands.delete(t_commands.c.c_id == self.c_id)
            await POSTGRESS_DB.execute(query)
        
        query = t_commands.insert().values(self.__dict__)

        await POSTGRESS_DB.execute(query)

    async def read():
        "Чтение всех комманд"
        query = t_commands.select().order_by(t_commands.c.c_id)
        
        return await POSTGRESS_DB.fetch_all(query)
    
    async def get(C_ID):
        "Получение команды по айдишнику"
        query = t_commands.select(t_commands.c.c_id == int(C_ID))
        res = await POSTGRESS_DB.fetch_one(query)
         
        return Command(**res)
        

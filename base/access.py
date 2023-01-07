from .postgress import metadata

from sqlalchemy import Table, Column, Integer, String, BigInteger

t_access = Table(
    "access",
    metadata,
    Column('u_id', BigInteger), # номер юзера
    Column('c_id', Integer), # номер команды
    Column('comment', String), # комментарий
    )
 

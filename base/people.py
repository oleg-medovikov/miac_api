from sqlalchemy import Table, Column, Integer, String
from .postgress import metadata


t_people = Table(
    "people",
    metadata,
    Column('u_id', Integer), # Идентификатор юзера в телеге
    Column('first_name', String), # Имя юзера в телеге
    Column('last_name', String), # Фамилия юзера в телеге
    Column('username', String), # username юзера в телеге
    )

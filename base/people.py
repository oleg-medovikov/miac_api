from sqlalchemy import Table, Column, Integer, String, BigInteger
from .postgress import metadata


t_people = Table(
    "people",
    metadata,
    Column('u_id', BigInteger), # Идентификатор юзера в телеге
    Column('first_name', String), # Имя юзера в телеге
    Column('last_name', String), # Фамилия юзера в телеге
    Column('username', String), # username юзера в телеге
    )

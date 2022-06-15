from sqlalchemy import Table, Column, Integer, String, Boolean
from .postgress import metadata

t_commands = Table(
    "commands",
    metadata,
    Column('c_id', Integer),        # Идентификатор команды
    Column('c_category', String),   # Категория команды 
    Column('c_name', String),       # Имя команды
    Column('c_procedure', String),  # Имя процедуры
    Column('c_arg', String),        # Аргументы команды
    Column('return_file', Boolean), # Возвращается ли файл?
    Column('asc_day', Boolean),     # Спрашивать ли день? 
    )

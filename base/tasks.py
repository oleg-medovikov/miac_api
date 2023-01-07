from .postgress import metadata

from sqlalchemy import Table, Column, Integer, String, DateTime, BigInteger
from sqlalchemy.dialects.postgresql import UUID
import uuid

t_tasks = Table(
    "tasks",
    metadata,
    Column('t_id', UUID(), primary_key=True, default=uuid.uuid4), #идентификатор задания
    Column('time_create', DateTime), # Время создания задачи
    Column('client', BigInteger),       # номер юзера создавшего задание
    Column('task_type', String),     # тип события
    Column('c_id', Integer),         # номер команды
    Column('c_func', String),        # функция команды
    Column('c_arg', String),         # аргумент функции команды
    Column('users_list', String),    # Список юзеров, которым нужно прислать ответ
    Column('time_start', DateTime),  # время начала выполнения задачи
    Column('time_stop', DateTime),   # время окончания выполнения задачи
    Column('comment', String),       # комментарий
    )
 

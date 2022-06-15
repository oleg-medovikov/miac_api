from .postgress import metadata

from sqlalchemy import Table, Column, Integer

t_choice = Table(
    "choice",
    metadata,
    Column('u_id', Integer),
    Column('c_id', Integer),
        )


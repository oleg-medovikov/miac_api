from .postgress import metadata

from sqlalchemy import Table, Column, Integer, BigInteger

t_choice = Table(
    "choice",
    metadata,
    Column('u_id', BigInteger),
    Column('c_id', Integer),
        )


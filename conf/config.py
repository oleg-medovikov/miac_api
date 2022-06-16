from starlette.config import Config

config = Config('.conf')

DATABASE_POSTGRESS = config('DATABASE_POSTGRESS', cast=str)

TOKEN = config('TOKEN', cast=str)


from starlette.config import Config

config = Config('../.config/bot/.conf')

DATABASE_POSTGRESS = config('DATABASE_POSTGRESS', cast=str)

TOKEN = config('TOKEN', cast=str)


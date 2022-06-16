from starlette.config import Config

config = Config('../.config/bot/.conf')

DATABASE_POSTGRESS = config('DATABASE_POSTGRESS', cast=str)

TOKEN = config('TOKEN', cast=str)

FAVICON_URL = config('FAVICON_URL', cast=str)


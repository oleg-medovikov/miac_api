from .app import app
from conf import FAVICON_URL

from fastapi.responses import RedirectResponse
@app.get('/')
async def root():
    return RedirectResponse( '/docs' )

@app.get('/favicon.ico')
async def favicon():
    return RedirectResponse( FAVICON_URL )

from fastapi import FastAPI

from src.router import register_api_router

app = FastAPI(
    title="Milky-Auth"
)

app.include_router(register_api_router(is_debug=True))

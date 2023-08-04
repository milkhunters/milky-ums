from fastapi import APIRouter

from src.controllers import auth


def register_api_router(is_debug: bool) -> APIRouter:
    root_api_router = APIRouter(prefix="/api/v1" if is_debug else "")

    root_api_router.include_router(auth.router, prefix="/auth", tags=["Auth"])

    return root_api_router

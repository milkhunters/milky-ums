import logging
from datetime import timedelta

from fastapi import FastAPI, APIRouter
from fastapi.exceptions import RequestValidationError

from ums.config import load_config
from ums.controllers import auth, user, role, stats
from ums.exceptions import (
    APIError,
    handle_api_error,
    handle_404_error,
    handle_pydantic_error
)
from ums.lifespan import LifeSpan

from ums.security.jwt import JwtTokenProcessor
from ums.security.middleware.jwt import JWTMiddlewareHTTP
from ums.utils.openapi import custom_openapi


class ApplicationFactory:

    @staticmethod
    def create_app() -> FastAPI:
        config = load_config()
        logging.basicConfig(level=logging.DEBUG if config.DEBUG else logging.INFO)
        app = FastAPI(
            title=config.BASE.TITLE,
            debug=config.DEBUG,
            version=config.BASE.VERSION,
            description=config.BASE.DESCRIPTION,
            root_path=config.BASE.SERVICE_PATH_PREFIX if not config.DEBUG else "",
            docs_url="/api/docs" if config.DEBUG else "/docs",
            redoc_url="/api/redoc" if config.DEBUG else "/redoc",
            swagger_ui_parameters={"syntaxHighlight.theme": "obsidian"},
            contact={
                "name": config.BASE.CONTACT.NAME,
                "url": config.BASE.CONTACT.URL,
                "email": config.BASE.CONTACT.EMAIL,
            },
        )
        app.openapi = lambda: custom_openapi(app)
        getattr(app, "state").config = config
        getattr(app, "state").jwt = JwtTokenProcessor(
            private_key=config.JWT.PRIVATE_KEY,
            public_key=config.JWT.PUBLIC_KEY,
            access_expires=timedelta(seconds=config.JWT.ACCESS_EXP_SEC),
            refresh_expires=timedelta(seconds=config.JWT.REFRESH_EXP_SEC),
            algorithm="ES256",
        )
        if not config.DEBUG:
            logging.getLogger("apscheduler").setLevel(logging.INFO)
            logging.getLogger("aiohttp").setLevel(logging.WARNING)
        lifespan = LifeSpan(app, config)
        app.add_event_handler("startup", lifespan.startup_handler)
        app.add_event_handler("shutdown", lifespan.shutdown_handler)

        logging.debug("Регистрация маршрутов API")
        api_router = APIRouter(prefix="/api/v1" if config.DEBUG else "")
        api_router.include_router(auth.router, prefix="/auth", tags=["Auth"])
        api_router.include_router(user.router, prefix="/user", tags=["User"])
        api_router.include_router(role.router, prefix="/role", tags=["Role"])
        api_router.include_router(stats.router, prefix="", tags=["Stats"])
        app.include_router(api_router)

        logging.debug("Регистрация обработчиков исключений")
        app.add_exception_handler(APIError, handle_api_error)
        app.add_exception_handler(404, handle_404_error)
        app.add_exception_handler(RequestValidationError, handle_pydantic_error)

        logging.debug("Регистрация middleware.")
        app.add_middleware(JWTMiddlewareHTTP)

        logging.info("Приложение успешно создано")
        return app


application = ApplicationFactory.create_app()

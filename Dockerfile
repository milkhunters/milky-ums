FROM python:3.12.2-alpine3.19

RUN mkdir /app

# Setup FastAPI application
COPY . /app/code
WORKDIR /app/code

RUN python -m venv venv
RUN . venv/bin/activate
RUN pip install -e .

ENV PYTHONPATH=/app/code/src

CMD ["uvicorn", "src.ums.main:application", "--proxy-headers", "--host", "0.0.0.0", "--port", "8000", "--no-server-header"]

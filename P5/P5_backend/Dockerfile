FROM python:3.11-slim

WORKDIR /usr/app/src

COPY executor.py ./
COPY executor_copy.py ./

RUN pip install numpy

EXPOSE 4000

CMD  ["python", "./executor.py" ]
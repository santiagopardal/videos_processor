FROM python:3.11.4-slim

RUN apt-get update
RUN apt-get -y install ffmpeg libsm6 libxext6

RUN pip install --upgrade pip
RUN pip install --upgrade setuptools
RUN pip install -r requirements-container.txt --no-cache-dir

COPY . .

CMD ["python", "-m", "src.processor"]

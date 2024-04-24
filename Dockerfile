FROM python:3.11.4-slim

WORKDIR /videos_processor

COPY . .

CMD ["python", "-m", "src.processor"]

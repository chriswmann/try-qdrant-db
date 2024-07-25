iter:
  docker compose build && docker compose run qdrant-client

full-iter:
  docker compose down && docker compose build && docker compose up -d --remove-orphans && docker compose run qdrant-client

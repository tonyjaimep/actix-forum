start:
	docker compose up -d

migrate:
	docker compose exec actix-forum diesel migration run

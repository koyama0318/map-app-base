env:
	@export $$(cat .env | xargs)

up: env
	docker-compose up --build

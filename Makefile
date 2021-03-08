build:
	cargo build

build-release:
	cargo build release


build-docker:
	docker build -t ewestar/campip:latest .

up:
	docker-compose up -d

down:
	docker-compose down

reboot: down up

logs:
	docker-compose logs -f

peek:
	docker-compose logs

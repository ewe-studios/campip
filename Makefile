build:
	cargo build

build-release:
	cargo build release

tag-docker:
	docker tag ewestudios-campip:latest ewestudios/campip:latest

build-docker:
	docker build -t ewestudios-campip:latest .

docker: build-docker tag-docker

up:
	docker-compose up -d

down:
	docker-compose down

reboot: down up

logs:
	docker-compose logs -f

peek:
	docker-compose logs

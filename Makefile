args?=

build:
	cargo build

release:
	cargo build --release

binary_exec:
	chmod +x target/debug/campip
	chmod +x target/release/campip

tag-docker:
	docker tag ewestudios-campip:latest ewestudios/campip:latest

build-docker: binary_exec
	docker build ${args} -t ewestudios-campip:latest .

push:
	docker push  ewestudios/campip:latest

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

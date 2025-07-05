.PHONY: build run clean test docker-build docker-run docker-dev docker-clean

build:
    cargo build --release

run:
    cargo run

test:
    cargo test

clean:
    cargo clean

docker-build:
    docker build -t linklike-toolbox:latest .

docker-run:
    docker-compose up linklike-toolbox

docker-dev:
    docker-compose --profile dev up linklike-dev

docker-clean:
    docker-compose down
    docker system prune -f

build-linux:
    cargo build --release --target x86_64-unknown-linux-gnu

build-windows:
    cargo build --release --target x86_64-pc-windows-gnu

docker-build-linux:
    docker build --target builder -t linklike-builder .
    docker run --rm -v $(PWD)/target:/app/target linklike-builder

release-all: build-linux build-windows docker-build
    echo "All platform builds completed"
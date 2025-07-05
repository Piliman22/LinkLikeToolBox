.PHONY: build run clean test docker-build docker-run docker-dev docker-clean deploy-build deploy-package

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

docker-build-linux:
    docker build --target builder -t linklike-builder .
ifeq ($(OS),Windows_NT)
    docker run --rm -v "%cd%/target:/app/target" linklike-builder
else
    docker run --rm -v $(PWD)/target:/app/target linklike-builder
endif

deploy-build:
    docker build -t linklike-toolbox:latest .
    docker create --name temp-container linklike-toolbox:latest
    docker cp temp-container:/usr/local/bin/linklike ./linklike-linux
    docker rm temp-container

deploy-package: deploy-build
ifeq ($(OS),Windows_NT)
    if not exist deploy mkdir deploy
    copy linklike-linux deploy\
    copy docker-compose.yml deploy\
    copy Dockerfile deploy\
    if exist config (xcopy /E /I config deploy\config) else (mkdir deploy\config)
    echo # LinkLike ToolBox > deploy\README.md
    echo ## Usage >> deploy\README.md
    echo ### Standalone: ./linklike-linux >> deploy\README.md
    echo ### Docker: docker-compose up linklike-toolbox >> deploy\README.md
    tar -czf linklike-toolbox-linux.tar.gz -C deploy .
else
    mkdir -p deploy
    cp linklike-linux deploy/
    cp docker-compose.yml deploy/
    cp Dockerfile deploy/
    cp -r config deploy/ 2>/dev/null || mkdir deploy/config
    echo "# LinkLike ToolBox" > deploy/README.md
    echo "## Usage" >> deploy/README.md
    echo "### Standalone: ./linklike-linux" >> deploy/README.md
    echo "### Docker: docker-compose up linklike-toolbox" >> deploy/README.md
    tar -czf linklike-toolbox-linux.tar.gz -C deploy .
endif

release-all: build-linux build-windows docker-build
    echo "All platform builds completed"
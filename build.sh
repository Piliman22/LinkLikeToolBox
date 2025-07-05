set -e

echo "Building LinkLike ToolBox for Linux..."

docker build -t linklike-toolbox:latest .

if [ "$1" = "--extract" ]; then
    echo "Extracting binary from Docker image..."
    docker create --name temp-container linklike-toolbox:latest
    docker cp temp-container:/usr/local/bin/linklike ./linklike-linux
    docker rm temp-container
    echo "Binary extracted to ./linklike-linux"
fi

echo "Build completed successfully!"
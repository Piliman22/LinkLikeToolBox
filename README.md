# Link Like ToolBox


```
# Dockerイメージをビルド
docker build -t linklike-toolbox .

# コンテナを実行
docker run --rm -v $(pwd)/cache:/app/cache -v $(pwd)/masterdata:/app/masterdata linklike-toolbox

# Docker Composeを使用
docker-compose up linklike-toolbox

# 開発モード
docker-compose --profile dev up linklike-dev
```
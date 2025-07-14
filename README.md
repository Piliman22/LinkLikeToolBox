# LinkLike ToolBox

LinkLive（Link！Like！ラブライブ！蓮ノ空スクールアイドルクラブ）の複合化、暗号化、データ取得が可能のRust製ツールです。

## 機能

### アセットバンドル操作
- **復号化**: LinkLikeフォーマットのアセットバンドルファイルを復号化
- **暗号化**: 通常のファイルをLinkLikeフォーマットのアセットバンドルに暗号化
- **バッチ処理**: フォルダ内の複数ファイルを一括処理

### チャートデータ操作
- **展開**: 圧縮されたチャートファイル(.bytes)をJSONに展開
- **圧縮**: JSONファイルをチャートフォーマット(.bytes)に圧縮
- **圧縮レベル調整**: 0-9の圧縮レベルを指定可能

### ダウンロード機能
- **自動更新**: ゲームデータの自動取得・更新
- **マニフェスト取得**: カタログマニフェストファイルのダウンロード
- **アセット一括取得**: カタログに基づくアセットファイルの一括ダウンロード
- **マスターデータ取得**: TSVフォーマットのマスターデータベースファイル取得

## インストール

### 前提条件
- Rust 1.70+
- Cargo

### ビルド方法

```bash
# リポジトリをクローン
git clone <repository-url>
cd LinkLikeToolBox

# デバッグビルド
cargo build

# リリースビルド
cargo build --release
```

### Docker使用
```bash
# Linuxバイナリをビルド
make build-linux

# デプロイメントパッケージを作成
make deploy-package
```

## 使用方法

### 基本コマンド

```bash
# ヘルプを表示
./linklike-toolbox help

# アセットバンドルを復号化
./linklike-toolbox decrypt ab <ファイルパス>

# チャートファイルを展開
./linklike-toolbox decrypt chart <ファイルパス>

# JSONをチャートファイルに圧縮（圧縮レベル6）
./linklike-toolbox crypt chart <ファイルパス> 6
```

### ダウンロードコマンド

```bash
# 自動更新（データベースのみ）
./linklike-toolbox download auto --db-only

# 強制更新（全ファイル）
./linklike-toolbox download auto --force

# 生ファイルを保持して更新
./linklike-toolbox download auto --keep-raw

# マニフェストファイルのダウンロード
./linklike-toolbox download manifest <real_name> <保存ディレクトリ>

# アセットファイルの一括ダウンロード
./linklike-toolbox download assets <カタログパス> <ダウンロードディレクトリ>
```

### 自動更新オプション

- `--force`: 強制的に全ファイルを更新
- `--db-only`: データベース（TSV）ファイルのみを取得

## プロジェクト構造

```
LinkLikeToolBox/
├── src/                    # メインアプリケーション
│   ├── main.rs            # エントリーポイント
│   └── lib.rs             # ライブラリエクスポート
├── crates/
│   ├── LinkLike_Core/     # コア機能
│   │   ├── src/
│   │   │   ├── crypt/     # 暗号化・復号化
│   │   │   ├── fetch/     # ダウンロード機能
│   │   │   ├── master/    # マスターデータ処理
│   │   │   └── manifest.rs # マニフェスト処理
│   │   └── Cargo.toml
│   └── LinkLike_CLI/      # CLI機能
│       ├── src/
│       │   ├── command.rs # コマンド処理
│       │   ├── banner.rs  # バナー表示
│       │   └── progress.rs # プログレス表示
│       └── Cargo.toml
├── masterdata/            # マスターデータファイル
├── cache/                 # キャッシュディレクトリ
├── example/               # サンプルファイル
├── Dockerfile            # Docker設定
├── docker-compose.yml    # Docker Compose設定
└── Makefile              # ビルドスクリプト
```

## 設定とディレクトリ

### デフォルトディレクトリ
- `cache/`: キャッシュとマニフェストファイル
- `cache/assets/`: ダウンロードしたアセットファイル
- `cache/plain/`: 復号化されたファイル
- `masterdata/`: マスターデータ（TSV）ファイル

### 設定ファイル
- `cache/currentVersion.txt`: 現在のリソースバージョン
- `cache/catalog.json`: アセットカタログ
- `cache/catalog_diff.json`: カタログ差分
- `cache/updated`: 更新フラグファイル

## 技術仕様

### 暗号化仕様
- **アセットバンドル**: カスタムヘッダー（0xAB 0x00）付きの独自フォーマット
- **マニフェスト**: AES-128-CBC + LZ4圧縮
- **チャートデータ**: Raw Deflate圧縮

### サポートフォーマット
- `.bytes`: 圧縮チャートファイル
- `.tsv`: マスターデータファイル
- `.json`: 展開されたチャートデータ
- アセットバンドル: 独自暗号化フォーマット

### 依存関係
- `reqwest`: HTTP通信
- `serde`: シリアライゼーション
- `tokio`: 非同期処理
- `aes`: AES暗号化
- `lz4_flex`: LZ4圧縮
- `flate2`: Deflate圧縮

## 開発

### 開発環境セットアップ
```bash
# 依存関係をインストール
cargo fetch

# テストを実行
cargo test

# フォーマット
cargo fmt

# Lintチェック
cargo clippy
```

### Docker開発環境
```bash
# 開発用コンテナを起動
docker-compose -f docker-compose.yml up dev

# Linuxビルド用コンテナ
docker build --target builder -t linklike-builder .
```

## ライセンス

本プロジェクトは教育・研究目的で開発されています。
ゲームデータの取り扱いについては、各自の責任で適切に行ってください。

## 貢献

バグ報告や機能提案は Issue にて受け付けています。
プルリクエストも歓迎いたします。

## 作者

Created by pim4n

---

**注意**: このツールは非公式のファンメイドツールです。
公式のゲーム運営とは一切関係ありません。
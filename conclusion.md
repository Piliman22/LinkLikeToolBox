# 解析

この記事は学習参考用であり、違法な目的で使用しないでください。

## 最初に
最近リンクラ(Link！Like！ラブライブ！蓮ノ空スクールアイドルクラブ)に音ゲー機能ができましたよね。

そこで初めてみてどういう構造をしているのか気になったためこれを作り始めました。

創作譜面に触れている自分としては、自分で譜面が作れて撮影できたらなと考えていました。

比較的こういった解析をしている人がいなかったのでインターネット上ではなかなか解析情報が転がっていなかったので自分で解析することになりました。

プロセカ(プロジェクト世界カラフルステージfeet.初音ミク)でも多少の解析はしていたものの、自分でここまでやるのは初めてだったので必要になったときに記録として残しておきます。

## apk

まずはインストーラーファイルの解析からです。

とは言ってもここでできるのは、`libil2cpp.so`と`global-metadata.dat`を抽出してそこからil2cppdumperを書けることぐらいです。

[ここから](https://apkpure.com/jp/link%EF%BC%81like%EF%BC%81%E3%83%A9%E3%83%96%E3%83%A9%E3%82%A4%E3%83%96%EF%BC%81/com.oddno.lovelive)まずはapkをもらいました。

xapkでした。最近のものはxapkが多いですよね。

そんなことは気にせずそれぞれ解凍していって取り出してil2cppdumperにかけました。

一発で通りましたね。いままでやってきたアプリではメモリから`global-metadata.dat`を抽出しないといけないものが多かったので、驚きでしたね。

自動化が楽になります。

## frida

ここからは動的解析の話です。

### Frida-gadgetの導入

あいにくroot化されているいい感じの端末が自宅にないためapkにfrida-gadgetを入れる方法しかありません。

objectionというものを使うと簡単にapkにfrida-gadgetを入れることができます。

objectionでもいいのですが、バージョンが変わったときとか自動でできないので、自動化の[リポジトリ](https://github.com/Piliman22/inject-lovelive)を作りました

ここからinjectしたapkを使ってandroidに入れてあげましょう。

### HMACSHA1の鍵抽出
ここからはフックする作業です。

フックするには関数のアドレス、もしくはクラス名がわからないとフックできません。
ですが、今回はil2cppdumperでapkからdump.csの生成に成功したためこちらを使います

まずは暗号化です。dump.csを見ていたところこちらを見かけました

```cs
public enum AssetUtils.eResourceType // TypeDefIndex: 11686
{
    // Fields
    public int value__; // 0x0
    public const AssetUtils.eResourceType PlainAssetBundle = 0;
    public const AssetUtils.eResourceType OffsetAssetBundle = 1;
    public const AssetUtils.eResourceType RawAssetMinValue = 128;
    public const AssetUtils.eResourceType RawFile = 128;
    public const AssetUtils.eResourceType EncodedBin0File = 192;
}
```
一個ずつ見ていきましょう

- PlainAssetBundle おそらく通常の.assetbundleもしくは.unity3dでしょう

- OffsetAssetBundle 先頭にオフセットがあるアセットバンドルでしょう。暗号化やヘッダー回避用ととらえておきましょう

- RawAssetMinValue / RawFile 名前の通り生のファイルでしょう。おそらくそのまま読み込むタイプのものだと考えられます。`txt`や`png`や`json`など。 

- EncodedBin0File　エンコードされたバイナリファイルでしょう。独自の暗号化や独自形式で保護されたファイルって考えときます。

ゲームファイル`/android/data/com.oddno.lovelive/files`に`D`と`M`というフォルダがありましたよね。
今回の場合は`D`にデータファイル、`M`にマニフェスト関連があると考えられます。

試しに`D`フォルダを見てみたところ、わけのわからないファイル名になっているのが多数です。
このままじゃ何もわかりませんので、マニフェストを複合化、もしくはAPIの中間攻撃を利用する以外厳しいです。

そこで、[ここ](https://blog.vibbit.me/2024/02/llll-reverse/)を読んでみたところ、どうやらこのManifestは"app"のHMACSHA1を計算し、得られたバイト列をbase64エンコードし、UnityEngine.PlayerPrefsのキーとして使用してるようです。
さらに、`RNGCryptoServiceProvider`を使用し、256バイトのランダムデータを生成し、これもbase64エンコードし、これも`UnityEngine.PlayerPrefs`として保存するようです

ということで、まずはHMACSHA1を見つけます。

dump.csにて

```cs
// Namespace: System.Security.Cryptography
public class HMACSHA1 : HMAC // TypeDefIndex: 10562
{
	// Methods

	// RVA: 0x357A160 Offset: 0x357A160 VA: 0x357A160
	public void .ctor() { }

	// RVA: 0x357A8AC Offset: 0x357A8AC VA: 0x357A8AC
	public void .ctor(byte[] key) { }

	// RVA: 0x357A8B4 Offset: 0x357A8B4 VA: 0x357A8B4
	public void .ctor(byte[] key, bool useManagedSha1) { }
}
```

こちらを見つけました。
つまりここからHMACSHA1のキーを監視することができますね。

このようにフックしてみました。

```ts
const il2cpp = Process.getModuleByName("libil2cpp.so").base;
console.log("il2cpp base address: " + il2cpp);

const ctorOffset = 0x357A8B4;
const ctorAddr = il2cpp.add(ctorOffset);

function readString(p: NativePointer): string | null{
    if (p.isNull()) return null;
    const length: number = p.add(0x10).readInt();
    return p.add(0x14).readUtf16String(length);
}

function readByteArray(p: NativePointer): ArrayBuffer | null {
    if (p.isNull()) {
        console.log("[DEBUG] Input pointer is null");
        return null;
    }
    
    console.log("[DEBUG] Memory at pointer: " + 
                p.readU32() + " " + 
                p.add(4).readU32() + " " +
                p.add(8).readU32() + " " +
                p.add(12).readU32());
    
    const dataSize = p.add(0x18).readUInt();
    console.log("[DEBUG] Data size at offset 0x18: " + dataSize);
    
    if (dataSize > 0 && dataSize <= 1000) {
        try {
            const keyData = p.add(0x20).readByteArray(dataSize);
            return keyData;
        } catch (e) {
            console.log("[ERROR] Failed to read key data: " + e);
            return null;
        }
    } else {
        console.log("[DEBUG] Invalid data size: " + dataSize);
        return null;
    }
}
function byteArrayToHex(buffer: ArrayBuffer | null): string {
    if (!buffer) return "null";
    return Array.from(new Uint8Array(buffer))
        .map(b => b.toString(16).padStart(2, '0'))
        .join(' ');
}

function hexDump(address: NativePointer, size: number = 64): string {
    let result = "";
    for (let i = 0; i < size; i += 16) {
        const bytes = Array.from(new Uint8Array(address.add(i).readByteArray(Math.min(16, size - i))!))
            .map(b => b.toString(16).padStart(2, '0'));
            
        const offset = "0x" + i.toString(16).padStart(4, '0');
        const hex = bytes.join(' ').padEnd(48, ' ');
        const ascii = bytes.map(b => {
            const code = parseInt(b, 16);
            return (code >= 32 && code <= 126) ? String.fromCharCode(code) : '.';
        }).join('');
        
        result += `${offset}: ${hex} | ${ascii}\n`;
    }
    return result;
}

Interceptor.attach(ctorAddr, {
    onEnter: function(args) {
        console.log("\n========== HMACSHA1 constructor called ==========");
        console.log("Instance: " + args[0]);
        console.log("Key pointer: " + args[1]);
        console.log("Memory dump of key pointer area:");
        if (!args[1].isNull()) {
            console.log(hexDump(args[1], 64));
        }
        const keyBuffer = readByteArray(args[1]);
        console.log("Key content: " + byteArrayToHex(keyBuffer));
        
        if (keyBuffer) {
            console.log("Key length: " + keyBuffer.byteLength + " bytes");
        }
    }
});
```

はい、実行してみましょう。
`frida -UF -l hook.ts -n Gadget`今回はGadgetをapkに入れたのでこのコマンドです。

実行したら、なんと、出てきましたね。

例：
```
1c c8 39 7e 77 99 b1 16 7f 88 a4 a1 e7 e9 62 87 
c0 5d 05 0e f8 96 13 d4 2e 94 4b c9 09 9c 44 42 
9b cd fd 13 ff 96 1d dc fb ad db 63 37 ff 90 6b 
95 30 d1 b5 8e f3 e0 2f 8f 1d 8e b0 08 6d 52 c1
```

はい、そしたら複合してみましょう

### PythonでのHMAC検証

```py
import hmac
import hashlib
import base64

key_hex = '''
1c c8 39 7e 77 99 b1 16 7f 88 a4 a1 e7 e9 62 87
c0 5d 05 0e f8 96 13 d4 2e 94 4b c9 09 9c 44 42
9b cd fd 13 ff 96 1d dc fb ad db 63 37 ff 90 6b
95 30 d1 b5 8e f3 e0 2f 8f 1d 8e b0 08 6d 52 c1
'''

key_bytes = bytes.fromhex(key_hex.replace('\n', ' ').strip())

# HMACSHA1("app")
message = b"app"
hmac_result = hmac.new(key_bytes, message, hashlib.sha1).digest()

# Base64エンコード
base64_key = base64.b64encode(hmac_result).decode()

print("PlayerPrefs Key:", base64_key)
```

このように出てきました
`PlayerPrefs Key: y+Bv+i7aGj9up6h6jXWU8u7b6F8=`

まとめ：
このようにして `HMACSHA1("app")` に使われる鍵を抽出し、PlayerPrefs のキーを完全に再現することができる。
あとは `GetString()` をフックするか、他の値と照合することで、ゲーム内部の暗号化セーブやユーザー識別子の保存方式を逆解析できる。

ですが、手順が大変です。

こう考えるとAPIの中間攻撃のほうが楽であり、他ユーザーでも簡単にマニフェスト関連の取得ができます。

ということでAPI周りに行きましょう

## API

APIについてです。

このゲームの基礎的なAPIは主にこの二つでした

- `https://assets.link-like-lovelive.app`
- `https://api.link-like-lovelive.app`

上のものはおそらくアセットのダウンロードに使うapiでしょう。
下のものはログインだったりのapi用でしょう。

さて、マニフェスト関連に行きましょう

### マニフェストについて

上記[HMACSHA1の鍵抽出](#hmacsha1の鍵抽出)でも述べた通り、ローカルでキャッシュされる場合は、異なるキーで暗号化されています。
ですが、apiから取得する際は、必ず同じ結果が返ってくるでしょう。
そうでない場合は、oddnumは256^256個のファイルを生成しないといけないことになりますね。
あまりにもこれは非現実的です。

ちなみに256^256は`1.34078079 × 10^616`となり、宇宙に存在すると推定されている原子の数はだいたい`10^80`程度なので非現実的なことがわかると思います。

話が少しそれましたがパケットキャプチャをしていきましょう。
今回はMitmProxyを使います。

### MitmProxyでのパケットキャプチャ

写真を撮りそびれたので文章でとなりますが、初回ログイン時にはほとんど空の内容を`POST`しています。

そしてサーバーのレスポンスヘッダーに`x-res-version`を含めて返してきます。

ユーザーはこれを基準にし、対応するバージョンのマニフェストを取得し、そこからアセットをダウンロードしていると考えられます。

### x-res-versionについて

実際のAPIを見ると、こういう風になってます。

**リクエスト（POST）:**
```json
{
    "device_specific_id": "",
    "player_id": "",
    "version": 1
}
```

**レスポンスヘッダー:**
```
x-res-version: R2402010
Content-Type: application/json
...
```

このx-res-versionがマニフェストのバージョン識別子として使用されます。

コード上では、以下のような流れでx-res-versionを取得しています：

1. **クライアントバージョンの取得** - Google Play Storeから最新のアプリバージョンを自動取得
2. **ログインAPI呼び出し** - 取得したクライアントバージョンを使ってログインリクエストを送信
3. **x-res-version抽出** - レスポンスヘッダーからx-res-versionを取得
4. **マニフェスト取得** - 取得したx-res-versionを使ってマニフェストファイルをダウンロード

例えば、x-res-versionが`R2402010`の場合、マニフェストファイルは以下のURLから取得されます：
```
https://assets.link-like-lovelive.app/catalog/R2402010
```

そして、このマニフェストファイルには、アセットバンドルの一覧とそのハッシュ値、ダウンロードパスなどが含まれており、ゲームが必要なリソースを特定し、適切な順序でダウンロードするための設計図として機能します。

興味深いことに、このx-res-versionはクライアントバージョンとは独立しており、同じクライアントバージョンでも複数のリソースバージョンが存在する可能性があります。これにより、アプリのアップデートなしにゲーム内容の更新が可能になっています。

### アセットダウンロードの流れ

x-res-versionを取得した後のアセットダウンロードの流れは以下のようになります：

1. マニフェストファイルの解析
2. 必要なアセットバンドルの特定
3. 並列ダウンロードによるアセット取得
4. 復号化とファイル変換処理

この仕組みにより、ゲームは効率的にリソース管理を行い、プレイヤーに最新のコンテンツを提供することができています。

### 実装

実際のコード[`playversion.rs`](crates/LinkLike_Core/src/fetch/playversion.rs)でGoogle Play Storeから最新のクライアントバージョンを自動取得しています：

```rust
pub async fn get_play_version(game_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("{}{}", VERSION_URL, game_id);
    // Google Play Storeのページを解析してバージョンを抽出
    let re = Regex::new(r#"\[\[\["([\d\.]+)"\]\],\[\[\[\d+\]\],\[\[\[\d+,"#).unwrap();
    // ...
```

そして[`login.rs`](crates/LinkLike_Core/src/fetch/login.rs)

```rust
pub async fn login(client_version: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = json!({
        "device_specific_id": "",
        "player_id": "",
        "version": 1
    });
    
    // レスポンスヘッダーからx-res-versionを抽出
    let res_info = res
        .headers()
        .get("x-res-version")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
}
```

## アセットの複合

マニフェストの取得した後は、実際のアセットファイルの復号化処理に入ります。

### 暗号化方式の解析

リンクラのアセットファイルは独自の暗号化方式を使用しています。

1. **固定プリフィックス** - 64バイトの固定値
2. **シード値** - マニフェストから取得
3. **CRC64** - ファイル名のCRC64ハッシュ
4. **CRC32** - プラットフォーム識別子（"android"）のCRC32
5. **可変長整数** - ファイルサイズのuvarint表現

これらを連結してSHA256でハッシュ化し、AES-128-CBCの鍵とIVを生成します

とても複雑ですね

```rust
pub fn decode_asset_with_data(manifest_data: &ManifestCryptoData, src: &[u8], dst: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    const PREFIX: &str = "c34ea77df4976cd8907096fa47bb97e61852305892494e3692ba0c7eb434f022c549c96cf7ca0ee1b6ba7f203b6c76e8679699ce9c44af7b1cb000173a515938";
    
    // 鍵導出バッファの構築
    let mut buf = Vec::new();
    buf.extend_from_slice(&hex::decode(PREFIX)?);
    buf.extend_from_slice(&manifest_data.seed.to_be_bytes());
    // ...
    
    // SHA256でキーとIVを生成
    let mut hasher = Sha256::new();
    hasher.update(&buf);
    let keyiv = hasher.finalize();
    
    let key = &keyiv[..16];
    let iv = &keyiv[16..];
}
```

### LZ4圧縮の処理

複合化後のデータはLZ4で圧縮されている場合がありました。

- 標準的なLZ4フレーム形式
- サイズプリペンド形式
- カスタムヘッダー形式

```rust
pub fn try_lz4_decompress_detailed(data: &[u8], expected_size: u64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // LZ4フレーム形式を試行
    if let Ok(result) = try_lz4_frame_decompress(data) {
        return Ok(result);
    }
    
    // カスタム形式を試行
    if let Ok(result) = try_custom_lz4_decompress(data, expected_size) {
        return Ok(result);
    }
    
    // サイズプリペンド形式を試行
    if let Ok(result) = lz4_flex::decompress_size_prepended(data) {
        return Ok(result);
    }
}
```

## データ形式の解析

### TSVファイルの処理

ゲームのマスターデータは主にTSV（Tab-Separated Values）形式で管理されています。[`parse.rs`](crates/LinkLike_Core/src/master/parse.rs)では、これらをJSONに変換する機能を提供しています：

```rust
pub fn parse_tsv_from_bytes(data: &[u8]) -> Result<Vec<Map<String, Value>>, Box<dyn std::error::Error>> {
    let text = String::from_utf8(data.to_vec())?;
    let lines: Vec<&str> = text.lines().collect();
    
    // ヘッダー行の解析
    let headers: Vec<&str> = lines[0].split('\t').collect();
    
    // データ行の変換
    for line in lines.iter().skip(1) {
        let values: Vec<&str> = line.split('\t').collect();
        let json_value = infer_json_value(value_str);
        // ...
    }
}
```

### 楽曲データの構造

特に苦労したのはこちらの音ゲーの譜面データです。これらは独自のバイナリ形式で保存されており、復号化後にJSONとして解析できます。

譜面ファイルは`rhythmgame_chart`で始まる名前を持ち、以下のような情報を含んでいると推測しました。：

- 楽曲ID
- 難易度レベル
- ノーツデータ（タイミング、位置、種類）
- BPM情報
- 楽曲メタデータ

## 自動化システム

### 差分更新の仕組み

[`auto_update.rs`](crates/LinkLike_Core/src/fetch/auto_update.rs)では、差分更新システムを実装しています：

```rust
pub async fn auto_update(&self, options: UpdateOptions) -> Result<UpdateResult, Box<dyn std::error::Error>> {
    // 現在のバージョンをチェック
    let current_version = self.read_current_version()?;
    
    if !options.force && res_info == current_version {
        println!("Nothing updated, stopping process.");
        return Ok(UpdateResult::NoUpdate);
    }
    
    // カタログの差分処理
    let catalog_processor = CatalogProcessor::new(self);
    let mut filtered_catalog = if options.force {
        catalog
    } else {
        catalog_processor.process_catalog_diff(catalog).await?
    };
}
```

[`catalog_processor.rs`](crates/LinkLike_Core/src/fetch/catalog_processor.rs)では、前回のカタログと比較して変更されたファイルのみをダウンロード対象とします：

```rust
pub async fn process_catalog_diff(&self, mut catalog: Catalog) -> Result<Catalog, Box<dyn std::error::Error>> {
    // 古いカタログを読み込み
    let old_entries = self.read_from_json_file::<Vec<Entry>>(&self.updater.catalog_json_file_prev)
        .unwrap_or_else(|_| Vec::new());
    
    // 差分を計算
    catalog.diff(&old_catalog);
}
```

### 並列ダウンロード

[`downloader.rs`](crates/LinkLike_Core/src/fetch/downloader.rs)では、セマフォを使用した並列ダウンロードを実装し、効率的なファイル取得を実現しています：

```rust
pub async fn download_assets_async(catalog: &Catalog, download_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENCY));
    
    let download_tasks = stream::iter(&catalog.entries)
        .map(|entry| {
            let semaphore = Arc::clone(&semaphore);
            async move {
                let _permit = semaphore.acquire().await.unwrap();
                download_one(&client, &entry, &download_dir, &headers, &counter, dl_amount).await
            }
        })
        .buffer_unordered(MAX_CONCURRENCY);
}
```

## まとめ

この解析により、リンクラのアセット管理システムの全体像が明らかになりました：

1. **バージョン管理** - Google Playからの自動バージョン取得とx-res-versionによるリソース管理
2. **暗号化システム** - 複雑な鍵導出とAES-128-CBC暗号化
3. **圧縮処理** - 複数のLZ4フォーマットへの対応
4. **データ形式** - TSVベースのマスターデータと独自の譜面フォーマット
5. **効率化** - 差分更新と並列ダウンロードによる最適化

これらの知見により、ゲームのアセット管理の仕組みを理解し、効率的なデータ取得と解析が可能になりました。

今後の発展として、譜面エディターや創作ツールの開発にこれらの技術を活用できるんじゃないでしょうか。


以上です。
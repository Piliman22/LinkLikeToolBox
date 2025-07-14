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

写真を撮りそびれたので文章でとなりますが、初回ログイン時にはほとんど空の内容を`POST`しています。


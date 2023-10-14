<div align="center">
    <img src="./doc/img/title.png">
</div>

# WinMobileController
windows端末をモバイル端末からマウスキーボード操作するためのアプリケーション
# SystemDesign
![alt](./doc/img/SystemDesign.drawio.svg)
# Setup
## ソースコードのクローン
* 本リポジトリをCloneしてください。
## Mosquittoのインストール
* 以下のリンクよりMosquittoをインストールしてください。
https://mosquitto.org/download/
## OpenSSLのインストール
* 以下のリンクよりOpenSSLをインストールしてください。
https://slproweb.com/products/Win32OpenSSL.html

* 以下のpathを通してください
set OPENSSL_DIR=C:\OpenSSL-Win64
※おそらく微妙にinstallされるパスがちがうので適宜変更

## TLS対応
### 自己認証局
./pem/caフォルダ内にて以下のコマンドを実行します。
秘密鍵の生成
```
openssl genrsa -des3 -out ca.key
```
CA証明書の生成
```
openssl req -new -x509 -days 1826 -key ca.key -out ca.crt
```

### サーバ証明書
./pem/serverフォルダ内にて以下のコマンドを実行します。
秘密鍵生成
```
openssl genrsa -out server.key 2048
```
CSR発行＆CA署名実行
※CommonNameには適当な値を入れてok
※subjectAltName属性にブローカを公開するIPアドレスを指定してください。
※クライアント側が証明書を判定する際にv3のSANフィールドのIPアドレスを参照するため必須
```
openssl req -x509 -newkey rsa:2048 -keyout server.key -out server.crt -days 1826 -addext "subjectAltName = IP:<ここにブローカのIPを指定>" -CA ../ca/ca.crt -CAkey ../ca/ca.key -nodes
```

### クライアント証明書
PCとスマホ用に2つのクライアント証明書を作ります。
./pem/win-clientフォルダ、./pem/mob-client内フォルダにて以下のコマンドを実行します。
秘密鍵生成
```
openssl genrsa -out <winまたはmob>-client.key 2048
```

CSR発行＆CA署名実行
※CommonNameには適当な値を入れてok
※クライアント証明書ではIPアドレスを指定できないので入れない。
```
openssl req -x509 -newkey rsa:2048 -keyout <winまたはmob>-client.key -out <winまたはmob>-client.crt -days 1826 -CA ../ca/ca.crt -CAkey ../ca/ca.key -nodes
```
PEM生成
```
cat <winまたはmob>-client.key <winまたはmob>-client.crt > <winまたはmob>-client.pem
```

### 証明書の配置
#### Mosquitto
```
C:\Program Files\mosquitto\certs　
```
直下にserver.crt，server.key，ca.crtを格納する。

#### Flutter
```
view/assets
```
Flutter側のコードではアプリケーション内部に証明書を組み込む。よって上記pathへca.crt, mob-client.crt, mob-client.keyを格納する。

### Mosquitto設定変更
```
C:\Program Files\mosquitto\mosquitto.conf
```
上記ファイルを修正します。
```
# ポート待ち受け
listener 8883
# 各種証明書
cafile C:/Program Files/mosquitto/certs/ca.crt
keyfile C:/Program Files/mosquitto/certs/server.key
certfile C:/Program Files/mosquitto/certs/server.crt
# クライアント証明書の検証
require_certificate true
use_identity_as_username true
```

※TLS対応せずに単純に外部から接続可能にする場合
```
# ポート待ち受け
listener 1883
# ログイン認証無効化
allow_anonymous true
```
### ファイアウォール設定
ポート番号に対してインバウンドTCPの穴あけを行う。

## ビルド
### Flutter
以下コマンドでapkファイルを生成します。
```
flutter build apk
```

USB接続して以下コマンドでインストールします。
```
flutter install build\app\outputs\flutter-apk\app-release.apk
```
### tauri
以下コマンドでmsiファイルを生成します。
```
cargo tauri build
```

生成したmsiファイルを使ってinstallする。

## 管理者権限付与
インストールしたtauri製のwindowsアプリのプロパティから常に管理者権限で実行するように設定する。

# Start App
## Windowsサイド起動
* ビルド済みのexeファイルを実行する。Setupで管理者権限を付与しているので自動的に管理者実行になる。

## Mobileサイド起動
* モバイル端末にインストールしたアプリを実行する。


# Auther
* Kano

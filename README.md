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

CSR発行
※Common Nameにはブローカを配置する端末のIPアドレスorドメインを入力してください。
```
openssl req -new -out server.csr -key server.key
```

CA署名実行
```
openssl x509 -req -in server.csr -CA ../ca/ca.crt -CAkey ../ca/ca.key -CAcreateserial -out server.crt -days 1826
```

### クライアント証明書
PCとスマホ用に2つのクライアント証明書を作ります。
./pem/win-clientフォルダ、./pem/mob-client内フォルダにて以下のコマンドを実行します。
※クライアント証明書のCommon Nameは適当に識別可能な文字列を入れておけば良いです。
秘密鍵生成
```
openssl genrsa -out <winまたはmob>-client.key 2048
```

CSR発行
※Common Nameにはブローカを配置する端末のIPアドレスorドメインを入力してください。
```
openssl req -new -out <winまたはmob>-client.csr -key <winまたはmob>-client.key
```

CA署名実行
```
openssl x509 -req -in <winまたはmob>-client.csr -CA ../ca/ca.crt -CAkey ../ca/ca.key -CAcreateserial -out <winまたはmob>-client.crt -days 1826
```

### 証明書の配置
### Mosquitto設定変更

## ビルド
*

## 管理者権限付与
*
# Start App
## バッチの実行
* bat/start_mosquitto.bat　を管理者権限で実行する。

## Windowsサイド起動
* ビルド済みのexeファイルを実行する。Setupで管理者権限を付与しているので自動的に管理者実行になる。

## Mobileサイド起動
* モバイル端末にインストールしたアプリを実行する。
# Auther
* Kano

# カウンター

デプロイする度にインクリメントされる、シンプルなスマートコントラクトです。

## 使い方

### Rust toolchainのセットアップ
スマートコントラクトの開発の為に、以下のコマンドで準備を行います。
```bash
$ make prepare
```

### スマートコントラクトのコンパイル
スマートコントラクトを、WASMファイルにコンパイルします。
```bash
$ make build-contract
```
### テストの実行
以下のコマンドで、integrationテストを実行することができます。
```bash
$ make test
```

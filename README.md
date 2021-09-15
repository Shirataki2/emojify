# Emojify

文字を引数にとって絵文字として使える画像を生成するBot

Rust製

![](https://i.gyazo.com/971f91eb1f8b3b90364875f551a5f539.png)

ローカルで動かす際はPythonにUvicorn, FastAPI, Pillowをインストールし，`./app` 内のConfig.Example.tomlをConfig.tomlに改名し，適宜欄を埋めてください．

内部サーバーを動かす場合は`./inner_api`で`uvicorn main:app --port 4444`のように実行します．

Botを動かす場合は`./app`で`cargo run --release`のように実行します．

# Rustでのロギング

Rustでのログを表示させる方法をいくつか調べてみます。

## log crate
Rustの[log](https://crates.io/crates/log)crateに`info!`等のマクロが定義されてますが、用途に応じてその実装を選べる作りになっているようです。

### log実装
[Available loggin implementations]( https://docs.rs/log/0.4.6/log/#available-logging-implementations)の転載です。

* Simple minimal loggers:
  * env\_logger
  * simple\_logger
  * simplelog
  * pretty\_env\_logger
  * stderrlog
  * flexi\_logger
* Complex configurable frameworks:
  * log4rs
  * fern
* Adaptors for other facilities:
  * syslog
  * slog-stdlog<Plug>_

### 参考記事
* [Rustでライブラリのデバッグをする](https://qiita.com/rejasupotaro/items/e45fe64623ac7462e2a9)

## env\_logger ver.
logの実装でどれを使えばいいのか分かりませんが、[crates.io](https://crates.io)によると[`env_logger`](https://docs.rs/env_logger/0.6.1/env_logger/)のダウンロード数が圧倒的に多いようです。

### 参考記事
[Rust：logでログ出力を行う](https://qiita.com/fujitayy/items/590145c0f4b4e7d06de7)

```Rust
#[macro_use]
extern crate log;
extern crate env_logger as logger;

use log::Level;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "info");
    // env::set_var("RUST_LOG", "trace");
    logger::init();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}
```
実行ファイルの名前がmainとすると、次のような実行結果が得られます。デフォルトのログレベルは`Error`で、標準エラー出力に書き出されます。ソースコードの中で環境変数の`RUST_LOG`を`info`に設定しているため`error`と`info`は出力されますが、`debug`は何も出ませんね。
```
$ ./main
[2019-03-24T14:16:11Z ERROR log_sample] this is printed by default
[2019-03-24T14:16:11Z INFO  log_sample] the answer was: 12
```

## simple\_logger ver.
logの実装でどれを使えばいいのか分かりませんが、[crates.io](https://crates.io)によると[`env_logger`](https://docs.rs/env_logger/0.6.1/env_logger/)のダウンロード数が圧倒的に多いようです。

### 参考記事
[Rust：logでログ出力を行う](https://qiita.com/fujitayy/items/590145c0f4b4e7d06de7)

```Rust
#[macro_use]
extern crate log;
extern crate env_logger as logger;

use log::Level;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "info");
    // env::set_var("RUST_LOG", "trace");
    logger::init();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}
```
実行ファイルの名前がmainとすると、次のような実行結果が得られます。デフォルトのログレベルは`Error`で、標準エラー出力に書き出されます。ソースコードの中で環境変数の`RUST_LOG`を`info`に設定しているため`error`と`info`は出力されますが、`debug`は何も出ませんね。
```
$ ./main
[2019-03-24T14:16:11Z ERROR log_sample] this is printed by default
[2019-03-24T14:16:11Z INFO  log_sample] the answer was: 12
```

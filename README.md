Rustで使えるロガーを調べてみます。この記事ではいくつかある実装のうち、
* env\_logger
* simple\_logger
* stderrlog
を動かしてみました。

## log crate
Rustの[log crate](https://crates.io/crates/log)に`info!`等のマクロが定義されてますが、用途に応じてその実装を選べる作りになっているようです。

### log実装の一覧
[Available loggin implementations](https://docs.rs/log/0.4.6/log/#available-logging-implementations)の転載です。

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

## env\_logger
logの実装でどれを使えばいいのか分かりませんが、[crates.io](https://crates.io)によると[`env_logger`](https://docs.rs/env_logger/0.6.1/env_logger/)のダウンロード数が圧倒的に多いようです。


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

実行ファイルの名前がmainとすると、次のような実行結果が得られます。`env_logger`のデフォルトログレベルは`Error`で、標準エラー出力に書き出されます。ソースコードの中で環境変数の`RUST_LOG`を`Info`に設定しているため、ここでは`error`と`info`は出力されていますが、`debug`は何も出ませんね。

```
$ ./main
[2019-03-24T14:16:11Z ERROR log_sample] this is printed by default
[2019-03-24T14:16:11Z INFO  log_sample] the answer was: 12
```

## simple\_logger
crateの名前の通り、シンプルに標準出力に書き出します

```Rust
#[macro_use]
extern crate log;
extern crate simple_logger as logger;

use log::Level;

fn main() {
    // logger::init().unwrap();
    logger::init_with_level(Level::Info).unwrap();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}
```
実行ファイルの名前がmainとすると、次のような実行結果が得られます。`simple_logger`のデフォルトログレベルはtraceですが、そのレベルは簡単に変更できるみたいです。多くのloggerの実装ではこのようなinit関数が用意されているとのことです。

```
$ ./main
2019-03-24 23:32:48 ERROR [log_sample] this is printed by default
2019-03-24 23:32:48 INFO  [log_sample] the answer was: 12
```

## stderrlog

もう一つくらい、[stderrlog](https://docs.rs/stderrlog/0.4.1/stderrlog/)試してみます。

```Rust
#[macro_use]
extern crate log;
extern crate stderrlog as logger;

use log::Level;

fn main() {
    logger::new()
        .module(module_path!())
        .verbosity(2)
        .init()
        .unwrap();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}
```

実行結果は次のようになります。タイムスタンプを入れたりもっと細かい設定もできますが、ここではサボっています。実行結果は次のようになります。

```
$ ./main
ERROR - this is printed by default
INFO - the answer was: 12
```
注意点としては、`stderrlog`のverbosityがログレベルに相当するのですが、`log`crateのLovelとは微妙にずれているので`verbosity(Level::Info as usize)`等とすると期待する結果とずれてしまいました。ソースコードで確認しといたほうが良さそうですね。

* [logのLevel](https://github.com/rust-lang-nursery/log/blob/v0.4.6/src/lib.rs#L330:L351)
* [stderrlogのverbosity](https://github.com/cardoe/stderrlog-rs/blob/v0.4.1/src/lib.rs#L388:L400)

## 参考にした記事など
* [Crate log](https://docs.rs/log/0.4.6/log/)
* [Rustでライブラリのデバッグをする](https://qiita.com/rejasupotaro/items/e45fe64623ac7462e2a9)
* [Rust：logでログ出力を行う](https://qiita.com/fujitayy/items/590145c0f4b4e7d06de7)



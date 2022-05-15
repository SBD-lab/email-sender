# Email notification on Windows login with Task Scheduler
## 開発環境
- Azure VM Linux (ubuntu 20.04)
- rustc 1.60.0 (7737e0b5c 2022-04-04)
- Visual Studio Code (ローカル PC から SSH 接続)
- x86_64-pc-windows-gnu クロスコンパイル環境
## クロスコンパイル (Linux → Windows)
- 事前に以下のコマンドを実行し、クロスコンパイルの環境を作成する。
    - `rustup toolchain install stable-x86_64-pc-windows-gnu`
    - `rustup target add x86_64-pc-windows-gnu`
    - `sudo apt install -y mingw-w64`
- .cargo/config にてクロスコンパイルの定義をしているので、そのままクロスコンパイル可能。
    - `cargo build --release`
- scp コマンドでローカル PC にコピーする。
    - `scp <username>@<remote-hostname>:/home/<username>/<to sorce dir...>/email-sender/target/x86_64-pc-windows-gnu/release/email-sender.exe .`
## タスクスケジューラの設定
- 以下のサイトを参考にタスクスケジューラでビルドしたファイルをログオン時に実行するように設定する。
- https://4thsight.xyz/33437#%E3%82%BF%E3%82%B9%E3%82%AF%E3%82%B9%E3%82%B1%E3%82%B8%E3%83%A5%E3%83%BC%E3%83%A9%E3%82%92%E4%BD%BF%E3%81%A3%E3%81%9F%E6%96%B9%E6%B3%95
- 引数の指定は以下の通りである。
    - `-e <通知先メールアドレス> -p <ログインパスワード>`
    - 二段階認証を設定しているアカウントの場合、アプリパスワードを設定する。
    - (参考) Gmail：https://support.google.com/mail/answer/185833?hl=ja

```
# SQLx CLIをインストール
cargo install sqlx-cli

# データベースの準備（.envファイルを使って）
sqlx database create
sqlx migrate add create_users_table
# ここで生成されたマイグレーションファイルに必要なSQLを記述
sqlx migrate run
```
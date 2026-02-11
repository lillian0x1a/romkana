# romkana

`romkana` は、**ローマ字をひらがなに変換する Rust ライブラリ**および **CLI ツール** です。  
リアルタイム入力変換やカスタム変換テーブルの利用も可能です。

## 特徴
- Rust で簡単に使えるライブラリ
- ターミナルでリアルタイムに変換
- カスタム CSV テーブルをサポート
- Windows / macOS / Linux 対応（crossterm 使用）

## インストール
### ライブラリとして利用

`Cargo.toml` に追加：
```toml  
[dependencies]
romkana = "0.1.0"
```
### CLI として利用
```bash
cargo install romkana
```
インストール後はターミナルで以下を実行できます：
```bash
romkana
```
## 使い方
### ライブラリ
```rust
use romkana::RomKana;

fn main() -> std::io::Result<()> {
    let engine = RomKana::new("romaji.csv")?;
    println!("{}", engine.convert("konnichiha")); // こんにちは
    Ok(())
}
```

### CLI
起動後、ターミナル上で文字を入力するとリアルタイムでローマ字がかなに変換されます。
- Esc: 終了
- Backspace: 文字削除
- 入力中の文字は即座にひらがなに変換
```bash
cargo run --bin romkana
```

## CSV テーブルのフォーマット
変換テーブルは CSV で管理されています。フォーマットは以下の通り：
```csv
romaji,hiragana[,next_state]
a,あ
ka,か
shi,し
```
- 1列目: ローマ字
- 2列目: 変換後のかな
- 3列目（任意）: 次の状態（連続変換用）

### 例
```rust
use romkana::RomKana;

let engine = RomKana::new("romaji.csv").unwrap();

assert_eq!(engine.convert("konnichiha"), "こんにちは");
assert_eq!(engine.convert("arigatou"), "ありがとう");
```

### 注意点
- CSV ファイルは UTF-8 で保存してください
- ファイルパスが間違っているとエラーになります
- CLI は crossterm を使っているため、ターミナルでのみ動作します
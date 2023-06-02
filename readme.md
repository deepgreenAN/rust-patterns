# Rustでデザインパターン

[こちらのサイト](https://github.com/fadeevab/design-patterns-rust)をベースに，binには他の実装なども

## 含めないパターン

- Mementパターン: Rustにはserdeに代表されるシリアライゼーション・デシリアライゼーションライブラリがあるため、そちらを利用すべき

## Rust特有の特徴

### 所有権についての特徴

- 所有権があるため、オブジェクト間で相互の参照を保持したり下位のオブジェクトが上位のオブジェクトの参照を保持するのは内部可変性を用いないと困難である．そのため上位のみが下位を所有するトップダウンの構造が強制される．これはデザインパターンにとっては制約であるが，構造の見通しが良くなる．
- 同じ理由で上位→下位の呼び出しが基本であり，下位→上位の呼び出しや横並びの呼び出しには以下のような制約がある．
  - 下位→上位の呼び出し: 下位のメソッドの引数に上位の共有参照・可変参照を渡してそのメソッドを上位から呼び出す必要がある．可変参照を渡す場合は，一度下位のオブジェクトの所有権を上位のオブジェクトから奪わなければならない．
  - 横並びの呼び出し: 上位から呼び出さない場合は、Boxを用いて自己参照構造体かトレイトオブジェクトを用いて所有権を保持する必要がある．これは厳密な横並びではなく同じ型やトレイトを実装した型に対して所有の上下関係を強制する．共有参照・可変参照を使いたい場合はメソッドの引数にして上位から呼び出す必要がある．
- 副次的なメリットとして，下位オブジェクトから上位オブジェクトを利用しない実装が自然となり，その結果上位オブジェクトをトレイトなどで抽象化しなくてもよくなる．(下位オブジェクトは上位オブジェクトを知らなくても良い)

### 抽象化についての特徴

- トレイトによるポリモーフィズムによって既存の型を拡張できる．これによって外部のよく使われている型を拡張したり，型に複数の役割を持たせることをができる．その場合は単一責任の原則を破らないよう注意する．
- 型パラメータ―に処理を部分的に委譲させることで、実装する側では同じインターフェースを持つ(トレイトを併用するのが望ましい)別々の型を，変更部分を最小限にとどめて実装できる．`impl<T> Trait for MyType<T>`のようにしてトレイトを実装しておけばユーザー側もインターフェースに対して実装できる．
- クロージャーや関数ポインタをフィールドとして保持できる．このため一つのメソッドのみを持つ型やトレイトを定義する必要性が少ない．
- `From`，`AsRef`，(`Deref`)トレイトがあるため、型の変換インターフェースが明確である．型の変換が積極的に外部インターフェースに公開されることをデザインで考慮できる．

### その他言語仕様の特徴

- マクロによってボイラープレートを削減できる．ポイラープレートの多さによって忌避された方法が選択肢に上がり，本当に変更すべき部分に注目できる．
- 厳密なコンパイル時チェックによって、他の言語においてバグを減らすための抽象化が必要ないときがある．列挙体などを使えば適切な変更を将来的に強制できる．

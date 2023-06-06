# Rustでデザインパターン

[こちらのサイト](https://github.com/fadeevab/design-patterns-rust)をベースに，binには他の実装なども

## 含めないパターン

- Iteratorパターン: RustにはビルトインのIteratorトレイトがあるため，そちらを利用する
- Mementパターン: Rustにはserdeに代表されるシリアライゼーション・デシリアライゼーションライブラリがあるため、そちらを利用する
- Strategyパターン: Rustでは関数を関数ポインタ・型パラメーターを用いた具象型・トレイトオブジェクトとしてフィールドに保持できるため、そちらを利用する(Strategyパターンの使い方としては関数ポインタかトレイトオブジェクトが利用できる)
- Template Methodパターン: トレイトの一般的な使い方である．
- Visitorパターン: Rustのトレイトは既存の型に実装することが可能であるため，そちらを利用する．元の型の実装を壊すことなく，新しい機能を追加できる(既存のデータ構造・アルゴリズムと新しい機能を別々の型に分ける必要がない)．スコープにトレイトをインポートするかどうかによって機能を絞り込むこともできるため，単一責任の原則にも可能な限り対応できる．
- Builderパターン: Rustでは型に対するビルダーパターンをよく用いる．[APIガイド](https://rust-lang.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder)に簡単な例が書かれており、非消費ビルダーであれば基本的にtyped-builder(コンパイル時に重複や不足をチェック)やderive-builderクレート(動的に重複や不足をチェック)を利用する．
- Prototypeパターン: Rustにはビルトインのクローントレイトがあるためそちらを利用する
- Singletonパターン: Rustにはlazyなグローバル変数が利用できるonce-cellクレートがあるため，そちらを利用する．値を変更したい場合は内部可変性のあるスマートポインタを一緒に用いる．

## Rust特有の特徴

### 所有権についての特徴

- 所有権があるため、オブジェクト間で相互の参照を保持したり下位のオブジェクトが上位のオブジェクトの参照を保持するのは内部可変性を用いないと困難である．そのため上位のみが下位を所有するトップダウンの構造が強制される．これはデザインパターンにとっては制約であるが，通常は構造の見通しが良くなる．
- 同じ理由で上位→下位の呼び出しが基本であり，下位→上位の呼び出しや横並びの呼び出しには以下のような制約がある．
  - 下位→上位の呼び出し: 下位のメソッドの引数に上位の共有参照・可変参照を渡してそのメソッドを上位から呼び出す必要がある．加えて可変参照を渡す場合は，一度下位のオブジェクトの所有権を上位のオブジェクトから奪わなければならない．
  - 横並びの呼び出し: 上位から呼び出さない場合は、Boxを用いて自己参照構造体かトレイトオブジェクトを用いて所有権を保持する必要がある．つまり厳密な横並びではなく同じ型やトレイトを実装した型に対して所有の上下関係を強制する．共有参照・可変参照を使いたい場合はメソッドの引数にして上位から呼び出す必要がある．
- 副次的なメリットとして，下位から上位を利用しない実装が自然となり，その結果上位をトレイトなどで抽象化しなくてもよくなる．(下位は上位を知らなくても良い)

### 抽象化についての特徴

- トレイトによるポリモーフィズムによって既存の型を拡張できる．これによって外部のよく使われている型を拡張したり，型に複数の役割を持たせることをができる．その場合は単一責任の原則を破らないよう注意する．
- トレイトは最適限要求するメソッドの返り値を除いて具体的なデータにアクセスできない．そのため抽象基底クラスのようにデータを用いた複雑なロジックを記述する場合は要求するメソッドに汎用性があるかどうか検討する必要がある．汎用性が無くなる場合は，具象型を使うことを検討する．
- Rustにおいてトレイトで抽象化したオブジェクトコンポジションを行う場合、トレイトオブジェクトを利用する場合を除いて型パラメータ―で静的ディスパッチを行う必要がある．これはクライアント側において，そのトレイトを実装した型への理解をある程度強制する．トレイトオブジェクトを利用したとしても，トレイトにコンストラクタを含めることはできない(オブジェクトセーフでない)ため，結局外部からそのトレイトを実装した型を与える必要がある．
- クロージャーや関数ポインタをフィールドとして保持できる．このため一つのメソッドのみを持つ型やトレイトを定義する必要性が少ない．
- `From`，`AsRef`，(`Deref`)トレイトがあるため、型の変換インターフェースが明確である．型の変換をインターフェースとして組み込める．

### その他言語仕様の特徴

- マクロによってボイラープレートを削減できる．ポイラープレートの多さによって忌避された方法が選択肢に上がり，本当に変更すべき部分に注目できる．
- 厳密なコンパイル時チェックによって、他の言語においてバグを減らすための抽象化が必要ないときがある．列挙体などを使えば適切な変更を将来的に強制できる．

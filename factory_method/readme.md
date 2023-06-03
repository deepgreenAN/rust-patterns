# ファクトリーメソッド

## 説明

- プロダクトに関するロジックは抽象クリエイターで定義し，具体的にどのクラスを作成するかについてはサブクラスがファクトリメソッドを実装することで決定する．つまり抽象クリエイターはそのサブクラスにプロダクトのインスタンス化を先送りする．
- 変更しにくいロジックと，変更が起きやすいプロダクトとその作成部分を分離することができる．
- さらに、下位コンポーネントに加えて上位コンポーネントも抽象プロダクトに対してプログラミングできる．これを依存性の反転という．この依存性の反転を実現するには依存性の注入が有効であるが，ファクトリメソッドにはその型で完結し，サブクラスによってカスタマイズできるというメリットがある．
- Rustでこれを実現するには素直にクリエイタートレイトとプロダクトトレイトを用いる方法と、`Creator<T, S>`(`Product<T>`はどちらでもよい)とトレイト境界用のトレイトを用いる方法の二つが考えられる．
  - クリエイタートレイトとプロダクトトレイトを用いる方法
  - 抽象プロダクト・抽象クリエイターをトレイトとして定義する．分かりやすい実装だが，トレイトにロジックを記述する必要があるため，フィールドを参照することができずデータの取得のためのメソッドが増えてしまう．結果として新しくクリエイターを作成し実装するときにファクトリ以外のメソッドが要求されることになる．
  - 具象プロダクトを表す型パラメータ―とファクトリを表す型パラメータ―を持った具象クリエイターを実装する．トレイト境界を設けるためプロダクトトレイト，ファクトリトレイトが必要となる．新しくクリエイターを増やしたければ，新しいファクトリにファクトリトレイトを実装するだけでよい．

ここまで複雑な実装を行わなくても，Rustではジェネリックなフィールドが扱えるため，具象クリエイター(場合によっては型パラメータ―を持つ)に対して依存性の注入を行って実現するのが自然である．注入するのが関数であれば，結果として関数をカスタムできるシンプルファクトリのようになる．関数でなくもConfigやビルダーなどを渡すことも考えられる．

mainは今回は依存性の注入を用いた．またプロダクトにも型パラメータ―を導入した．
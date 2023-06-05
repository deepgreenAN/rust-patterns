# コンポジットパターン

## 説明

- ツリー構造を持ち、末端の要素と子を持つ要素を区別したくないときのパターン．
- Rustは列挙体にメソッドを実装できるため，ノードを列挙体とすることによって、クライアントは子を持つかどうかを区別せずに利用できる．
- Rustは所有権があるため，子を保持したコンポジットをそのまま所有権を奪ってイテレーション・可変参照をイテレーションができない(参照のイテレーションは可能)．リーフは所有権を奪ってイテレーションが可能(可変参照をイテレーションはできない)．
- リーフはあらかじめ種類が分かっているなら列挙体を、クライアントに定義させたいならトレイトオブジェクトを用いて実装する．
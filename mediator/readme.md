# Mediatorパターン

## 説明

- 各コンボーネント間の結合を粗にして、コンポーネントは管制塔のようなMediatorとのみ依存するようにする．
- コンポーネントは`Mediator`への連絡の仕方`Mediator::notify_*`を知っていて，自分のロジックの中でその連絡の返答を利用する．
- 一方でMediatorは連絡を受けると(`Mediator::notify_*`の定義内で)そのコンポーネント以外のコンポーネントを利用して整合性をとる．それによって間接的にコンポーネント間の連絡を実現する．
- rustの所有権を満たすため，`Mediator`が各コンポーネントを所有し、各コンポーネントから`Mediator`への連絡の際はそのコポーネントの所有権を一時的に`Mediator`から奪う必要がある．
- 外部インターフェースはコンポーネントを所有する`Mediator`を実装した型に対して実装するが(外部インターフェース用のメソッドを`Mediator`トレイトを通さなくてもよい)，そのメソッドの実装内では各コンポーネントを直接利用する．(コンポーネント間の相互作用自体は`Mediator`トレイトの実装自体に押し込められるため，考えなくてよい．)

### メリット

- 各コンポーネントは自身のロジックの他に`Mediator`トレイトを実装した型との連絡とその結果に注意すればよい
- 各コンポーネントは連絡を取りたいコンポーネントの具象型やその実装を知っている必要がないため，他のシチュエーションや他の`Mediator`で使いまわせる．
- rustでは所有権があるため上から下への連絡は簡単だが，下から上への連絡や横並びの連絡は制約があり，何か設計パターンを必要とする．`Mediator`パターンはその一つである．

### デメリット

- コンポーネント間の連絡を体系的にまとめあげないと`Mediator`トレイトの数が多くなり，実装する型が巨大になる．

### Rustの基本的な構文

- if文ではなくif式になっていて必ず同じ型を返り値として受け取る必要がある

### match式
- パターンマッチの機能と一緒

```rust

fn pred(v: u32) -> Option<u32> {
  if v == 0 {
    None
  } else {
    Some(v - 1)
  }
}

fn print_pred(v: u32) {
  match pred(v) {
    Some(W) => {
      printIn!("pred({}) = {}", v, w)
    }
    None => {
      printIn!("pred({}) is undefined", v)
    }
  }
}

```

- `fn pred`はu32型の値の前の値（1を引いた値）をリターンする関数（u32型に0の前の値がないので0を受け取ったらNoneを返す）
- `match pred(v)`でパターンマッチを開始
- Someの場合に整数値wという変数に代入してマクロで結果を表示、Noneの場合はundefindを表示


### for文

- C言語と異なる点として繰り返し対象を明示的に指定する

### loop文
- Rustだとwhile, do whileがないのでloop文を使う

### 所有権

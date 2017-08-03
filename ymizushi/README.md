# 1.4 トークンの集合とLexerを拡張する

## 概要

このセクションでは、以下のTokenをLexing出来るようにする

* `==`
* `!`
* `!=`
* `-`
* `/`
* `*`
* `<`
* `>`
* `true`
* `false`
* `if`
* `else`
* `return`


以下の三種類のトークンを識別できるようにすることが必要である
* 1文字
* 二文字
* キーワード

## `nextToken` を実装する

`switch` 文で 入力一文字に対して分岐して、Tokenインスタンスを作る.


## `peekChar` を実装する
`l.position`  をインクリメントしないところ以外は `readChar()`  に似ている.

`peekChar` は `==` とか `!=` を lexing するために必要.

これがないと、例えば `Lexer` が 入力から==を受け取った時に `token.ASSIGN` を２つ作ってしまう.

これを回避するためにpeekCharを使う.


# 1.5 REPLの開始

`Monkey` 言語には `REPL` が必要.

`REPL` とは `Read Eval Print Loop` の略.

標準入力から `Lexer` に食わせるinput stringを受け取り、`nextToken` で `EOF` が来るまで、順番に呼び出して、結果を標準出力に出力することで `REPL` が実装できる.

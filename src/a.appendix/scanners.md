# `RESS` Scanners
In the initial implementation of the `ress` scanner, it was more important to get something working correctly than to have something blazing fast. To that end, the original `Scanner` performs a significant amount of memory allocation, which slows everything down quite a bit. To improve upon that `ress` offers a section option the `RefScanner`, which is a bit unfortunately named as it doesn't actually use any references. The `RefScanner` provides almost the same information as the `Scanner` but it does so without making any copies from the original javascript string, it the has the option to request the `String` for any `Item` giving the control to the user. Here is an example of the two approaches.

## Example JS
```js
{{#include ../../scanners_example/example.js}}
```

## Example Rust
```rust
{{#include ../../scanners_example/src/main.rs}}
```

## Output
```
{{#include ../../scanners_example/out.log}}
```

Let's look at token 7, the original token is `Token::Numeric(Number(String::From("1")))` while the ref token is `Token::Numeric(Number::Dec)`, both give similar information but the ref token doesn't allocate a new string for the text being represented, instead just informing the user that it is a decimal number. If you wanted to know what that string was, you could use the `RefScanner::string_for` method by passing it `RefItem.span`, this will return an `Option<String>` and so long as your span doesn't overflow the length of the js provided, it will have the value you are looking for. 
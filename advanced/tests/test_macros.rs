use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
/// 在 Rust 中宏分为两大类：声明式宏( declarative macros ) macro_rules! 和三种过程宏( procedural macros ):
/// #[derive]，在之前多次见到的派生宏，可以为目标结构体或枚举派生指定的代码，例如 Debug 特征
/// 类属性宏(Attribute-like macro)，用于为目标添加自定义的属性
/// 类函数宏(Function-like macro)，看上去就像是函数调用

macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut v = Vec::new();
          $(
              v.push($x);
          )*
          v
        }
    };
}

#[test]
fn test_vec() {
  let v = vec![1, 2, 3];
  println!("{:?}", v);
}

#[derive(HelloMacro)]
struct Sunfei;

#[test]
fn test_hello_macro() {
  Sunfei::hello_macro();
}

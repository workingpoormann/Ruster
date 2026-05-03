/*
    A reference provides a way to access another value
    without taking ownership of the value,
    and is also called “borrowing”.
    Shared references are read-only, and the referenced data cannot change.*

    参照はほかの値をアクセスできる方法を提供する
    その値からownership(?)を奪うことなく
    そして、この処理を "borrowing" と呼ぶ
    共有参照は読み込みのみで、共有参照されたデータは変更できない

    A shared reference to a type T has type &T.
    A reference value is made with the & operator.
    The * operator “dereferences” a reference, yielding its value.

    共有参照(T)は &T という型を持つ
    共有参照の値は &記号から作られる
    * 記号は その値 参照を奪い取る
*/
fn shared_reference() {
    println!("* shared_references\n");
    let a = 'a';
    let b = 'b';

    // cannot change
    let mut r: &char = &a; // borrowing
    dbg!(r);

    r = &b;
    dbg!(r);
}

/*
    Exclusive references, also known as mutable references,
    allow changing the value they refer to.
    They have type &mut T.

    排他的参照とは、可変参照とも呼ばれます、
    その値が参照している値を変更することを可能にする
    &mut T とします
*/
fn exclusive_references() {
    println!("* exclusive_references\n");
    // mutableな値 可変な値
    let mut point = (1, 2);

    // exclusive references, mutable references
    let x_coord = &mut point.0;

    // can change its value!
    *x_coord = 20;
    println!("point: {point:?}");
}

/*
    Slices borrow data from the sliced type.
*/
fn slices() {
    println!("* slices\n");
    let a: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("a: {a:?}");

    // borrow
    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");
}

/*
    We can now understand the two string types in Rust:
        &str is a slice of UTF-8 encoded bytes, similar to &[u8].
        String is an owned buffer of UTF-8 encoded bytes, similar to Vec<T>.*

    Rustにおける二つの文字列タイプを理解できるようになる
        &str: UTF-8 エンコードバイトです
            文字列スライスとも呼ばれたりメモリ上の他の場所に
            ある文字列データを参照するだけの型
            所有権を持たない（つまり変更不可能）

        String: UTF-8 バッファーされたバイト列
            ヒープ上に確保された文字列の所有者
            長さの変更、新しい文字列を構築できる
*/
fn strings() {
    println!("* strings\n");

    // can't change
    let s1: &str = "World";
    println!("s1: {s1}");

    // mutable
    let mut s2: String = String::from("hello");
    println!("s2: {s2}");

    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[2..9];
    println!("s3: {s3}");

    let s4: String = String::from("hoge");
    println!("{}", s4);
}

/*
    Rust enforces a number of rules for references
    that make them always safe to use.

    常にその参照を安全に使うためにたくさんのルールを強制しています

    One rule is that references can never be null,
    making them safe to use without null checks.

    一つのルールは、参照は絶対にNullにできません
    Null Checks をせずに安全に参照を使うことはできません

    The other rule we’ll look at for now is that references
    can’t outlive the data they point to.

    他のルールは、その参照が指してるデータは参照の外側で
    生きることはできません
*/
fn reference_validity() {
    println!("* strings\n");
    /*
    let x_ref = {
        let x = 10;
        &x
    }
    dbg!(x_ref);
    */
}

fn main() {
    shared_reference();
    exclusive_references();
    slices();
    strings();
    reference_validity();
}

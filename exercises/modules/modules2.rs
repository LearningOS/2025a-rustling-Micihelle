// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.




// use crate::delicious_snacks::fruits ::PEAR as fruit;
// use crate::delicious_snacks::veggies::CUCUMBER as veggie;
// crate到底是以一个什么样的形式存在？为什么这么定义依然对于 delicious_snacks::fruit,有 not found in `delicious_snacks`的错误？

mod delicious_snacks {
    // TODO: Fix these use statements
    pub use self::fruits::PEAR as fruit;   //这样写明明建立链接了 但是编译器依然无法识别该路径，为什么？
    pub use self::veggies::CUCUMBER as veggie; // ✅ re-export

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}

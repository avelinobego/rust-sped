#[macro_export]
macro_rules! hello {
    ($name:expr) => {
        println!("Olá, {}!", $name)
    };
}

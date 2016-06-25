macro_rules! full_path {
    ($local_path: expr) => {
        ::CRATE_PATH.to_owned() + "/" + $local_path
    }
}

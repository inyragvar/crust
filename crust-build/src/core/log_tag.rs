#[macro_export]
macro_rules! log_tag {
    () => {{
        #[allow(dead_code)]
        fn this() {}

        #[allow(dead_code)]
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name<T>()
        }
    }};
}
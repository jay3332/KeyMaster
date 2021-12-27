#[macro_export]
macro_rules! get_database {
    () => {
        crate::database::DATABASE_POOL
            .get()
            .expect("Failed to get database pool.")
    };
}

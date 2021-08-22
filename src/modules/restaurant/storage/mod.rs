pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

use crate::common::MainPooledConnection;

struct SqlStorage {
    connection: MainPooledConnection,
}

impl SqlStorage {
    fn new(connection: MainPooledConnection) -> Self {
        return Self { connection };
    }
}

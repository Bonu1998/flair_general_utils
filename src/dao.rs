use log::error;
use mysql::{
    prelude::{FromValue, Queryable},
    Pool, PooledConn,
};

#[derive(Debug)]
pub struct MySqlDao {
    pub conn: PooledConn,
}

impl MySqlDao {
    pub fn init(database_url: String) -> Result<MySqlDao, String> {
        match Pool::new(database_url.as_str()) {
            Ok(pool) => match pool.get_conn() {
                Ok(conn) => Ok(MySqlDao { conn }),
                Err(e) => Err(format!("{}", e)),
            },
            Err(e) => Err(format!("{}", e)),
        }
    }

    pub fn get<T: FromValue>(mut self, query: String) -> Vec<T> {
        match self.conn.query::<T, String>(query) {
            Ok(d) => d,
            Err(e) => {
                error!("\nMySqlDao get: {}", e);
                vec![]
            }
        }
    }
}

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::error::*;
use crate::schema;

use rusqlite::{Connection, OpenFlags};
use std::{ops::{Deref, DerefMut}, path::PathBuf, sync::{atomic::AtomicUsize, Arc}};

 pub struct FormsDb {
    writer: Connection,
    interrupt_counter: Arc<AtomicUsize>,
}
impl FormsDb {
    #[cfg(test)]
    pub fn new_memory(db_path: &str) -> Result<Self> {
        let name = PathBuf::from(format!("file:{}?mode=memory&cache=shared", db_path));
        Self::new_named(name)
    }

    fn new_named(db_path: PathBuf) -> Result<Self> {
        // We always create the read-write connection for an initial open so
        // we can create the schema and/or do version upgrades.
        let flags = OpenFlags::SQLITE_OPEN_NO_MUTEX
            | OpenFlags::SQLITE_OPEN_URI
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_READ_WRITE;

        let conn = Connection::open_with_flags(db_path.clone(), flags)?;

        init_sql_connection(&conn, true)?;
        Ok(Self {
            writer: conn,
            interrupt_counter: Arc::new(AtomicUsize::new(0))
        })
    }
}

impl Deref for FormsDb {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.writer
    }
}

impl DerefMut for FormsDb {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.writer
    }
}

fn init_sql_connection(conn: &Connection, is_writable: bool) -> Result<()> {
    define_functions(&conn)?;
    conn.set_prepared_statement_cache_capacity(128);
    if is_writable {
        let tx = conn.unchecked_transaction()?;
        schema::init(&conn)?;
        tx.commit()?;
    };
    Ok(())
}

fn define_functions(c: &Connection) -> Result<()> {
    use rusqlite::functions::FunctionFlags;
    c.create_scalar_function(
        "generate_guid",
        0,
        FunctionFlags::SQLITE_UTF8,
        sql_fns::generate_guid,
    )?;
    Ok(())
}

pub(crate) mod sql_fns {
    use rusqlite::{functions::Context, Result};
    use sync_guid::Guid as SyncGuid;

    #[inline(never)]
    pub fn generate_guid(_ctx: &Context<'_>) -> Result<SyncGuid> {
        Ok(SyncGuid::random())
    }
}

// Helpers for tests
#[cfg(test)]
pub mod test {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    // A helper for our tests to get their own memory Api.
    static ATOMIC_COUNTER: AtomicUsize = AtomicUsize::new(0);

    pub fn new_mem_db() -> FormsDb {
        let _ = env_logger::try_init();
        let counter = ATOMIC_COUNTER.fetch_add(1, Ordering::Relaxed);
        FormsDb::new_memory(&format!("test_forms-api-{}", counter)).expect("should get an API")
    }
}
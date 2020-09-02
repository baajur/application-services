/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::error::Result;
use rusqlite::Connection;

const CREATE_SHARED_SCHEMA_SQL: &str = include_str!("../sql/create_shared_schema.sql");

pub fn init(db: &Connection) -> Result<()> {
    create(db)?;
    Ok(())
}

fn create(db: &Connection) -> Result<()> {
    log::debug!("Creating schema");
    db.execute_batch(CREATE_SHARED_SCHEMA_SQL)?;

    Ok(())
}
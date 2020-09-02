/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// use crate::db::StorageDb;
// use crate::error::*;
// use std::path::Path;

// pub struct Store {
//     db: StorageDb,
// }

// impl Store {
//     /// Creates a store backed by a database at `db_path`. The path can be a
//     /// file path or `file:` URI.
//     pub fn new(db_path: impl AsRef<Path>) -> Result<Self> {
//         Ok(Self {
//             db: StorageDb::new(db_path)?,
//         })
//     }

// //     /// Creates a store backed by an in-memory database.
// //     #[cfg(test)]
// //     pub fn new_memory(db_path: &str) -> Result<Self> {
// //         Ok(Self {
// //             db: StorageDb::new_memory(db_path)?,
// //         })
//     // }
// }
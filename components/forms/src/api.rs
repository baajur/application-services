/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::error::*;
use rusqlite::{Connection, NO_PARAMS, Transaction};
use serde::Serialize;
use serde_derive::*;
use sql_support::{self, ConnExt};
use std::time;
use sync_guid::Guid;

#[derive(Debug, Clone, Hash, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(rename = "id")]
    pub guid: Guid,

    pub given_name: String,

    pub additional_name: String,

    pub family_name: String,

    #[serde(default)]
    pub organization: String,

    #[serde(default)]
    pub street_address: String,

    pub address_level3: String,

    pub address_level2: String,

    pub address_level1: String,

    pub postal_code: String,

    pub country: String,

    #[serde(default)]
    pub tel: String,

    #[serde(default)]
    pub email: String,

    #[serde(default)]
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub time_created: i64,

    #[serde(default)]
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub time_last_used: i64,

    #[serde(default)]
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub time_last_modified: i64,

    #[serde(default)]
    pub times_used: i64,

    // pub sync_change_counter: i64,
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> std::result::Result<i64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::de::Deserialize;
    // Invalid and negative timestamps are all replaced with 0. Eventually we
    // should investigate replacing values that are unreasonable but still fit
    // in an i64 (a date 1000 years in the future, for example), but
    // appropriately handling that is complex.
    Ok(i64::deserialize(deserializer).unwrap_or_default().max(0))
}

/*
    TODO:
        - get all
        - update
        - delete
        - check if deleted on create and/or update?
*/

fn add_address(tx: &Transaction<'_>, mut address: Address) -> Result<Address> {
    let d = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap_or_default();
    let now_ms = (d.as_secs() as i64) * 1000 + (i64::from(d.subsec_nanos()) / 1_000_000);

    address.guid = Guid::random();
    address.time_created = now_ms;
    address.time_last_used = 0;
    address.time_last_modified = now_ms;
    address.times_used = 0;

    tx.execute_named_cached(
        "INSERT OR IGNORE INTO addresses_data (
            guid,
            given_name,
            additional_name,
            family_name,
            organization,
            street_address,
            address_level3,
            address_level2,
            address_level1,
            postal_code,
            country,
            tel,
            email,
            time_created,
            time_last_used,
            time_last_modified,
            times_used,
            sync_change_counter
        ) VALUES (
            :guid,
            :given_name,
            :additional_name,
            :family_name,
            :organization,
            :street_address,
            :address_level3,
            :address_level2,
            :address_level1,
            :postal_code,
            :country,
            :tel,
            :email,
            :time_created,
            :time_last_used,
            :time_last_modified,
            :times_used,
            1
        )",
        rusqlite::named_params! {
            ":guid": address.guid,
            ":given_name": address.given_name,
            ":additional_name": address.additional_name,
            ":family_name": address.family_name,
            ":organization": address.organization,
            ":street_address": address.street_address,
            ":address_level3": address.address_level3,
            ":address_level2": address.address_level2,
            ":address_level1": address.address_level1,
            ":postal_code": address.postal_code,
            ":country": address.country,
            ":tel": address.tel,
            ":email": address.email,
            ":time_created": address.time_created,
            ":time_last_used": address.time_last_used,
            ":time_last_modified": address.time_last_modified,
            ":times_used": address.times_used,
        },
    )?;
    Ok(address)
}

fn get_address(conn: &Connection, guid: &Guid) -> Result<Option<Address>> {
    let sql = "
        SELECT
            guid,
            given_name,
            additional_name,
            family_name,
            organization,
            street_address,
            address_level3,
            address_level2,
            address_level1,
            postal_code,
            country,
            tel,
            email,
            time_created,
            time_last_used,
            time_last_modified,
            times_used
        FROM addresses_data
        WHERE guid = :guid";
    let address = conn.query_row(
        sql,
        &[guid.as_str()],
        |row| {
            Ok(Address {
                guid: Guid::from_string(row.get::<_, String>("guid")?),
                given_name: row.get::<_, String>("given_name")?,
                additional_name: row.get::<_, String>("additional_name")?,
                family_name: row.get::<_, String>("family_name")?,
                organization: row.get::<_, String>("organization")?,
                street_address: row.get::<_, String>("street_address")?,
                address_level3: row.get::<_, String>("address_level3")?,
                address_level2: row.get::<_, String>("address_level2")?,
                address_level1: row.get::<_, String>("address_level1")?,
                postal_code: row.get::<_, String>("postal_code")?,
                country: row.get::<_, String>("country")?,
                tel: row.get::<_, String>("tel")?,
                email: row.get::<_, String>("email")?,
                time_created: row.get::<_, i64>("time_created")?,
                time_last_used: row.get::<_, i64>("time_last_used")?,
                time_last_modified: row.get::<_, i64>("time_last_modified")?,
                times_used: row.get::<_, i64>("times_used")?,
            })
        }
    )?;

    Ok(Some(address))
}

fn get_all_addresses() -> Result<Vec<Address>> {
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test::new_mem_db;

    #[test]
    fn test_address_api() {
        let mut db = new_mem_db();
        let tx = db.transaction().expect("should get transaction");

        let address = Address {
            given_name: "jane".to_string(),
            additional_name: "".to_string(),
            family_name: "doe".to_string(),
            organization: "".to_string(),
            street_address: "123 Main Street".to_string(),
            address_level3: "".to_string(),
            address_level2: "Seattle, WA".to_string(),
            address_level1: "".to_string(),
            postal_code: "".to_string(),
            country: "United States".to_string(),
            tel: "".to_string(),
            email: "".to_string(),

            ..Address::default()
        };

        assert_eq!(Guid::default(), address.guid);
        let saved_address = add_address(&tx, address).expect("should contain saved address");
        assert_ne!(Guid::default(), saved_address.guid);
        let retrieved_address = get_address(&tx, &saved_address.guid)
            .expect("should contain optional retrieved address");
        assert!(retrieved_address.is_some());
        assert_eq!(saved_address.guid, retrieved_address.unwrap().guid);
    }
}
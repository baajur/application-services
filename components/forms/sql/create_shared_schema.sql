CREATE TABLE IF NOT EXISTS addresses_data (
    address_id          TEXT NOT NULL PRIMARY KEY,
    given_name          TEXT NOT NULL,
    additional_name     TEXT NOT NULL,
    family_name         TEXT NOT NULL,
    organization        TEXT NOT NULL,       -- Company
    street_address      TEXT NOT NULL,       -- (Multiline)
    address_level3      TEXT NOT NULL,       -- Suburb/Sublocality
    address_level2      TEXT NOT NULL,       -- City/Town
    address_level1      TEXT NOT NULL,       -- Province (Standardized code if possible)
    postal_code         TEXT,
    country             TEXT,                -- ISO 3166
    tel                 TEXT,                -- Stored in E.164 format
    email               TEXT,

    -- removed computed fields, need to figure out

    time_created        INTEGER NOT NULL,
    time_last_used      INTEGER,
    time_last_modified  INTEGER NOT NULL,
    is_deleted          TINYINT NOT NULL DEFAULT 0,

    sync_change_counter INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS addresses_mirror (
    guid                TEXT NOT NULL PRIMARY KEY,

    address_id          TEXT NOT NULL PRIMARY KEY,
    given_name          TEXT NOT NULL,
    additional_name     TEXT NOT NULL,
    family_name         TEXT NOT NULL,
    organization        TEXT NOT NULL,       -- Company
    street_address      TEXT NOT NULL,       -- (Multiline)
    address_level3      TEXT NOT NULL,       -- Suburb/Sublocality
    address_level2      TEXT NOT NULL,       -- City/Town
    address_level1      TEXT NOT NULL,       -- Province (Standardized code if possible)
    postal_code         TEXT,
    country             TEXT,                -- ISO 3166
    tel                 TEXT,                -- Stored in E.164 format
    email               TEXT,
    time_created        INTEGER NOT NULL,
    time_last_used      INTEGER,
    time_last_modified  INTEGER NOT NULL,
    is_deleted          TINYINT NOT NULL DEFAULT 0,

    sync_change_counter INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS credit_cards_data (
    form_id             TEXT NOT NULL PRIMARY KEY,
    cc_name             TEXT NOT NULL, -- full name
    cc_given_name       TEXT NOT NULL,
    cc_additonal_name   TEXT NOT NULL,
    cc_family_name      TEXT NOT NULL,
    cc_number           TEXT NOT NULL,
    cc_exp_month        INTEGER NOT NULL,
    cc_exp_year         INTEGER NOT NULL,
    cc_type             TEXT NOT NULL,
    cc_exp              TEXT NOT NULL, -- text format of the expiration date e.g. "[cc_exp_year]-[cc_exp_month]"
    time_created        INTEGER NOT NULL,
    time_last_used      INTEGER,
    time_last_modified  INTEGER NOT NULL,
    last_synced_fields  TEXT,
    is_deleted          TINYINT NOT NULL DEFAULT 0,

    /* Same "sync change counter" strategy used by other components. */
    sync_change_counter INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS credit_cards_mirror (
    guid                TEXT NOT NULL PRIMARY KEY,

    form_id             TEXT NOT NULL PRIMARY KEY,
    cc_name             TEXT NOT NULL,
    cc_given_name       TEXT NOT NULL,
    cc_additonal_name   TEXT NOT NULL,
    cc_family_name      TEXT NOT NULL,
    cc_number           TEXT NOT NULL,
    cc_exp_month        INTEGER NOT NULL,
    cc_exp_year         INTEGER NOT NULL,
    cc_type             TEXT NOT NULL,
    cc_exp              TEXT NOT NULL,
    time_created        INTEGER NOT NULL,
    time_last_used      INTEGER,
    time_last_modified  INTEGER NOT NULL,
    last_synced_fields  TEXT,
    is_deleted          TINYINT NOT NULL DEFAULT 0,
);
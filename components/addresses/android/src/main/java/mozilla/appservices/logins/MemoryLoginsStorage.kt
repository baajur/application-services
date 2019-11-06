/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.appservices.addresses

import android.util.Log
import java.util.UUID
import mozilla.appservices.sync15.SyncTelemetryPing

private enum class AddressesStorageState {
    Unlocked,
    Locked,
    Closed,
}

class MemoryAddressesStorage(private var list: List<ServerPassword>) : AutoCloseable, AddressesStorage {

    private var state: AddressesStorageState = AddressesStorageState.Locked

    init {
        // Check that the list we were given as input doesn't have any duplicated IDs.
        val ids = HashSet<String>(list.map { it.id })
        if (ids.size != list.size) {
            throw AddressesStorageException("MemoryAddressesStorage was provided with addresses list that had duplicated IDs")
        }
    }

    override fun getHandle(): Long {
        throw UnsupportedOperationException("Only DatabaseAddressesStorage supports getHandle")
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun close() {
        state = AddressesStorageState.Closed
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun lock() {
        checkNotClosed()
        if (state == AddressesStorageState.Locked) {
            throw MismatchedLockException("Lock called when we are already locked")
        }
        state = AddressesStorageState.Locked
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun unlock(encryptionKey: String) {
        checkNotClosed()
        if (state == AddressesStorageState.Unlocked) {
            throw MismatchedLockException("Unlock called when we are already unlocked")
        }
        state = AddressesStorageState.Unlocked
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun unlock(encryptionKey: ByteArray) {
        // Currently we never check the key for the in-memory version, so this is fine.
        unlock("")
    }

    @Synchronized
    override fun isLocked(): Boolean {
        return state == AddressesStorageState.Locked
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun ensureUnlocked(encryptionKey: String) {
        if (isLocked()) {
            this.unlock(encryptionKey)
        }
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun ensureUnlocked(encryptionKey: ByteArray) {
        if (isLocked()) {
            this.unlock(encryptionKey)
        }
    }

    @Synchronized
    override fun ensureLocked() {
        if (!isLocked()) {
            this.lock()
        }
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun sync(syncInfo: SyncUnlockInfo): SyncTelemetryPing {
        checkUnlocked()
        Log.w("MemoryAddressesStorage", "Not syncing because this implementation can not sync")
        return SyncTelemetryPing(version = 1, uid = "uid", events = emptyList(), syncs = emptyList())
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun reset() {
        checkUnlocked()
        Log.w("MemoryAddressesStorage", "Reset is a noop becasue this implementation can not sync")
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun wipe() {
        checkUnlocked()
        list = ArrayList()
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun wipeLocal() {
        checkUnlocked()
        // No remote state.
        list = ArrayList()
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun delete(id: String): Boolean {
        checkUnlocked()
        val oldLen = list.size
        list = list.filter { it.id != id }
        return oldLen != list.size
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun get(id: String): ServerPassword? {
        checkUnlocked()
        return list.find { it.id == id }
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun touch(id: String) {
        checkUnlocked()
        val sp = list.find { it.id == id }
                ?: throw NoSuchRecordException("No such record: $id")
        // ServerPasswords are immutable, so we remove the current one from the list and
        // add a new one with updated properties
        list = list.filter { it.id != id }

        val newsp = sp.copy(
            timeLastUsed = System.currentTimeMillis(),
            timesUsed = sp.timesUsed + 1
        )
        list += newsp
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun add(address: ServerPassword): String {
        checkUnlocked()
        val toInsert = if (address.id.isEmpty()) {
            // This isn't anything like what the IDs we generate in rust look like
            // but whatever.
            address.copy(id = UUID.randomUUID().toString())
        } else {
            address
        }.copy(
            timesUsed = 1,
            timeLastUsed = System.currentTimeMillis(),
            timeCreated = System.currentTimeMillis(),
            timePasswordChanged = System.currentTimeMillis()
        )

        checkValid(toInsert)

        val sp = list.find { it.id == toInsert.id }
        if (sp != null) {
            // Note: Not the way this is formatted in rust -- don't rely on the formatting!
            throw IdCollisionException("Id already exists " + toInsert.id)
        }

        list += toInsert
        return toInsert.id
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun importAddresses(addresses: Array<ServerPassword>): Long {
        checkUnlocked()
        var numErrors = 0L
        for (address in addresses) {
            val toInsert = address.copy(id = UUID.randomUUID().toString())
            try {
                checkValid(toInsert)
                list += toInsert
            } catch (e: InvalidRecordException) {
                numErrors += 1
            }
        }
        return numErrors
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun update(address: ServerPassword) {
        checkUnlocked()
        val current = list.find { it.id == address.id }
                ?: throw NoSuchRecordException("No such record: " + address.id)

        val newRecord = address.copy(
                timeLastUsed = System.currentTimeMillis(),
                timesUsed = current.timesUsed + 1,
                timeCreated = current.timeCreated,
                timePasswordChanged = if (current.password == address.password) {
                    current.timePasswordChanged
                } else {
                    System.currentTimeMillis()
                })

        checkValid(newRecord)

        list = list.filter { it.id != address.id }

        list += newRecord
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun list(): List<ServerPassword> {
        checkUnlocked()
        // Return a copy so that mutations aren't visible (AIUI using `val` consistently in
        // ServerPassword means it's immutable, so it's fine that this is a shallow copy)
        return ArrayList(list)
    }

    @Synchronized
    @Throws(AddressesStorageException::class)
    override fun getByHostname(hostname: String): List<ServerPassword> {
        checkUnlocked()
        list = list.filter { it.hostname == hostname }
        return ArrayList(list)
    }

    private fun checkNotClosed() {
        if (state == AddressesStorageState.Closed) {
            throw AddressesStorageException("Using MemoryAddressesStorage after close!")
        }
    }

    private fun checkUnlocked() {
        if (state != AddressesStorageState.Unlocked) {
            throw AddressesStorageException("Using MemoryAddressesStorage without unlocking first: $state")
        }
    }

    @Suppress("ThrowsCount")
    private fun checkValid(address: ServerPassword) {
        if (address.hostname == "") {
            throw InvalidRecordException("Invalid address: Hostname is empty")
        }
        if (address.password == "") {
            throw InvalidRecordException("Invalid address: Password is empty")
        }
        if (address.formSubmitURL != null && address.httpRealm != null) {
            throw InvalidRecordException(
                    "Invalid address: Both `formSubmitUrl` and `httpRealm` are present")
        }
        if (address.formSubmitURL == null && address.httpRealm == null) {
            throw InvalidRecordException(
                    "Invalid address: Neither `formSubmitUrl` and `httpRealm` are present")
        }
    }
}
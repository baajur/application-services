// This file was autogenerated by some hot garbage in the `uniffi` crate.
// Trust me, you don't want to mess with it!

#pragma once

#include <stdbool.h>
#include <stdint.h>

typedef struct RustBuffer {
    int64_t len;
    uint8_t *_Nullable data;
} RustBuffer;

// Error definitions
// Each error has an error code enum, and a struct
typedef struct NativeRustError {
    int32_t code;
    char *_Nullable message;
} NativeRustError;

  
void ffi_fxa_client_FirefoxAccount_object_free(
      uint64_t handle
    
    );
uint64_t fxa_client_FirefoxAccount_new(
      char*_Nonnull content_url,char*_Nonnull client_id,char*_Nonnull redirect_uri,RustBuffer token_server_url_override
    
    );
char*_Nonnull fxa_client_FirefoxAccount_to_json(
      uint64_t handle
    ,NativeRustError *_Nonnull out_err
    );
char*_Nonnull fxa_client_FirefoxAccount_get_pairing_authority_url(
      uint64_t handle
    ,NativeRustError *_Nonnull out_err
    );
char*_Nonnull fxa_client_FirefoxAccount_get_token_server_endpoint_url(
      uint64_t handle
    ,NativeRustError *_Nonnull out_err
    );
char*_Nonnull fxa_client_FirefoxAccount_get_connection_success_url(
      uint64_t handle
    ,NativeRustError *_Nonnull out_err
    );
char*_Nonnull fxa_client_FirefoxAccount_get_manage_account_url(
      uint64_t handle,char*_Nonnull entrypoint
    ,NativeRustError *_Nonnull out_err
    );
char*_Nonnull fxa_client_FirefoxAccount_get_manage_devices_url(
      uint64_t handle,char*_Nonnull entrypoint
    ,NativeRustError *_Nonnull out_err
    );
char*_Nonnull fxa_client_FirefoxAccount_begin_oauth_flow(
      uint64_t handle,RustBuffer scopes,char*_Nonnull entrypoint
    ,NativeRustError *_Nonnull out_err
    );
char*_Nonnull fxa_client_FirefoxAccount_begin_pairing_flow(
      uint64_t handle,char*_Nonnull pairing_url,RustBuffer scopes,char*_Nonnull entrypoint
    ,NativeRustError *_Nonnull out_err
    );
void fxa_client_FirefoxAccount_complete_oauth_flow(
      uint64_t handle,char*_Nonnull code,char*_Nonnull state
    ,NativeRustError *_Nonnull out_err
    );
void fxa_client_FirefoxAccount_disconnect(
      uint64_t handle
    
    );
RustBuffer fxa_client_FirefoxAccount_check_authorization_status(
      uint64_t handle
    ,NativeRustError *_Nonnull out_err
    );
RustBuffer fxa_client_FirefoxAccount_get_access_token(
      uint64_t handle,char*_Nonnull scope,RustBuffer ttl
    ,NativeRustError *_Nonnull out_err
    );
char*_Nonnull fxa_client_FirefoxAccount_get_session_token(
      uint64_t handle
    ,NativeRustError *_Nonnull out_err
    );
char*_Nonnull fxa_client_FirefoxAccount_get_current_device_id(
      uint64_t handle
    ,NativeRustError *_Nonnull out_err
    );
char*_Nonnull fxa_client_FirefoxAccount_authorize_code_using_session_token(
      uint64_t handle,RustBuffer params
    ,NativeRustError *_Nonnull out_err
    );
void fxa_client_FirefoxAccount_clear_access_token_cache(
      uint64_t handle
    
    );
RustBuffer fxa_client_FirefoxAccount_migrate_from_session_token(
      uint64_t handle,char*_Nonnull session_token,char*_Nonnull k_sync,char*_Nonnull k_xcs,uint8_t copy_session_token
    ,NativeRustError *_Nonnull out_err
    );
RustBuffer fxa_client_FirefoxAccount_retry_migrate_from_session_token(
      uint64_t handle
    ,NativeRustError *_Nonnull out_err
    );
uint32_t fxa_client_FirefoxAccount_is_in_migration_state(
      uint64_t handle
    
    );
RustBuffer fxa_client_FirefoxAccount_get_profile(
      uint64_t handle,uint8_t ignore_cache
    ,NativeRustError *_Nonnull out_err
    );
void fxa_client_FirefoxAccount_initialize_device(
      uint64_t handle,char*_Nonnull name,uint32_t device_type,RustBuffer supported_capabilities
    ,NativeRustError *_Nonnull out_err
    );
void fxa_client_FirefoxAccount_ensure_capabilities(
      uint64_t handle,RustBuffer supported_capabilities
    ,NativeRustError *_Nonnull out_err
    );
void fxa_client_FirefoxAccount_set_push_subscription(
      uint64_t handle,RustBuffer subscription
    ,NativeRustError *_Nonnull out_err
    );
void fxa_client_FirefoxAccount_set_device_name(
      uint64_t handle,char*_Nonnull display_name
    ,NativeRustError *_Nonnull out_err
    );
void fxa_client_FirefoxAccount_send_tab(
      uint64_t handle,char*_Nonnull target_device_id,char*_Nonnull title,char*_Nonnull url
    ,NativeRustError *_Nonnull out_err
    );
RustBuffer ffi_fxa_client_bytebuffer_alloc(
      uint32_t size
    
    );
void ffi_fxa_client_bytebuffer_free(
      RustBuffer buf
    
    );
char*_Nonnull ffi_fxa_client_string_alloc_from(
      const char*_Nonnull str
    ,NativeRustError *_Nonnull out_err
    );
void ffi_fxa_client_string_free(
      char*_Nonnull str
    
    );

use alloc::format;
use alloc::vec::Vec;
use core::slice;

use app_xrp::errors::XRPError;
use cty::c_char;

use ur_registry::bytes::Bytes;
use ur_registry::traits::RegistryItem;

use crate::common::errors::ErrorCodes;
use crate::common::structs::{SimpleResponse, TransactionCheckResult, TransactionParseResult};
use crate::common::types::{PtrBytes, PtrString, PtrT, PtrUR};
use crate::common::ur::{UREncodeResult, FRAGMENT_MAX_LENGTH_DEFAULT};
use crate::common::utils::{convert_c_char, recover_c_char};
use crate::extract_array;
use crate::extract_ptr_with_type;

use structs::DisplayXrpTx;

pub mod structs;

#[no_mangle]
pub unsafe extern "C" fn xrp_get_address(
    hd_path: PtrString,
    root_x_pub: PtrString,
    root_path: PtrString,
) -> *mut SimpleResponse<c_char> {
    let hd_path = recover_c_char(hd_path);
    let root_x_pub = recover_c_char(root_x_pub);
    let root_path = recover_c_char(root_path);
    if !hd_path.starts_with(root_path.as_str()) {
        return SimpleResponse::from(XRPError::InvalidHDPath(format!(
            "{hd_path} does not match {root_path}"
        )))
        .simple_c_ptr();
    }
    let address = app_xrp::address::get_address(&hd_path, &root_x_pub, &root_path);
    match address {
        Ok(result) => SimpleResponse::success(convert_c_char(result) as *mut c_char).simple_c_ptr(),
        Err(e) => SimpleResponse::from(e).simple_c_ptr(),
    }
}

unsafe fn build_sign_result(
    ptr: PtrUR,
    hd_path: PtrString,
    seed: &[u8],
) -> Result<Vec<u8>, XRPError> {
    let crypto_bytes = extract_ptr_with_type!(ptr, Bytes);
    let hd_path = recover_c_char(hd_path);
    app_xrp::sign_tx(crypto_bytes.get_bytes().as_slice(), &hd_path, seed)
}

#[no_mangle]
pub unsafe extern "C" fn xrp_parse_tx(ptr: PtrUR) -> PtrT<TransactionParseResult<DisplayXrpTx>> {
    let crypto_bytes = extract_ptr_with_type!(ptr, Bytes);
    match app_xrp::parse(crypto_bytes.get_bytes().as_slice()) {
        Ok(v) => TransactionParseResult::success(DisplayXrpTx::from(v).c_ptr()).c_ptr(),
        Err(e) => TransactionParseResult::from(e).c_ptr(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn xrp_sign_tx(
    ptr: PtrUR,
    hd_path: PtrString,
    seed: PtrBytes,
    seed_len: u32,
) -> PtrT<UREncodeResult> {
    let seed = extract_array!(seed, u8, seed_len);
    let result = build_sign_result(ptr, hd_path, seed);
    match result.map(|v| Bytes::new(v).try_into()) {
        Ok(v) => match v {
            Ok(data) => UREncodeResult::encode(
                data,
                Bytes::get_registry_type().get_type(),
                FRAGMENT_MAX_LENGTH_DEFAULT,
            )
            .c_ptr(),
            Err(e) => UREncodeResult::from(e).c_ptr(),
        },
        Err(e) => UREncodeResult::from(e).c_ptr(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn xrp_check_tx(
    ptr: PtrUR,
    root_xpub: PtrString,
    cached_pubkey: PtrString,
) -> PtrT<TransactionCheckResult> {
    let crypto_bytes = extract_ptr_with_type!(ptr, Bytes);
    let root_xpub = recover_c_char(root_xpub);
    let cached_pubkey = recover_c_char(cached_pubkey);
    match app_xrp::check_tx(
        crypto_bytes.get_bytes().as_slice(),
        &root_xpub,
        &cached_pubkey,
    ) {
        Ok(p) => TransactionCheckResult::error(ErrorCodes::Success, p).c_ptr(),
        Err(e) => TransactionCheckResult::from(e).c_ptr(),
    }
}

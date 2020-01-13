#![no_std]

//#[macro_use]
extern crate alloc;
// And now you can use `alloc` types!
pub use alloc::boxed::Box;
pub use alloc::vec::Vec;

// mod ext;
// mod ext_int64;
//mod big_int;
mod address;

pub use address::*;

// Note: contracts and the api are not mutable.
// They simply pass on/retrieve data to/from the protocol.
// When mocking the blockchain state, we use the Rc/RefCell pattern 
// to isolate mock state mutability from the contract interface.
pub trait ContractHookApi<BI> {

    fn get_owner(&self) -> Address;

    fn get_caller(&self) -> Address;

    fn signal_error(&self);

    fn write_log(&self, topics: &[[u8;32]], data: &[u8]);

    fn storage_store_big_int(&self, key: &StorageKey, value: &BI);
    
    fn storage_load_big_int(&self, key: &StorageKey) -> BI;
    
    fn get_call_value_big_int(&self) -> BI;
}

pub trait ContractIOApi<BI, BU> {

    fn check_num_arguments(&self, expected: i32) -> bool;

    fn check_not_payable(&self) -> bool;

    fn get_argument_bytes32(&self, arg_index: i32) -> [u8; 32];
    
    fn get_argument_address(&self, arg_index: i32) -> Address;
    
    fn get_argument_big_int_signed(&self, arg_id: i32) -> BI;

    fn get_argument_big_int_unsigned(&self, arg_id: i32) -> BU;
    
    fn get_argument_i64(&self, arg_id: i32) -> i64;
    
    fn finish_big_int_signed(&self, b: BI);

    fn finish_big_int_unsigned(&self, b: BU);

    fn finish_i64(&self, value: i64);
}

use core::ops::{Add, Sub, Mul};
use core::ops::{AddAssign, SubAssign, MulAssign};

pub trait BigIntApi: 
        Sized + 
        From<i64> +
        From<i32> +
        Clone +
        Add + 
        AddAssign + 
        Sub + 
        SubAssign +
        Mul +
        MulAssign +
        PartialEq +
        Eq +
        PartialOrd +
        Ord +
{
    fn byte_length(&self) -> i32;

    fn copy_to_slice(&self, slice: &mut [u8]) -> i32;

    fn get_bytes_big_endian(&self) -> Vec<u8>;

    fn get_bytes_big_endian_pad_right(&self, nr_bytes: usize) -> Vec<u8>;

    // only needed at compilation, value will never be used
    fn phantom() -> Self;
}

// we just use it to signal the api to interpret inputs as unsigned
// so minimal logic, just convert to/from signed
pub trait BigUintApi<BI>: 
    Sized +
    From<BI>
{
    // convert to the signed big int, consuming self
    fn into_signed(self) -> BI;

    // only needed at compilation, value will never be used
    fn phantom() -> Self;
}

pub trait CallableContract {
    fn call(&self, fn_name: &'static str);

    fn clone_contract(&self) -> Box<dyn CallableContract>;
}

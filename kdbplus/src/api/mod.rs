//! This `api` module provides API mirroring the C API of q/kdb+. The expected usage is to build a shared library for q/kdb+ in Rust.
//!
//! In order to avoid writing too large `unsafe` block leading to poor optimization, most of native C API functions were provided
//!  with a wrapper funtion with a bit of ergonomic safety and with intuitive implementation as a trait method. The only exceptions
//!  are:
//! - `knk`
//! - `k`
//! These functions are using elipsis (`...`) as its argument and cannot be provided with a stable distribution. When you need to use
//!  either of them you can find them under `native` namespace together with the other naked C API functions.
//!
//! *Notes:*
//!
//! - This library is for kdb+ version >= 3.0.
//! - Meangless C macros are excluded but accessors of an underlying array like `kC`, `kJ`, `kK` etc. are provided in Rust way.
//!
//! ## Examples
//!
//! In order to encourage to use Rust style API, examples of the C API style is not provided here (you can see them in [README of the repository](https://github.com/diamondrod/kdbplus)).
//!  The examples below are written without `unsafe` code. You can see how comfortably breathing are the wrapped functions in the code.
//!
//! ```no_run
//! use kdbplus::str_to_S;
//! use kdbplus::api::*;
//! use kdbplus::qtype;
//!
//! #[no_mangle]
//! pub extern "C" fn create_symbol_list2(_: K) -> K{
//!   let mut list=new_list(qtype::SYMBOL_LIST, 0);
//!   list.push_symbol("Abraham").unwrap();
//!   list.push_symbol("Isaac").unwrap();
//!   list.push_symbol("Jacob").unwrap();
//!   list.push_symbol_n("Josephine", 6).unwrap();
//!   list
//! }
//!
//! #[no_mangle]
//! fn no_panick(func: K, args: K) -> K{
//!   let result=unsafe{error_to_string(apply(func, args))};
//!   if let Ok(error) = result.get_error_string(){
//!     println!("FYI: {}", error);
//!     // Decrement reference count of the error object which is no longer used.
//!     unsafe{decrement_reference_count(result)};
//!     KNULL
//!   }
//!   else{
//!     println!("success!");
//!     result
//!   }
//! }
//!
//! #[no_mangle]
//! pub extern "C" fn create_table2(_: K) -> K{
//!   // Build keys
//!   let keys=new_list(qtype::SYMBOL_LIST, 2);
//!   let keys_slice=unsafe{keys.as_mut_slice::<S>()};
//!   keys_slice[0]=unsafe{enumerate(str_to_S!("time"))};
//!   keys_slice[1]=unsafe{enumerate_n(str_to_S!("temperature_and_humidity"), 11)};
//!
//!   // Build values
//!   let values=new_list(qtype::COMPOUND_LIST, 2);
//!   let time=new_list(qtype::TIMESTAMP_LIST, 3);
//!   // 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392
//!   unsafe{time.as_mut_slice::<J>()}.copy_from_slice(&[119067859167018272_i64, 201766609419710368, 271897944018691392]);
//!   let temperature=new_list(qtype::FLOAT_LIST, 3);
//!   unsafe{temperature.as_mut_slice::<F>()}.copy_from_slice(&[22.1_f64, 24.7, 30.5]);
//!   unsafe{values.as_mut_slice::<K>()}.copy_from_slice(&[time, temperature]);
//!   
//!   unsafe{flip(new_dictionary(keys, values))}
//! }
//! ```
//!
//! And q code is here:
//!
//! ```q
//! q)summon:`libapi_examples 2: (`create_symbol_list2; 1)
//! q)summon[]
//! `Abraham`Isaac`Jacob`Joseph
//! q)chill: `libapi_examples 2: (`no_panick; 2);
//! q)chill[$; ("J"; "42")]
//! success!
//! 42
//! q)chill[+; (1; `a)]
//! FYI: type
//! q)climate_change: libc_api_examples 2: (`create_table2; 1);
//! q)climate_change[]
//! time                          temperature
//! -----------------------------------------
//! 2003.10.10D02:24:19.167018272 22.1       
//! 2006.05.24D06:16:49.419710368 24.7       
//! 2008.08.12D23:12:24.018691392 30.5  
//! ```

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Settings
//++++++++++++++++++++++++++++++++++++++++++++++++++//

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Load Libraries
//++++++++++++++++++++++++++++++++++++++++++++++++++//

use crate::qtype;
use libc::{c_char, c_double, c_float, c_int, c_longlong, c_schar, c_short, c_uchar, c_void};
use std::convert::TryInto;
use std::ffi::CStr;
use std::str;
pub mod native;
mod re_exports;
pub use re_exports::*;

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Global Variables
//++++++++++++++++++++++++++++++++++++++++++++++++++//

/// `K` nullptr. This value can be used as void value of a function which is called directly by q process
///  and returns `K`. This null pointer is interpreted as a general null value (`::`) whose type is `101h`.
/// # Example
/// ```
/// use kdbplus::api::*;
///
/// #[no_mangle]
/// pub extern "C" fn vanity(_: K) -> K{
///   println!("Initialized something, probably it is your mindset.");
///   KNULL
/// }
/// ```
/// # Warning
/// This value must NOT be used as a returned value for functions called by another function
///  because [`error_to_string`](fn.error_to_string.html) misunderstands the value as an error.
///  For detail, see its warning section.
pub const KNULL: K = 0 as K;

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Macros
//++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Utility %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Convert `&str` to `S` (null-terminated character array).
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// use kdbplus::str_to_S;
///
/// #[no_mangle]
/// pub extern "C" fn pingpong(_: K) -> K{
///   unsafe{native::k(0, str_to_S!("ping"), new_int(77), KNULL)}
/// }
/// ```
/// ```q
/// q)ping:{[int] `$string[int], "_pong!!"}
/// q)pingpong: `libapi_examples 2: (`pingpong; 1);
/// q)pingpong[]
/// `77_pong!!
/// ```
/// # Note
/// This macro cannot be created as a function due to freeing resource of Rust (not sure).
#[macro_export]
macro_rules! str_to_S {
    ($string: expr) => {
        [$string.as_bytes(), &[b'\0']].concat().as_mut_ptr() as S
    };
}

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Structs
//++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Alias %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// `char*` in C. Also used to access symbol of q.
pub type S = *mut c_char;
/// `const char*` in C.
pub type const_S = *const c_char;
/// `char` in C. Also used to access char of q.
pub type C = c_char;
/// `unsigned char` in C. Also used to access byte of q.
pub type G = c_uchar;
/// `i16` in C. Also used to access short of q.
pub type H = c_short;
/// `i32` in C. Also used to access int and compatible types (month, date, minute, second and time) of q.
pub type I = c_int;
/// `i64` in C. Also used to access long and compatible types (timestamp and timespan) of q.
pub type J = c_longlong;
/// `f32` in C. Also used to access real of q.
pub type E = c_float;
/// `f64` in C. Also used to access float and datetime of q.
pub type F = c_double;
/// `void` in C.
pub type V = c_void;

//%% U %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Struct representing 16-bytes GUID.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct U {
    pub guid: [G; 16],
}

//%% K %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Underlying list value of q object.
/// # Note
/// Usually this struct does not need to be accessed this struct directly unless user wants to
///  access via a raw pointer for non-trivial stuff.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct k0_list {
    /// Length of the list.
    pub n: J,
    /// Pointer referring to the head of the list. This pointer will be interpreted
    ///  as various types when accessing `K` object to edit the list with
    ///  [`as_mut_slice`](trait.KUtility.html#tymethod.as_mut_slice).
    pub G0: [G; 1],
}

/// Underlying atom value of q object.
/// # Note
/// Usually this struct does not need to be accessed directly unless user wants to
///  access via a raw pointer for non-trivial stuff.
#[derive(Clone, Copy)]
#[repr(C)]
pub union k0_inner {
    /// Byte type holder.
    pub byte: G,
    /// Short type holder.
    pub short: H,
    /// Int type holder.
    pub int: I,
    /// Long type older.
    pub long: J,
    /// Real type holder.
    pub real: E,
    /// Float type holder.
    pub float: F,
    /// Symbol type holder.
    pub symbol: S,
    /// Table type holder.
    pub table: *mut k0,
    /// List type holder.
    pub list: k0_list,
}

/// Underlying struct of `K` object.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct k0 {
    /// For internal usage.
    m: c_schar,
    /// For internal usage.
    a: c_schar,
    /// Type indicator.
    pub qtype: c_schar,
    /// Attribute of list.
    pub attribute: C,
    /// Reference count of the object.
    pub refcount: I,
    /// Underlying value.
    pub value: k0_inner,
}

/// Struct representing q object.
pub type K = *mut k0;

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Structs
//++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% KUtility %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Trait which defines utility methods to manipulate q object.
#[allow(clippy::len_without_is_empty)]
pub trait KUtility {
    /// Derefer `K` as a mutable slice of the specified type. The supported types are:
    /// - `G`: Equivalent to C API macro `kG`.
    /// - `H`: Equivalent to C API macro `kH`.
    /// - `I`: Equivalent to C API macro `kI`.
    /// - `J`: Equivalent to C API macro `kJ`.
    /// - `E`: Equivalent to C API macro `kE`.
    /// - `F`: Equivalent to C API macro `kF`.
    /// - `C`: Equivalent to C API macro `kC`.
    /// - `S`: Equivalent to C API macro `kS`.
    /// - `K`: Equivalent to C API macro `kK`.
    /// # Example
    /// ```
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn modify_long_list_a_bit(long_list: K) -> K{
    ///   if long_list.len() >= 2{
    ///     // Derefer as a mutable i64 slice.
    ///     unsafe{long_list.as_mut_slice::<J>()[1]=30000_i64};
    ///     // Increment the counter to reuse on q side.
    ///     increment_reference_count(long_list)
    ///   }
    ///   else{
    ///     new_error("this list is not long enough. how ironic...\0")
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)ironic: `libapi_examples 2: (`modify_long_list_a_bit; 1);
    /// q)list:1 2 3;
    /// q)ironic list
    /// 1 30000 3
    /// q)ironic enlist 1
    /// ```
    /// # Note
    /// Intuitively the parameter should be `&mut self` but it restricts a manipulating
    ///  `K` objects in the form of slice simultaneously. As copying a pointer is not
    ///  an expensive operation, using `self` should be fine.
    ///
    /// # Safety
    /// self must be a valid, non-null, pointer to a `K` object.
    #[allow(clippy::wrong_self_convention)]
    unsafe fn as_mut_slice<'a, T>(self) -> &'a mut [T];

    /// Get an underlying q byte.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn print_bool(atom: K) -> K{
    ///   match atom.get_bool(){
    ///     Ok(boolean) => {
    ///       println!("bool: {}", boolean);
    ///       KNULL
    ///     },
    ///     Err(error) => new_error(error)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)print_boole: `libapi_examples 2: (`print_bool; 1);
    /// q)print_bool[1b]
    /// bool: true
    /// ```
    fn get_bool(&self) -> Result<bool, &'static str>;

    /// Get an underlying q byte.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn print_guid(atom: K) -> K{
    ///   match atom.get_guid(){
    ///     Ok(guid) => {
    ///       let strguid=guid.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    ///       println!("GUID: {}-{}-{}-{}-{}", &strguid[0..4], &strguid[4..6], &strguid[6..8], &strguid[8..10], &strguid[10..16]);
    ///       KNULL
    ///     },
    ///     Err(error) => new_error(error)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)print_guid: `libapi_examples 2: (`print_guid; 1);
    /// q)guid: first 1?0Ng;
    /// q)print_guid[guid]
    /// GUID: 8c6b-8b-64-68-156084
    /// ```
    fn get_guid(&self) -> Result<[u8; 16], &'static str>;

    /// Get an underlying q byte.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn print_byte(atom: K) -> K{
    ///   match atom.get_byte(){
    ///     Ok(byte) => {
    ///       println!("byte: {:#4x}", byte);
    ///       KNULL
    ///     },
    ///     Err(error) => new_error(error)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)print_byte: `libapi_examples 2: (`print_byte; 1);
    /// q)print_byte[0xc4]
    /// byte: 0xc4
    /// ```
    fn get_byte(&self) -> Result<u8, &'static str>;

    /// Get an underlying q short.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn print_short(atom: K) -> K{
    ///   match atom.get_short(){
    ///     Ok(short) => {
    ///       println!("short: {}", short);
    ///       KNULL
    ///     },
    ///     Err(error) => new_error(error)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)print_short: `libapi_examples 2: (`print_short; 1);
    /// q)print_short[10h]
    /// short: 10
    /// ```
    fn get_short(&self) -> Result<i16, &'static str>;

    /// Get an underlying q int.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn print_int(atom: K) -> K{
    ///   match atom.get_int(){
    ///     Ok(int) => {
    ///       println!("int: {}", int);
    ///       KNULL
    ///     },
    ///     Err(error) => new_error(error)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)print_int: `libapi_examples 2: (`print_int; 1);
    /// q)print_int[03:57:20]
    /// int: 14240
    /// ```
    fn get_int(&self) -> Result<i32, &'static str>;

    /// Get an underlying q long.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn print_long(atom: K) -> K{
    ///   match atom.get_long(){
    ///     Ok(long) => {
    ///       println!("long: {}", long);
    ///       KNULL
    ///     },
    ///     Err(error) => new_error(error)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)print_long: `libapi_examples 2: (`print_long; 1);
    /// q)print_long[2000.01.01D12:00:00.123456789]
    /// long: 43200123456789
    /// ```
    fn get_long(&self) -> Result<i64, &'static str>;

    /// Get an underlying q real.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn print_real(atom: K) -> K{
    ///   match atom.get_real(){
    ///     Ok(real) => {
    ///       println!("real: {}", real);
    ///       KNULL
    ///     },
    ///     Err(error) => new_error(error)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)print_real: `libapi_examples 2: (`print_real; 1);
    /// q)print_real[193810.32e]
    /// real: 193810.31
    /// ```
    fn get_real(&self) -> Result<f32, &'static str>;

    /// Get an underlying q float.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn print_float(atom: K) -> K{
    ///   match atom.get_float(){
    ///     Ok(float) => {
    ///       println!("float: {:.8}", float);
    ///       KNULL
    ///     },
    ///     Err(error) => new_error(error)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)print_float: `libapi_examples 2: (`print_float; 1);
    /// q)print_float[2002.01.12T10:03:45.332]
    /// float: 742.41927468
    /// ```
    fn get_float(&self) -> Result<f64, &'static str>;

    /// Get an underlying q char.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn print_char(atom: K) -> K{
    ///   match atom.get_char(){
    ///     Ok(character) => {
    ///       println!("char: \"{}\"", character);
    ///       KNULL
    ///     },
    ///     Err(error) => new_error(error)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)print_char: `libapi_examples 2: (`print_char; 1);
    /// q)print_char["k"]
    /// char: "k"
    /// ```
    fn get_char(&self) -> Result<char, &'static str>;

    /// Get an underlying q symbol.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn print_symbol2(atom: K) -> K{
    ///   match atom.get_symbol(){
    ///     Ok(symbol) => {
    ///       println!("symbol: `{}", symbol);
    ///       KNULL
    ///     },
    ///     Err(error) => new_error(error)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)print_symbol2: `libapi_examples 2: (`print_symbol2; 1);
    /// q)print_symbol2[`locust]
    /// symbol: `locust
    /// ```
    fn get_symbol(&self) -> Result<&str, &'static str>;

    /// Get an underlying q string as `&str`.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn print_string(string: K) -> K{
    ///   match string.get_str(){
    ///     Ok(string_) => {
    ///       println!("string: \"{}\"", string_);
    ///       KNULL
    ///     },
    ///     Err(error) => new_error(error)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)print_string: `libapi_examples 2: (`print_string; 1);
    /// q)print_string["gnat"]
    /// string: "gnat"
    /// ```
    fn get_str(&self) -> Result<&str, &'static str>;

    /// Get an underlying q string as `String`.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn print_string2(string: K) -> K{
    ///   match string.get_string(){
    ///     Ok(string_) => {
    ///       println!("string: \"{}\"", string_);
    ///       KNULL
    ///     },
    ///     Err(error) => new_error(error)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)print_string: `libapi_examples 2: (`print_string; 1);
    /// q)print_string["grasshopper"]
    /// string: "grasshopper"
    /// ```
    fn get_string(&self) -> Result<String, &'static str>;

    /// Get a flipped underlying q table as `K` (dictionary).
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn hidden_key(table: K) -> K{
    ///   match table.get_dictionary(){
    ///     Ok(dictionary) => unsafe{dictionary.as_mut_slice::<K>()[0]}.q_ipc_encode(3).unwrap(),
    ///     Err(error) => new_error(error)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)perceive_the_man: `libapi_examples 2: (`hidden_key; 1);
    /// q)perceive_the_man ([] t: `timestamp$.z.p+1e9*til 9; chr:"ljppkgfgs"; is: 7 8 12 14 21 316 400 1000 6000i)
    /// 0x01000000170000000b0003000000740063687200697300
    /// ```
    /// # Note
    /// This method is provided because the ony way to examine the value of table type is to access the underlying dictionary (flipped table).
    ///  Also when some serialization is necessary for a table, you can reuse a serializer for a dictionary if it is already provided. Actually
    ///  when q serialize a table object with `-8!` (q function) or `b9` (C code), it just serializes the underlying dictionary with an additional
    ///  marker indicating a table type.
    fn get_dictionary(&self) -> Result<K, &'static str>;

    /// Get an underlying error symbol as `&str`. This function avoids false positive of processing `KNULL` as an error.
    /// # Example
    /// See the example of [`error_to_string`](fn.error_to_string.html).
    fn get_error_string(&self) -> Result<&str, &'static str>;

    /// Get a table row of the given index. For enumerated column, a names of a target `sym` list
    ///  to which symbol values are cast must be passed. In the example below, it is assumed that
    ///  there is a single enum column in a table and the column values are cast to a symbol list whose name is `sym`.
    /// ```no_run
    /// use kdbplus::api::*;
    /// use kdbplus::qtype;
    /// use kdbplus::str_to_S;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn print_row(object: K, index: K) -> K{
    ///   match object.get_type(){
    ///     qtype::TABLE => {
    ///       match object.get_row(index.get_long().unwrap() as usize, &["sym"]){
    ///         Ok(row) => {
    ///           let null = unsafe{native::k(0, str_to_S!("{-1 \"row: \", .Q.s1 x}"), row, KNULL)};
    ///           unsafe{decrement_reference_count(null)};
    ///           KNULL
    ///         }
    ///         Err(error) => new_error(error)
    ///       }
    ///     },
    ///     _ => new_error("not a table\0")
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)row: `libapi_examples 2: (`print_row; 2)
    /// q)table: ([] time: asc `timestamp$.z.p + 3?1000000000; sym: -3?`Green`Yellow`Red; go: "oxx"; miscellaneous: ("cow"; `lion; "eagle"))
    /// q)row[table;2]
    /// row: `time`sym`go`miscellaneous!(2022.01.30D07:55:48.404520689;`Yellow;"x";"eagle")
    /// q)row[table;1]
    /// row: `time`sym`go`miscellaneous!(2022.01.30D07:55:47.987133353;`Green;"x";`lion)
    /// ```
    fn get_row(&self, index: usize, enum_sources: &[&str]) -> Result<K, &'static str>;

    /// Get an attribute of a q object.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    /// use kdbplus::qattribute;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn murmur(list: K) -> K{
    ///   match list.get_attribute(){
    ///     qattribute::SORTED => {
    ///       new_string("Clean")
    ///     },
    ///     qattribute::UNIQUE => {
    ///       new_symbol("Alone")
    ///     },
    ///     _ => KNULL
    ///   }
    /// }
    /// ```
    fn get_attribute(&self) -> C;

    /// Get a reference count of a q object.
    fn get_refcount(&self) -> I;

    /// Append a q list object to a q list.
    ///  Returns a pointer to the (potentially reallocated) `K` object.
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn concat_list2(mut list1: K, list2: K) -> K{
    ///   if let Err(err) = unsafe{list1.append(increment_reference_count(list2))} {
    ///     new_error(err)
    ///   }
    ///   else{
    ///     increment_reference_count(list1)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)plunder: `libapi_examples 2: (`concat_list2; 2);
    /// q)plunder[(::; `metals; `fire); ("clay"; 316)]
    /// ::
    /// `metals
    /// `fire
    /// "clay"
    /// 316
    /// q)plunder[1 2 3; 4 5]
    /// 1 2 3 4 5
    /// q)plunder[`a`b`c; `d`e]
    /// `a`b`c`d`e
    /// ```
    /// # Note
    /// While native function [`jv`](native/fn.jv.html) does not consume the appended list,
    ///  this function does for intuitiveness. To append externally provided list (i.e., passed
    ///  from q process), apply [`increment_reference_count`](fn.increment_reference_count.html)
    ///  before appending the list.
    ///
    /// # Safety
    /// you need to track the reference count of the appended list to avoid double free.
    unsafe fn append(&mut self, list: K) -> Result<K, &'static str>;

    /// Add a q object to a q compound list while the appended one is consumed.
    ///  Returns a pointer to the (potentially reallocated) `K` object.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    /// use kdbplus::qtype;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn create_compound_list(int: K) -> K{
    ///   let mut list=new_list(qtype::COMPOUND_LIST, 0);
    ///   for i in 0..5{
    ///     unsafe{list.push(new_long(i))}.unwrap();
    ///   }
    ///   unsafe{list.push(increment_reference_count(int))}.unwrap();
    ///   list
    /// }
    /// ```
    /// ```q
    /// q)nums: `libapi_examples 2: (`create_compound_list2; 1);
    /// q)nums[5i]
    /// 0
    /// 1
    /// 2
    /// 3
    /// 4
    /// 5i
    /// ```
    /// # Note
    /// In this example we did not allocate an array as `new_list(qtype::COMPOUND_LIST, 0)` to use `push`.
    ///  As `new_list` initializes the internal list size `n` with its argument, preallocating memory with `new_list` and
    ///  then using `push` will crash. If you want to allocate a memory in advance, you can substitute a value
    ///  after converting the q list object into a slice with [`as_mut_slice`](trait.KUtility.html#tymethod.as_mut_slice).
    ///
    /// # Safety
    /// Using this after preallocating memory with `new_list` will crash.
    unsafe fn push(&mut self, atom: K) -> Result<K, &'static str>;

    /// Add a raw value to a q simple list and returns a pointer to the (potentially reallocated) `K` object.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    /// use kdbplus::qtype;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn create_simple_list2(_: K) -> K{
    ///   let mut list=new_list(qtype::DATE_LIST, 0);
    ///   for i in 0..5{
    ///     list.push_raw(i).unwrap();
    ///   }
    ///   list
    /// }
    /// ```
    /// ```q
    /// q)simple_is_the_best: `lic_api_example 2: (`create_simple_list2; 1);
    /// q)simple_is_the_best[]
    /// 2000.01.01 2000.01.02 2000.01.03 2000.01.04 2000.01.05
    /// ```
    /// # Note
    /// - Concrete type of `T` is not checked. Its type must be either of `I`, `J`, `E` and `F` and it must be compatible
    ///  with the list type. For example, timestamp list requires `J` type atom.
    /// - For symbol list, use [`push_symbol`](#fn.push_symbol) or [`push_symbol_n`](#fn.push_symbol_n).
    fn push_raw<T>(&mut self, atom: T) -> Result<K, &'static str>;

    /// Add a `str` input to a symbol list while enumerating the character array internally.
    ///  Returns a pointer to the (potentially reallocated) `K` object.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    /// use kdbplus::qtype;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn create_symbol_list2(_: K) -> K{
    ///   let mut list=new_list(qtype::SYMBOL_LIST, 0);
    ///   list.push_symbol("Abraham").unwrap();
    ///   list.push_symbol("Isaac").unwrap();
    ///   list.push_symbol("Jacob").unwrap();
    ///   list.push_symbol_n("Josephine", 6).unwrap();
    ///   list
    /// }
    /// ```
    /// ```q
    /// q)summon:`libapi_examples 2: (`create_symbol_list2; 1)
    /// q)summon[]
    /// `Abraham`Isaac`Jacob`Joseph
    /// q)`Abraham`Isaac`Jacob`Joseph ~ summon[]
    /// 1b
    /// ```
    /// # Note
    /// In this example we did not allocate an array as `new_list(qtype::SYMBOL_LIST as I, 0)` to use `push_symbol`.
    ///  As `new_list` initializes the internal list size `n` with its argument, preallocating memory with `new_list`
    ///  and then using `push_symbol` will crash. If you want to allocate a memory in advance, you can substitute a value
    ///  after converting the q list object into a slice with [`as_mut_slice`](trait.KUtility.html#tymethod.as_mut_slice).
    fn push_symbol(&mut self, symbol: &str) -> Result<K, &'static str>;

    /// Add the first `n` characters of a `str` input to a symbol list while enumerating the character array internally.
    ///  Returns a pointer to the (potentially reallocated) `K` object.
    /// # Example
    /// See the example of [`push_symbol`](#fn.push_symbol).
    fn push_symbol_n(&mut self, symbol: &str, n: I) -> Result<K, &'static str>;

    /// Get the length of q object. The meaning of the returned value varies according to the type:
    /// - atom: 1
    /// - list: The number of elements in the list.
    /// - table: The number of rows.
    /// - dictionary: The number of keys.
    /// - general null: 1
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn numbers(obj: K) -> K{
    ///   let count=format!("{} people are in numbers", obj.len());
    ///   new_string(&count)
    /// }
    /// ```
    /// ```q
    /// q)census: `libapi_examples 2: (`numbers; 1);
    /// q)census[(::)]
    /// "1 people are in numbers"
    /// q)census[til 4]
    /// "4 people are in numbers"
    /// q)census[`a`b!("many"; `split`asunder)]
    /// "2 people are in numbers"
    /// q)census[([] id: til 1000)]
    /// "1000 people are in numbers"
    /// ```
    fn len(&self) -> i64;

    /// Get a type of `K` object.
    fn get_type(&self) -> i8;

    /// Set a type of `K` object.
    /// # Example
    /// See the example of [`load_as_q_function](fn.load_as_q_function.html).
    fn set_type(&mut self, qtype: i8);

    /// Set an attribute to q list object.
    /// # Example
    /// ```no_run
    /// use kdbplus::qattribute;
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn labeling(mut list: K) -> K{
    ///   match list.set_attribute(qattribute::SORTED){
    ///     Ok(_) => increment_reference_count(list),
    ///     Err(error) => new_error(error)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)setter: `libapi_examples.so (`labeling; 1);
    /// q)setter 1 2 3
    /// `s#1 2 3
    /// q)setter 777
    /// 'not a simple list
    /// ```
    /// # Note
    /// q does NOT validate the attribute. Wrong attribute can lead to suboptimal behavior or application crash if
    ///  you are unfortunate.
    fn set_attribute(&mut self, attribute: i8) -> Result<(), &'static str>;

    /// Serialize q object and return serialized q byte list object on success: otherwise null.
    ///  Mode is either of:
    /// - -1: Serialize within the same process.
    /// - 1: retain enumerations, allow serialization of timespan and timestamp: Useful for passing data between threads
    /// - 2: unenumerate, allow serialization of timespan and timestamp
    /// - 3: unenumerate, compress, allow serialization of timespan and timestamp
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn encrypt(object: K)->K{
    ///   match object.q_ipc_encode(3){
    ///     Ok(bytes) => bytes,
    ///     Err(error) => new_error(error)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)disguise: `libapi_examples 2: (`encrypt; 1);
    /// q)list: (til 3; "abc"; 2018.02.18D04:30:00.000000000; `revive);
    /// q)disguise list
    /// 0x010000004600000000000400000007000300000000000000000000000100000000000000020..
    /// ```
    fn q_ipc_encode(&self, mode: I) -> Result<K, &'static str>;

    /// Deserialize a bytes into q object.
    /// # Example
    /// ```no_run
    /// use kdbplus::api::*;
    ///
    /// #[no_mangle]
    /// pub extern "C" fn decrypt(bytes: K)->K{
    ///   match bytes.q_ipc_decode(){
    ///     Ok(object) => object,
    ///     Err(error) => new_error(error)
    ///   }
    /// }
    /// ```
    /// ```q
    /// q)uncover: `libapi_examples 2: (`decrypt; 1);
    /// q)uncover -8!"What is the purpose of CREATION?"
    /// "What is the purpose of CREATION?"
    /// ```
    fn q_ipc_decode(&self) -> Result<K, &'static str>;
}

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Implementation
//++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% U %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

impl U {
    /// Create 16-byte GUID object.
    pub fn new(guid: [u8; 16]) -> Self {
        U { guid }
    }
}

//%% K %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

unsafe impl Send for k0_inner {}
unsafe impl Send for k0 {}

impl KUtility for K {
    #[inline]
    unsafe fn as_mut_slice<'a, T>(self) -> &'a mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(
                (*self).value.list.G0.as_mut_ptr() as *mut T,
                (*self).value.list.n as usize,
            )
        }
    }

    fn get_row(&self, index: usize, enum_sources: &[&str]) -> Result<K, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::TABLE => {
                let keys = unsafe { (**self).value.table.as_mut_slice::<K>() }[0];
                let values = unsafe { (**self).value.table.as_mut_slice::<K>() }[1];
                if (unsafe { (*values.as_mut_slice::<K>()[0]).value.list }.n as usize) < index + 1 {
                    // Index out of bounds
                    Err("index out of bounds\0")
                } else {
                    let num_columns = unsafe { (*keys).value.list }.n;
                    let row = new_list(qtype::COMPOUND_LIST, num_columns);
                    let row_slice = unsafe { row.as_mut_slice::<K>() };
                    let mut enum_source_index = 0;
                    for (i, column) in unsafe { values.as_mut_slice::<K>().iter_mut().enumerate() }
                    {
                        match column.get_type() {
                            qtype::BOOL_LIST => {
                                row_slice[i] =
                                    new_bool(unsafe { column.as_mut_slice::<G>() }[index] as i32);
                            }
                            qtype::BYTE_LIST => {
                                row_slice[i] =
                                    new_byte(unsafe { column.as_mut_slice::<G>() }[index] as i32);
                            }
                            qtype::SHORT_LIST => {
                                row_slice[i] =
                                    new_short(unsafe { column.as_mut_slice::<H>() }[index] as i32);
                            }
                            qtype::INT_LIST => {
                                row_slice[i] =
                                    new_int(unsafe { column.as_mut_slice::<I>() }[index]);
                            }
                            qtype::LONG_LIST => {
                                row_slice[i] =
                                    new_long(unsafe { column.as_mut_slice::<J>() }[index]);
                            }
                            qtype::REAL_LIST => {
                                row_slice[i] =
                                    new_real(unsafe { column.as_mut_slice::<E>() }[index] as f64);
                            }
                            qtype::FLOAT_LIST => {
                                row_slice[i] =
                                    new_float(unsafe { column.as_mut_slice::<F>() }[index]);
                            }
                            qtype::STRING => {
                                row_slice[i] =
                                    new_char(unsafe { column.as_mut_slice::<G>() }[index] as char);
                            }
                            qtype::SYMBOL_LIST => {
                                row_slice[i] = new_symbol(unsafe {
                                    S_to_str(column.as_mut_slice::<S>()[index])
                                });
                            }
                            qtype::TIMESTAMP_LIST => {
                                row_slice[i] =
                                    new_timestamp(unsafe { column.as_mut_slice::<J>() }[index]);
                            }
                            qtype::MONTH_LIST => {
                                row_slice[i] =
                                    new_month(unsafe { column.as_mut_slice::<I>() }[index]);
                            }
                            qtype::DATE_LIST => {
                                row_slice[i] =
                                    new_date(unsafe { column.as_mut_slice::<I>() }[index]);
                            }
                            qtype::DATETIME_LIST => {
                                row_slice[i] =
                                    new_datetime(unsafe { column.as_mut_slice::<F>() }[index]);
                            }
                            qtype::TIMESPAN_LIST => {
                                row_slice[i] =
                                    new_timespan(unsafe { column.as_mut_slice::<J>() }[index]);
                            }
                            qtype::MINUTE_LIST => {
                                row_slice[i] =
                                    new_minute(unsafe { column.as_mut_slice::<I>() }[index]);
                            }
                            qtype::SECOND_LIST => {
                                row_slice[i] =
                                    new_second(unsafe { column.as_mut_slice::<I>() }[index]);
                            }
                            qtype::TIME_LIST => {
                                row_slice[i] =
                                    new_time(unsafe { column.as_mut_slice::<I>() }[index]);
                            }
                            qtype::ENUM_LIST => {
                                if enum_sources.len() <= enum_source_index {
                                    // Index out of bounds
                                    unsafe { decrement_reference_count(row) };
                                    return Err("insufficient enum sources\0");
                                }
                                let enum_value = new_enum(
                                    enum_sources[enum_source_index],
                                    unsafe { column.as_mut_slice::<J>() }[index],
                                );
                                if unsafe { (*enum_value).qtype } == qtype::ERROR {
                                    // Error in creating enum object.
                                    unsafe { decrement_reference_count(row) };
                                    let error =
                                        unsafe { S_to_str(unsafe { (*enum_value).value.symbol }) };
                                    unsafe { decrement_reference_count(enum_value) };
                                    return Err(error);
                                } else {
                                    row_slice[i] = enum_value;
                                    enum_source_index += 1;
                                }
                            }
                            qtype::COMPOUND_LIST => {
                                // Increment reference count since compound list consumes the element.
                                row_slice[i] = increment_reference_count(
                                    unsafe { column.as_mut_slice::<K>() }[index],
                                );
                            }
                            // There are no other list type
                            _ => unreachable!(),
                        }
                    }
                    Ok(unsafe { new_dictionary(increment_reference_count(keys), row) })
                }
            }
            _ => Err("not a table\0"),
        }
    }

    #[inline]
    fn get_bool(&self) -> Result<bool, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::BOOL_ATOM => Ok(unsafe { (**self).value.byte != 0 }),
            _ => Err("not a bool\0"),
        }
    }

    #[inline]
    fn get_guid(&self) -> Result<[u8; 16], &'static str> {
        match unsafe { (**self).qtype } {
            qtype::GUID_ATOM => {
                Ok(
                    unsafe { std::slice::from_raw_parts((**self).value.list.G0.as_ptr(), 16) }
                        .try_into()
                        .unwrap(),
                )
            }
            _ => Err("not a GUID\0"),
        }
    }

    #[inline]
    fn get_byte(&self) -> Result<u8, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::BYTE_ATOM => Ok(unsafe { (**self).value.byte }),
            _ => Err("not a byte\0"),
        }
    }

    #[inline]
    fn get_short(&self) -> Result<i16, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::SHORT_ATOM => Ok(unsafe { (**self).value.short }),
            _ => Err("not a short\0"),
        }
    }

    #[inline]
    fn get_int(&self) -> Result<i32, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::INT_ATOM
            | qtype::MONTH_ATOM
            | qtype::DATE_ATOM
            | qtype::MINUTE_ATOM
            | qtype::SECOND_ATOM
            | qtype::TIME_ATOM => Ok(unsafe { (**self).value.int }),
            _ => Err("not an int\0"),
        }
    }

    #[inline]
    fn get_long(&self) -> Result<i64, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::LONG_ATOM | qtype::TIMESTAMP_ATOM | qtype::TIMESPAN_ATOM | qtype::ENUM_ATOM => {
                Ok(unsafe { (**self).value.long })
            }
            _ => Err("not a long\0"),
        }
    }

    #[inline]
    fn get_real(&self) -> Result<f32, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::REAL_ATOM => Ok(unsafe { (**self).value.real }),
            _ => Err("not a real\0"),
        }
    }

    #[inline]
    fn get_float(&self) -> Result<f64, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::FLOAT_ATOM | qtype::DATETIME_ATOM => Ok(unsafe { (**self).value.float }),
            _ => Err("not a float\0"),
        }
    }

    #[inline]
    fn get_char(&self) -> Result<char, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::CHAR => Ok(unsafe { (**self).value.byte as char }),
            _ => Err("not a char\0"),
        }
    }

    #[inline]
    fn get_symbol(&self) -> Result<&str, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::SYMBOL_ATOM => Ok(unsafe { S_to_str(unsafe { (**self).value.symbol }) }),
            _ => Err("not a symbol\0"),
        }
    }

    #[inline]
    fn get_str(&self) -> Result<&str, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::STRING => Ok(unsafe { str::from_utf8_unchecked_mut(self.as_mut_slice::<G>()) }),
            _ => Err("not a string\0"),
        }
    }

    #[inline]
    fn get_string(&self) -> Result<String, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::STRING => {
                Ok(unsafe { String::from_utf8_unchecked(self.as_mut_slice::<G>().to_vec()) })
            }
            _ => Err("not a string\0"),
        }
    }

    #[inline]
    fn get_dictionary(&self) -> Result<K, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::TABLE => Ok(unsafe { (**self).value.table }),
            _ => Err("not a table\0"),
        }
    }

    #[inline]
    fn get_error_string(&self) -> Result<&str, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::ERROR => {
                if !unsafe { (**self).value.symbol }.is_null() {
                    Ok(unsafe { S_to_str(unsafe { (**self).value.symbol }) })
                } else {
                    Err("not an error\0")
                }
            }
            _ => Err("not an error\0"),
        }
    }

    #[inline]
    fn get_attribute(&self) -> i8 {
        unsafe { (**self).attribute }
    }

    #[inline]
    fn get_refcount(&self) -> i32 {
        unsafe { (**self).refcount }
    }

    #[inline]
    unsafe fn append(&mut self, list: K) -> Result<K, &'static str> {
        if unsafe { (**self).qtype } >= 0 && unsafe { (**self).qtype } == unsafe { (*list).qtype } {
            let result = Ok(unsafe { native::jv(self, list) });
            // Free appended list for internally created object.
            unsafe { native::r0(list) };
            result
        } else {
            Err("not a list or types do not match\0")
        }
    }

    #[inline]
    unsafe fn push(&mut self, atom: K) -> Result<K, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::COMPOUND_LIST => Ok(unsafe { native::jk(self, atom) }),
            _ => Err("not a list or types do not match\0"),
        }
    }

    #[inline]
    fn push_raw<T>(&mut self, mut atom: T) -> Result<K, &'static str> {
        match unsafe { (**self).qtype } {
            _t @ qtype::BOOL_LIST..=qtype::ENUM_LIST => {
                Ok(unsafe { native::ja(self, std::mem::transmute::<*mut T, *mut V>(&mut atom)) })
            }
            _ => Err("not a simple list or types do not match\0"),
        }
    }

    #[inline]
    fn push_symbol(&mut self, symbol: &str) -> Result<K, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::SYMBOL_LIST => Ok(unsafe { native::js(self, native::ss(str_to_S!(symbol))) }),
            _ => Err("not a symbol list\0"),
        }
    }

    #[inline]
    fn push_symbol_n(&mut self, symbol: &str, n: I) -> Result<K, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::SYMBOL_LIST => Ok(unsafe { native::js(self, native::sn(str_to_S!(symbol), n)) }),
            _ => Err("not a symbol list or types do not match\0"),
        }
    }

    #[inline]
    fn len(&self) -> i64 {
        match unsafe { (**self).qtype } {
            _t @ qtype::ENUM_ATOM..=qtype::BOOL_ATOM => {
                // Atom
                1
            }
            _t @ qtype::COMPOUND_LIST..=qtype::ENUM_LIST => {
                // List
                unsafe { (**self).value.list }.n
            }
            qtype::TABLE => {
                // Table
                // Access underlying table (dictionary structure) and retrieve values of the dictionary.
                // The values (columns) is assured to be a list of lists as it is a table. so cast it to list of `K`.
                // Size of the table is a length of the first column.
                unsafe {
                    (*((**self).value.table).as_mut_slice::<K>()[1].as_mut_slice::<K>()[0])
                        .value
                        .list
                }
                .n
            }
            qtype::DICTIONARY | qtype::SORTED_DICTIONARY => {
                // Dictionary
                // Access to keys of the dictionary and retrieve its length.
                unsafe { (*(*self).as_mut_slice::<K>()[0]).value.list }.n
            }
            _ => {
                // General null, function, foreign
                1
            }
        }
    }

    #[inline]
    fn get_type(&self) -> i8 {
        unsafe { (**self).qtype }
    }

    #[inline]
    fn set_type(&mut self, qtype: i8) {
        unsafe { (**self).qtype = qtype };
    }

    #[inline]
    fn set_attribute(&mut self, attribute: i8) -> Result<(), &'static str> {
        match unsafe { (**self).qtype } {
            _t @ qtype::BOOL_LIST..=qtype::TIME_LIST => {
                unsafe { (**self).attribute = attribute };
                Ok(())
            }
            _ => Err("not a simple list\0"),
        }
    }

    #[inline]
    fn q_ipc_encode(&self, mode: I) -> Result<K, &'static str> {
        let result = unsafe { error_to_string(unsafe { native::b9(mode, *self) }) };
        match unsafe { (*result).qtype } {
            qtype::ERROR => {
                unsafe { decrement_reference_count(result) };
                Err("failed to encode\0")
            }
            _ => Ok(result),
        }
    }

    #[inline]
    fn q_ipc_decode(&self) -> Result<K, &'static str> {
        match unsafe { (**self).qtype } {
            qtype::BYTE_LIST => {
                let result = unsafe { error_to_string(unsafe { native::d9(*self) }) };
                match unsafe { (*result).qtype } {
                    qtype::ERROR => {
                        unsafe { decrement_reference_count(result) };
                        Err("failed to decode\0")
                    }
                    _ => Ok(result),
                }
            }
            _ => Err("not bytes\0"),
        }
    }
}

impl k0 {
    /// Derefer `k0` as a mutable slice. For supported types, see [`as_mut_slice`](trait.KUtility.html#tymethod.as_mut_slice).
    /// # Note
    /// Used if `K` needs to be sent to another thread. `K` cannot implement `Send` and therefore
    ///  its inner struct must be sent instead.
    /// # Example
    /// See the example of [`setm`](native/fn.setm.html).
    #[inline]
    pub fn as_mut_slice<'a, T>(&mut self) -> &'a mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.value.list.G0.as_mut_ptr() as *mut T,
                self.value.list.n as usize,
            )
        }
    }
}

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Utility
//++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Utility %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Convert `S` to `&str`. This function is intended to convert symbol type (null-terminated char-array) to `str`.
/// # Extern
/// ```no_run
/// use kdbplus::*;
/// use kdbplus::api::*;
///
/// #[no_mangle]
/// pub extern "C" fn print_symbol(symbol: K) -> K{
///   unsafe{
///     if (*symbol).qtype == qtype::SYMBOL_ATOM{
///       println!("symbol: `{}", S_to_str((*symbol).value.symbol));
///     }
///     // return null
///     KNULL
///   }
/// }
/// ```
/// ```q
/// q)print_symbol:`libapi_examples 2: (`print_symbol; 1)
/// q)a:`kx
/// q)print_symbol a
/// symbol: `kx
/// ```
///
/// # Safety
/// * The memory pointed to by `cstring` must contain a valid nul terminator at the
///   end of the string.
/// * `cstring` must be [valid](core::ptr#safety) for reads of bytes up to and including the null terminator.
///   This means in particular:
///   * The entire memory range of this `CStr` must be contained within a single allocated object!
///   * `cstring` must be non-null even for a zero-length cstr.
/// * The memory referenced by the returned `CStr` must not be mutated for
///   the duration of lifetime `'a`.
#[inline]
pub unsafe fn S_to_str<'a>(cstring: S) -> &'a str {
    unsafe { CStr::from_ptr(cstring).to_str().unwrap() }
}

/// Convert null-terminated `&str` to `S`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
///
/// #[no_mangle]
/// pub extern "C" fn pingpong2(_: K) -> K{
///   unsafe{native::k(0, null_terminated_str_to_S("ping\0"), new_int(77), KNULL)}
/// }
/// ```
/// ```q
/// q)ping:{[int] `$string[int], "_pong!!"};
/// q)pingpong: `libapi_examples 2: (`pingpong2; 1);
/// q)pingpong[]
/// `77_pong!!
/// ```
#[inline]
pub fn null_terminated_str_to_S(string: &str) -> S {
    unsafe { CStr::from_bytes_with_nul_unchecked(string.as_bytes()).as_ptr() as S }
}

/// Convert null terminated `&str` into `const_S`. Expected usage is to build
///  a q error object with `krr`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// use kdbplus::api::native::*;
/// use kdbplus::qtype;
///
/// pub extern "C" fn must_be_int2(obj: K) -> K{
///   unsafe{
///     if (*obj).qtype != qtype::INT_ATOM{
///       krr(null_terminated_str_to_const_S("not an int\0"))
///     }
///     else{
///       KNULL
///     }
///   }
/// }
/// ```
/// ```q
/// q)check:`libapi_examples 2: (`must_be_int; 1)
/// q)a:100
/// q)check a
/// 'not an int
///   [0]  check a
///        ^
/// q)a:42i
/// q)check a
/// ```
pub fn null_terminated_str_to_const_S(string: &str) -> const_S {
    string.as_bytes().as_ptr() as const_S
}

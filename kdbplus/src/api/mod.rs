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
//! #[macro_use]
//! extern crate kdbplus;
//! use kdbplus::api::*;
//! use kdbplus::qtype;
//! 
//! #[no_mangle]
//! pub extern "C" fn create_symbol_list2(_: K) -> K{
//!   let mut list=new_simple_list(qtype::SYMBOL_LIST, 0);
//!   list.push_symbol("Abraham").unwrap();
//!   list.push_symbol("Isaac").unwrap();
//!   list.push_symbol("Jacob").unwrap();
//!   list.push_symbol_n("Josephine", 6).unwrap();
//!   list
//! }
//! 
//! #[no_mangle]
//! fn no_panick(func: K, args: K) -> K{
//!   let result=error_to_string(apply(func, args));
//!   if result.get_type() == qtype::ERROR{
//!     println!("FYI: {}", result.get_symbol().unwrap());
//!     // Decrement reference count of the error object which is no longer used.
//!     decrement_reference_count(result);
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
//!   let keys=new_simple_list(qtype::SYMBOL_LIST, 2);
//!   let keys_slice=keys.as_mut_slice::<S>();
//!   keys_slice[0]=internalize(str_to_S!("time"));
//!   keys_slice[1]=internalize_n(str_to_S!("temperature_and_humidity"), 11);
//! 
//!   // Build values
//!   let values=new_simple_list(qtype::COMPOUND_LIST, 2);
//!   let time=new_simple_list(qtype::TIMESTAMP_LIST, 3);
//!   // 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392
//!   time.as_mut_slice::<J>().copy_from_slice(&[119067859167018272_i64, 201766609419710368, 271897944018691392]);
//!   let temperature=new_simple_list(qtype::FLOAT_LIST, 3);
//!   temperature.as_mut_slice::<F>().copy_from_slice(&[22.1_f64, 24.7, 30.5]);
//!   values.as_mut_slice::<K>().copy_from_slice(&[time, temperature]);
//!   
//!   flip(new_dictionary(keys, values))
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

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                              Settings                                //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                            Load Libraries                            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

use std::str;
use std::ffi::CStr;
use std::convert::TryInto;
use std::os::raw::{c_char, c_double, c_float, c_int, c_longlong, c_short, c_schar, c_uchar, c_void};
use super::qtype;
pub mod native;

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          Global Variables                            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// `K` nullptr. This value is used as general null returned value (`(::)`).
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
pub const KNULL:K=0 as K;

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                                Macros                                //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Utility %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Convert `&str` to `S` (null-terminated character array).
/// # Example
/// ```no_run
/// #[macro_use]
/// use kdbplus::api::*;
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

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                               Structs                                //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

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
pub struct U{
  pub guid: [G; 16]
}

//%% K %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Underlying list value of q object.
/// # Note
/// Usually this struct does not need to be accessed this struct directly unless user wants to
///  access via a raw pointer for non-trivial stuff. 
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct k0_list{
  /// Length of the list.
  pub n: J,
  /// Pointer referring to the head of the list. This pointer will be interpreted
  ///  as various types when accessing `K` object to edit the list with
  ///  [`as_mut_slice`](trait.KUtility.html#tymethod.as_mut_slice).
  pub G0: [G; 1]
}

/// Underlying atom value of q object.
/// # Note
/// Usually this struct does not need to be accessed directly unless user wants to
///  access via a raw pointer for non-trivial stuff. 
#[derive(Clone, Copy)]
#[repr(C)]
pub union k0_inner{
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
  pub list: k0_list
}

/// Underlying struct of `K` object.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct k0{
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
  pub value: k0_inner
}

/// Struct representing q object.
pub type K=*mut k0;

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                               Structs                                //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% KUtility %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

pub trait KUtility{
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
  ///     long_list.as_mut_slice::<J>()[1]=30000_i64;
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
  fn as_mut_slice<'a, T>(self) -> &'a mut[T];

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
  ///     Ok(dictionary) => dictionary.as_mut_slice::<K>()[0].q_ipc_encode(3).unwrap(),
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

  /// Get an underlying error symbol as `&str`.
  /// # Example
  /// See the example of [`error_to_string`](fn.error_to_string.html).
  fn get_error_string(&self) -> Result<&str, &'static str>;

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
  ///   if let Err(err) = list1.append(list2){
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
  fn append(&mut self, list: K)->Result<K, &'static str>;

  /// Add a q object to a q compound list.
  ///  Returns a pointer to the (potentially reallocated) `K` object.
  /// # Example
  /// ```no_run
  /// use kdbplus::api::*;
  /// use kdbplus::qtype;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_compound_list(int: K) -> K{
  ///   let mut list=new_simple_list(qtype::COMPOUND_LIST, 0);
  ///   for i in 0..5{
  ///     list.push(new_long(i)).unwrap();
  ///   }
  ///   list.push(increment_reference_count(int)).unwrap();
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
  /// In this example we did not allocate an array as `new_simple_list(qtype::COMPOUND_LIST, 0)` to use `push`.
  ///  As `new_simple_list` initializes the internal list size `n` with its argument, preallocating memory with `new_simple_list` and
  ///  then using `push` will crash. If you want to allocate a memory in advance, you can substitute a value
  ///  after converting the q list object into a slice with [`as_mut_slice`](rait.KUtility.html#tymethod.as_mut_slice).
  fn push(&mut self, atom: K)->Result<K, &'static str>;

  /// Add a raw value to a q simple list and returns a pointer to the (potentially reallocated) `K` object.
  /// # Example
  /// ```no_run
  /// use kdbplus::api::*;
  /// use kdbplus::qtype;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_simple_list2(_: K) -> K{
  ///   let mut list=new_simple_list(qtype::DATE_LIST, 0);
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
  /// - Concrete type of `T` is not checked. Its type must be either of `I`, `J` and `F` and it must be compatible
  ///  with the list type. For example, timestamp list requires `J` type atom.
  /// - For symbol list, use [`push_symbol`](#fn.push_symbol) or [`push_symbol_n`](#fn.push_symbol_n).
  fn push_raw<T>(&mut self, atom: T)->Result<K, &'static str>;

  /// Add an internalized char array to symbol list.
  ///  Returns a pointer to the (potentially reallocated) `K` object.
  /// # Example
  /// ```no_run
  /// use kdbplus::api::*;
  /// use kdbplus::qtype;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_symbol_list2(_: K) -> K{
  ///   let mut list=new_simple_list(qtype::SYMBOL_LIST, 0);
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
  /// In this example we did not allocate an array as `new_simple_list(qtype::SYMBOL_LIST as I, 0)` to use `push_symbol`.
  ///  As `new_simple_list` initializes the internal list size `n` with its argument, preallocating memory with `new_simple_list`
  ///  and then using `push_symbol` will crash. If you want to allocate a memory in advance, you can substitute a value
  ///  after converting the q list object into a slice with [`as_mut_slice`](trait.KUtility.html#tymethod.as_mut_slice).
  fn push_symbol(&mut self, symbol: &str)->Result<K, &'static str>;

  /// Add an internalized char array to symbol list.
  ///  Returns a pointer to the (potentially reallocated) `K` object.
  /// # Example
  /// See the example of [`push_symbol`](#fn.push_symbol).
  fn push_symbol_n(&mut self, symbol: &str, n: I)->Result<K, &'static str>;

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

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                            Implementation                            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% U %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

impl U{
  /// Create 16-byte GUID object.
  pub fn new(guid: [u8; 16]) -> Self{
    U{guid:guid}
  }
}

//%% K %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

unsafe impl Send for k0_inner{}
unsafe impl Send for k0{}

impl KUtility for K{
  #[inline]
  fn as_mut_slice<'a, T>(self) -> &'a mut[T]{
    unsafe{
      std::slice::from_raw_parts_mut((*self).value.list.G0.as_mut_ptr() as *mut T, (*self).value.list.n as usize)
    }
  }

  #[inline]
  fn get_bool(&self) -> Result<bool, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::BOOL_ATOM => Ok(unsafe{(**self).value.byte != 0}),
      _ => Err("not a bool\0")
    }
  }

  #[inline]
  fn get_guid(&self) -> Result<[u8; 16], &'static str>{
    match unsafe{(**self).qtype}{
      qtype::GUID_ATOM => {
        Ok(unsafe{std::slice::from_raw_parts((**self).value.list.G0.as_ptr(), 16)}.try_into().unwrap())
      },
      _ => Err("not a GUID\0")
    }
  }

  #[inline]
  fn get_byte(&self) -> Result<u8, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::BYTE_ATOM => Ok(unsafe{(**self).value.byte}),
      _ => Err("not a byte\0")
    }
  }

  #[inline]
  fn get_short(&self) -> Result<i16, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::SHORT_ATOM => Ok(unsafe{(**self).value.short}),
      _ => Err("not a short\0")
    }
  }

  #[inline]
  fn get_int(&self) -> Result<i32, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::INT_ATOM | qtype::MONTH_ATOM | qtype::DATE_ATOM | qtype::MINUTE_ATOM | qtype::SECOND_ATOM | qtype::TIME_ATOM => Ok(unsafe{(**self).value.int}),
      _ => Err("not an int\0")
    }
  }

  #[inline]
  fn get_long(&self) -> Result<i64, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::LONG_ATOM | qtype::TIMESTAMP_ATOM | qtype::TIMESPAN_ATOM => Ok(unsafe{(**self).value.long}),
      _ => Err("not a long\0")
    }
  }

  #[inline]
  fn get_real(&self) -> Result<f32, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::REAL_ATOM => Ok(unsafe{(**self).value.real}),
      _ => Err("not a real\0")
    }
  }

  #[inline]
  fn get_float(&self) -> Result<f64, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::FLOAT_ATOM | qtype::DATETIME_ATOM => Ok(unsafe{(**self).value.float}),
      _ => Err("not a float\0")
    }
  }

  #[inline]
  fn get_char(&self) -> Result<char, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::CHAR => Ok(unsafe{(**self).value.byte as char}),
      _ => Err("not a char\0")
    }
  }

  #[inline]
  fn get_symbol(&self) -> Result<&str, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::SYMBOL_ATOM => Ok(S_to_str(unsafe{(**self).value.symbol})),
      _ => Err("not a symbol\0")
    }
  }

  #[inline]
  fn get_str(&self) -> Result<&str, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::STRING => {
        Ok(unsafe{str::from_utf8_unchecked_mut(self.as_mut_slice::<G>())})
      },
      _ => Err("not a string\0")
    }
  }

  #[inline]
  fn get_string(&self) -> Result<String, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::STRING => {
        Ok(unsafe{String::from_utf8_unchecked(self.as_mut_slice::<G>().to_vec())})
      },
      _ => Err("not a string\0")
    }
  }

  #[inline]
  fn get_dictionary(&self) -> Result<K, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::TABLE => {
        Ok(unsafe{(**self).value.table})
      },
      _ => Err("not a table\0")
    }
  }

  #[inline]
  fn get_error_string(&self) -> Result<&str, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::ERROR => {
        Ok(S_to_str(unsafe{(**self).value.symbol}))
      },
      _ => Err("not an error\0")
    }
  }

  #[inline]
  fn get_attribute(&self) -> i8{
    unsafe{(**self).attribute}
  }

  #[inline]
  fn get_refcount(&self) -> i32{
    unsafe{(**self).refcount}
  }

  #[inline]
  fn append(&mut self, list: K)->Result<K, &'static str>{
    if unsafe{(**self).qtype} >= 0 && unsafe{(**self).qtype} == unsafe{(*list).qtype}{
      Ok(unsafe{native::jv(self, list)})
    }
    else{
      Err("not a list or types do not match\0")
    }
  }

  #[inline]
  fn push(&mut self, atom: K)->Result<K, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::COMPOUND_LIST => Ok(unsafe{native::jk(self, atom)}),
      _ => Err("not a list or types do not match\0")
    }
  }

  #[inline]
  fn push_raw<T>(&mut self, mut atom: T)->Result<K, &'static str>{
    match unsafe{(**self).qtype}{
      _t@qtype::BOOL_LIST..=qtype::TIME_LIST => Ok(unsafe{native::ja(self, std::mem::transmute::<*mut T, *mut V>(&mut atom))}),
      _ => Err("not a simple list or types do not match\0")
    }
  }

  #[inline]
  fn push_symbol(&mut self, symbol: &str)->Result<K, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::SYMBOL_LIST => Ok(unsafe{native::js(self, native::ss(str_to_S!(symbol)))}),
      _ => Err("not a symbol list\0")
    }
  }

  #[inline]
  fn push_symbol_n(&mut self, symbol: &str, n: I)->Result<K, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::SYMBOL_LIST => Ok(unsafe{native::js(self, native::sn(str_to_S!(symbol), n))}),
      _ => Err("not a symbol list or types do not match\0")
    }
  }

  #[inline]
  fn len(&self) -> i64{
    match unsafe{(**self).qtype}{
      _t@qtype::TIME_ATOM..=qtype::BOOL_ATOM => {
        // Atom
        1
      },
      _t@qtype::COMPOUND_LIST..=qtype::TIME_LIST => {
        // List
        unsafe{(**self).value.list}.n
      },
      qtype::TABLE => {
        // Table
        // Access underlying table (dictionary structure) and retrieve values of the dictionary.
        // The values (columns) is assured to be a list of lists as it is a table. so cast it to list of `K`.
        // Size of the table is a length of the first column.
        unsafe{(*((**self).value.table).as_mut_slice::<K>()[1].as_mut_slice::<K>()[0]).value.list}.n
      },
      qtype::DICTIONARY | qtype::SORTED_DICTIONARY => {
        // Dictionary
        // Access to keys of the dictionary and retrieve its length.
        unsafe{(*(*self).as_mut_slice::<K>()[0]).value.list}.n
      },
      _ => {
        // General null, function, foreign
        1
      },
    }
  }

  #[inline]
  fn get_type(&self) -> i8{
    unsafe{(**self).qtype}
  }

  #[inline]
  fn set_type(&mut self, qtype: i8){
    unsafe{(**self).qtype=qtype};
  }

  #[inline]
  fn set_attribute(&mut self, attribute: i8) -> Result<(), &'static str>{
    match unsafe{(**self).qtype}{
      _t@qtype::BOOL_LIST..=qtype::TIME_LIST => Ok(unsafe{(**self).attribute=attribute}),
      _ => Err("not a simple list\0")
    }
  }

  #[inline]
  fn q_ipc_encode(&self, mode: I) -> Result<K, &'static str>{
    let result=error_to_string(unsafe{
      native::b9(mode, *self)
    });
    match unsafe{(*result).qtype}{
      qtype::ERROR => {
        decrement_reference_count(result);
        Err("failed to encode\0")
      },
      _ => Ok(result)
    }
  }

  #[inline]
  fn q_ipc_decode(&self) -> Result<K, &'static str>{
    match unsafe{(**self).qtype}{
      qtype::BYTE_LIST => {
        let result=error_to_string(unsafe{
          native::d9(*self)
        });
        match unsafe{(*result).qtype}{
          qtype::ERROR => {
            decrement_reference_count(result);
            Err("failed to decode\0")
          },
          _ => Ok(result)
        }
      },
      _ => Err("not bytes\0")
    }
  }
}

impl k0{
  /// Derefer `k0` as a mutable slice. For supported types, see [`as_mut_slice`](trait.KUtility.html#tymethod.as_mut_slice).
  /// # Note
  /// Used if `K` needs to be sent to another thread. `K` cannot implement `Send` and therefore
  ///  its inner struct must be sent instead.
  /// # Example
  /// See the example of [`setm`](native/fn.setm.html).
  #[inline]
  pub fn as_mut_slice<'a, T>(&mut self) -> &'a mut[T]{
    unsafe{
      std::slice::from_raw_parts_mut(self.value.list.G0.as_mut_ptr() as *mut T, self.value.list.n as usize)
    }
  }
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                              Utility                                 //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Utility %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

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
#[inline]
pub fn S_to_str<'a>(cstring: S) -> &'a str{
  unsafe{
    CStr::from_ptr(cstring).to_str().unwrap()
  }
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
  unsafe{
    CStr::from_bytes_with_nul_unchecked(string.as_bytes()).as_ptr() as S
  }
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

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                              Re-export                               //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Constructor %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Constructor of q bool object. Relabeling of `kb`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_bool(_: K) -> K{
///   new_bool(0)
/// }
/// ```
/// ```q
/// q)no: `libapi_examples 2: (`create_bool; 1);
/// q)no[]
/// 0b
/// ```
#[inline]
pub fn new_bool(boolean: I) -> K{
  unsafe{
    native::kb(boolean)
  }
}

/// Constructor of q GUID object. Relabeling of `ku`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_guid(_: K) -> K{
///   new_guid([0x1e_u8, 0x11, 0x17, 0x0c, 0x42, 0x24, 0x25, 0x2c, 0x1c, 0x14, 0x1e, 0x22, 0x4d, 0x3d, 0x46, 0x24])
/// }
/// ```
/// ```q
/// q)create_guid: `libapi_examples 2: (`create_guid; 1);
/// q)create_guid[]
/// 1e11170c-4224-252c-1c14-1e224d3d4624
/// ```
#[inline]
pub fn new_guid(guid: [G; 16]) -> K{
  unsafe{
    native::ku(U::new(guid))
  }
}

/// Constructor of q byte object. Relabeling of `kg`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_byte(_: K) -> K{
///   new_byte(0x3c)
/// }
/// ```
/// ```q
/// q)create_byte: `libapi_examples 2: (`create_byte; 1);
/// q)create_byte[]
/// 0x3c
/// ```
#[inline]
pub fn new_byte(byte: I) -> K{
  unsafe{
    native::kg(byte)
  }
}

/// Constructor of q short object. Relabeling of `kh`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_short(_: K) -> K{
///   new_short(-144)
/// }
/// ```
/// ```q
/// q)shortage: `libapi_examples 2: (`create_short; 1);
/// q)shortage[]
/// -144h
/// ```
#[inline]
pub fn new_short(short: I) -> K{
  unsafe{
    native::kh(short)
  }
}

/// Constructor of q int object. Relabeling of `ki`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_int(_: K) -> K{
///   new_int(86400000)
/// }
/// ```
/// ```q
/// q)trvial: `libapi_examples 2: (`create_int; 1);
/// q)trivial[]
/// 86400000i
/// ```
#[inline]
pub fn new_int(int: I) -> K{
  unsafe{
    native::ki(int)
  }
}

/// Constructor of q long object. Relabeling of `kj`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_long(_: K) -> K{
///   new_long(-668541276001729000)
/// }
/// ```
/// ```q
/// q)lengthy: `libapi_examples 2: (`create_long; 1);
/// q)lengthy[]
/// -668541276001729000
/// ```
#[inline]
pub fn new_long(long: J) -> K{
  unsafe{
    native::kj(long)
  }
}

/// Constructor of q real object. Relabeling of `ke`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_real(_: K) -> K{
///   new_real(0.00324)
/// }
/// ```
/// ```q
/// q)reality: `libapi_examples 2: (`create_real; 1);
/// q)reality[]
/// 0.00324e
/// ```
#[inline]
pub fn new_real(real: F) -> K{
  unsafe{
    native::ke(real)
  }
}

/// Constructor of q float object. Relabeling of `kf`.
/// # Example
/// ```
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_float(_: K) -> K{
///   new_float(-6302.620)
/// }
/// ```
/// ```q
/// q)coffee_float: `libapi_examples 2: (`create_float; 1);
/// q)coffee_float[]
/// -6302.62
/// ```
#[inline]
pub fn new_float(float: F) -> K{
  unsafe{
    native::kf(float)
  }
}

///  Constructor of q char object. Relabeling of `kc`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_char2(_: K) -> K{
///   new_char('t')
/// }
/// ```
/// ```q
/// q)heavy: `libapi_examples 2: (`create_char2; 1);
/// q)heavy[]
/// "t"
/// ```
#[inline]
pub fn new_char(character: char) -> K{
  unsafe{
    native::kc(character as I)
  }
}

/// Constructor of q symbol object. Relabeling of `ks`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_symbol2(_: K) -> K{
///   new_symbol("symbolic")
/// }
/// ```
/// ```q
/// q)hard: `libapi_examples 2: (`create_symbol2; 1);
/// q)hard[]
/// `symbolic
/// q)`symbolic ~ hard[]
/// 1b
/// ```
#[inline]
pub fn new_symbol(symbol: &str) -> K{
  unsafe{
    native::ks(str_to_S!(symbol))
  }
}

/// Constructor of q timestamp from elapsed time in nanoseconds since kdb+ epoch (`2000.01.01`). Relabeling of `ktj`.
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_timestamp2(_: K) -> K{
///   // 2015.03.16D00:00:00:00.000000000
///   new_timestamp(479779200000000000)
/// }
/// ```
/// ```q
/// q)stamp: `libapi_examples 2: (`create_timestamp2; 1);
/// q)stamp[]
/// 2015.03.16D00:00:00.000000000
/// ```
#[inline]
pub fn new_timestamp(nanoseconds: J) -> K{
  unsafe{
    native::ktj(qtype::TIMESTAMP_ATOM as I, nanoseconds)
  }
}

/// Create a month object from the number of months since kdb+ epoch (`2000.01.01`).
///  This is a complememtal constructor of missing month type.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_month(_: K) -> K{
///   // 2010.07m
///   new_month(126)
/// }
/// ```
/// ```q
/// q)create_month: `libapi_examples 2: (`create_month; 1);
/// q)create_month[]
/// 2010.07m
/// ```
#[inline]
pub fn new_month(months: I) -> K{
  unsafe{
    let month=native::ka(qtype::MONTH_ATOM as I);
    (*month).value.int=months;
    month
  }
}

/// Constructor of q date object. Relabeling of `kd`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_date(_: K) -> K{
///   // 1999.12.25
///   new_date(-7)
/// }
/// ```
/// ```q
/// q)nostradamus: `libapi_examples 2: (`create_date; 1);
/// q)nostradamus[]
/// 1999.12.25
/// ```
#[inline]
pub fn new_date(days: I) -> K{
  unsafe{
    native::kd(days)
  }
}

/// Constructor of q datetime object from the number of days since kdb+ epoch (`2000.01.01`). Relabeling of `kz`.
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_datetime(_: K) -> K{
///   // 2015.03.16T12:00:00:00.000
///   new_datetime(5553.5)
/// }
/// ```
/// ```q
/// q)omega_date: libc_api_examples 2: (`create_datetime; 1);
/// q)omega_date[]
/// 2015.03.16T12:00:00.000
/// ```
#[inline]
pub fn new_datetime(days: F) -> K{
  unsafe{
    native::kz(days)
  }
}

/// Constructor of q timespan object from nanoseconds. Relabeling of `ktj`.
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_timespan2(_: K) -> K{
///   // -1D01:30:00.001234567
///   new_timespan(-91800001234567)
/// }
/// ```
/// ```q
/// q)duration: libc_api_examples 2: (`create_timespan2; 1);
/// q)duration[]
/// -1D01:30:00.001234567
/// ```
#[inline]
pub fn new_timespan(nanoseconds: J) -> K{
  unsafe{
    native::ktj(qtype::TIMESPAN_ATOM as I, nanoseconds)
  }
}

/// Create a month object. This is a complememtal constructor of
///  missing minute type.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_minute(_: K) -> K{
///   // 10:40
///   new_minute(640)
/// }
/// ```
/// ```q
/// q)minty: `libapi_examples 2: (`create_minute; 1);
/// q)minty[]
/// 10:40
/// ```
#[inline]
pub fn new_minute(minutes: I) -> K{
  unsafe{
    let minute=native::ka(qtype::MINUTE_ATOM as I);
    (*minute).value.int=minutes;
    minute
  }
}

/// Create a month object. This is a complememtal constructor of
///  missing second type.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_second(_: K) -> K{
///   // -02:00:00
///   new_second(-7200)
/// }
/// ```
/// ```q
/// q)third: `libapi_examples 2: (`create_second; 1);
/// q)third[]
/// -02:00:00
/// ```
#[inline]
pub fn new_second(seconds: I) -> K{
  unsafe{
    let second=native::ka(qtype::SECOND_ATOM as I);
    (*second).value.int=seconds;
    second
  }
}

/// Constructor of q time object. Relabeling of `kt`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_time(_: K) -> K{
///   // -01:30:00.123
///   new_time(-5400123)
/// }
/// ```
/// ```q
/// q)ancient: libc_api_examples 2: (`create_time; 1);
/// q)ancient[]
/// -01:30:00.123
/// ```
#[inline]
pub fn new_time(milliseconds: I) -> K{
  unsafe{
    native::kt(milliseconds)
  }
}

/// Constructor of q simple list.
/// # Example
/// See the example of [`new_dictionary`](fn.new_dictionary.html).
#[inline]
pub fn new_simple_list(qtype: i8, length: J) -> K{
  unsafe{
    native::ktn(qtype as I, length)
  }
}

/// Constructor of q string object.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_string(_: K) -> K{
///   new_string("this is a text.")
/// }
/// ```
/// ```q
/// q)text: libc_api_examples 2: (`create_string; 1);
/// q)text[]
/// "this is a text."
/// ```
#[inline]
pub fn new_string(string: &str) -> K{
  unsafe{
    native::kp(str_to_S!(string))
  }
}

/// Constructor if q string object with a fixed length.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_string2(_: K) -> K{
///   new_string_n("The meeting was too long and I felt it s...", 24)
/// }
/// ```
/// ```q
/// q)speak_inwardly: libc_api_examples 2: (`create_string2; 1);
/// q)speak_inwardly[]
/// "The meeting was too long"
/// ```
#[inline]
pub fn new_string_n(string: &str, length: J) -> K{
  unsafe{
    native::kpn(str_to_S!(string), length)
  }
}

/// Constructor of q dictionary object.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// use kdbplus::qtype;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_dictionary() -> K{
///   let keys=new_simple_list(qtype::INT_LIST, 2);
///   keys.as_mut_slice::<I>()[0..2].copy_from_slice(&[0, 1]);
///   let values=new_simple_list(qtype::COMPOUND_LIST, 2);
///   let date_list=new_simple_list(qtype::DATE_LIST, 3);
///   // 2000.01.01 2000.01.02 2000.01.03
///   date_list.as_mut_slice::<I>()[0..3].copy_from_slice(&[0, 1, 2]);
///   let string=new_string("I'm afraid I would crash the application...");
///   values.as_mut_slice::<K>()[0..2].copy_from_slice(&[date_list, string]);
///   new_dictionary(keys, values)
/// }
/// ```
/// ```q
/// q)create_dictionary: `libapi_examples 2: (`create_dictionary; 1);
/// q)create_dictionary[]
/// 0| 2000.01.01 2000.01.02 2000.01.03
/// 1| "I'm afraid I would crash the application..."
/// ```
#[inline]
pub fn new_dictionary(keys: K, values: K) -> K{
  unsafe{
    native::xD(keys, values)
  }
}

/// Constructor of q general null.
/// # Example
/// ```no_run
/// use kdbplus::qtype;
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn nullify(_: K) -> K{
///   let nulls=new_simple_list(qtype::COMPOUND_LIST, 3);
///   let null_slice=nulls.as_mut_slice::<K>();
///   null_slice[0]=new_null();
///   null_slice[1]=new_string("null is not a general null");
///   null_slice[2]=new_null();
///   nulls
/// }
/// ```
/// ```q
/// q)void: `libapi_examples 2: (`nullify; 1);
/// q)void[]
/// ::
/// "null is not a general null"
/// ::
/// ```
#[inline]
pub fn new_null() -> K{
  unsafe{
    let null=native::ka(qtype::NULL as I);
    (*null).value.byte=0;
    null
  }
}

/// Constructor of q error. The input must be null-terminated.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// pub extern "C" fn thai_kick(_: K) -> K{
///   new_error("Thai kick unconditionally!!\0")
/// }
/// ```
/// ```q
/// q)monstrous: `libapi_examples 2: (`thai_kick; 1);
/// q)monstrous[]
/// 'Thai kick unconditionally!!
/// [0]  monstrous[]
///      ^
/// ```
#[inline]
pub fn new_error(message: &str) -> K{
  unsafe{
    native::krr(null_terminated_str_to_const_S(message))
  }
}

/// Similar to `new_error` but this function appends a system-error message to string `S` before passing it to internal `krr`.
///  The input must be null-terminated.
#[inline]
pub fn new_error_os(message: &str) -> K{
  unsafe{
    native::orr(null_terminated_str_to_const_S(message))
  }
}

/// Convert an error object into usual K object which has the error string in the field `s`.
/// # Example
/// ```no_run
/// use kdbplus::*;
/// use kdbplus::api::*;
/// 
/// extern "C" fn no_panick(func: K, args: K) -> K{
///   let result=error_to_string(apply(func, args));
///   if result.get_type() == qtype::ERROR{
///     println!("FYI: {}", result.get_error_string().unwrap());
///     // Decrement reference count of the error object which is no longer used.
///     decrement_reference_count(result);
///     KNULL
///   }
///   else{
///     result
///   }
/// }
/// ```
/// ```q
/// q)chill: `libapi_examples 2: (`no_panick; 2);
/// q)chill[$; ("J"; "42")]
/// success!
/// 42
/// q)chill[+; (1; `a)]
/// FYI: type
/// ```
#[inline]
pub fn error_to_string(error: K) -> K{
  unsafe{
    native::ee(error)
  }
}

//%% Symbol %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Intern `n` chars from a char array.
///  Returns an interned char array and should be used to add char array to a symbol vector.
/// # Example
/// See the example of [`flip`](fn.flip.html).
#[inline]
pub fn internalize_n(string: S, n: I) -> S{
  unsafe{
    native::sn(string, n)
  }
}

/// Intern a null-terminated char array.
///  Returns an interned char array and should be used to add char array to a symbol vector.
/// # Example
/// See the example of [`flip`](fn.flip.html).
#[inline]
pub fn internalize(string: S) -> S{
  unsafe{
    native::ss(string)
  }
}

//%% Table %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Constructor of q table object from a q dictionary object.
/// # Note
/// Basically this is a `flip` command of q. Hence the value of the dictionary must have
///  lists as its elements.
/// ```no_run
/// #[macro_use]
/// extern crate kdbplus;
/// use kdbplus::api::*;
/// use kdbplus::qtype;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_table2(_: K) -> K{
///   // Build keys
///   let keys=new_simple_list(qtype::SYMBOL_LIST, 2);
///   let keys_slice=keys.as_mut_slice::<S>();
///   keys_slice[0]=internalize(str_to_S!("time"));
///   keys_slice[1]=internalize_n(str_to_S!("temperature_and_humidity"), 11);
///   
///   // Build values
///   let values=new_simple_list(qtype::COMPOUND_LIST, 2);
///   let time=new_simple_list(qtype::TIMESTAMP_LIST, 3);
///   // 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392
///   time.as_mut_slice::<J>().copy_from_slice(&[119067859167018272_i64, 201766609419710368, 271897944018691392]);
///   let temperature=new_simple_list(qtype::FLOAT_LIST, 3);
///   temperature.as_mut_slice::<F>().copy_from_slice(&[22.1_f64, 24.7, 30.5]);
///   values.as_mut_slice::<K>().copy_from_slice(&[time, temperature]);
///   
///   flip(new_dictionary(keys, values))
/// }
/// ```
/// ```q
/// q)climate_change: libc_api_examples 2: (`create_table2; 1);
/// q)climate_change[]
/// time                          temperature
/// -----------------------------------------
/// 2003.10.10D02:24:19.167018272 22.1       
/// 2006.05.24D06:16:49.419710368 24.7       
/// 2008.08.12D23:12:24.018691392 30.5    
/// ```
#[inline]
pub fn flip(dictionary: K) -> K{
  match unsafe{(*dictionary).qtype}{
    qtype::DICTIONARY => unsafe{native::xT(dictionary)},
    _ => unsafe{native::krr(null_terminated_str_to_const_S("not a dictionary\0"))}
  }
}

/// Constructor of simple q table object from a q keyed table object.
/// # Example
/// ```no_run
/// #[macro_use]
/// extern crate kdbplus;
/// use kdbplus::api::*;
/// use kdbplus::qtype;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_table2(_: K) -> K{
///   // Build keys
///   let keys=new_simple_list(qtype::SYMBOL_LIST, 2);
///   let keys_slice=keys.as_mut_slice::<S>();
///   keys_slice[0]=internalize(str_to_S!("time"));
///   keys_slice[1]=internalize_n(str_to_S!("temperature_and_humidity"), 11);
///   
///   // Build values
///   let values=new_simple_list(qtype::COMPOUND_LIST, 2);
///   let time=new_simple_list(qtype::TIMESTAMP_LIST, 3);
///   // 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392
///   time.as_mut_slice::<J>().copy_from_slice(&[119067859167018272_i64, 201766609419710368, 271897944018691392]);
///   let temperature=new_simple_list(qtype::FLOAT_LIST, 3);
///   temperature.as_mut_slice::<F>().copy_from_slice(&[22.1_f64, 24.7, 30.5]);
///   values.as_mut_slice::<K>().copy_from_slice(&[time, temperature]);
///   
///   flip(new_dictionary(keys, values))
/// }
/// 
/// #[no_mangle]
/// pub extern "C" fn create_keyed_table(dummy: K) -> K{
///   enkey(create_table2(dummy), 1)
/// }
/// 
/// #[no_mangle]
/// pub extern "C" fn keyed_to_simple_table(dummy: K) -> K{
///   unkey(create_keyed_table(dummy))
/// }
/// ```
/// ```q
/// q)unkey: libc_api_examples 2: (`keyed_to_simple_table; 1);
/// q)unkey[]
/// time                          temperature
/// -----------------------------------------
/// 2003.10.10D02:24:19.167018272 22.1       
/// 2006.05.24D06:16:49.419710368 24.7       
/// 2008.08.12D23:12:24.018691392 30.5    
/// ```
#[inline]
pub fn unkey(keyed_table: K) -> K{
  match unsafe{(*keyed_table).qtype}{
    qtype::DICTIONARY => unsafe{native::ktd(keyed_table)},
    _ => unsafe{native::krr(null_terminated_str_to_const_S("not a keyed table\0"))}
  }
}

/// Constructor of q keyed table object.
/// # Parameters
/// - `table`: q table object to be enkeyed.
/// - `n`: The number of key columns from the left.
/// # Example
/// ```no_run
/// #[macro_use]
/// extern crate kdbplus;
/// use kdbplus::api::*;
/// use kdbplus::qtype;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_table2(_: K) -> K{
///   // Build keys
///   let keys=new_simple_list(qtype::SYMBOL_LIST, 2);
///   let keys_slice=keys.as_mut_slice::<S>();
///   keys_slice[0]=internalize(str_to_S!("time"));
///   keys_slice[1]=internalize_n(str_to_S!("temperature_and_humidity"), 11);
///   
///   // Build values
///   let values=new_simple_list(qtype::COMPOUND_LIST, 2);
///   let time=new_simple_list(qtype::TIMESTAMP_LIST, 3);
///   // 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392
///   time.as_mut_slice::<J>().copy_from_slice(&[119067859167018272_i64, 201766609419710368, 271897944018691392]);
///   let temperature=new_simple_list(qtype::FLOAT_LIST, 3);
///   temperature.as_mut_slice::<F>().copy_from_slice(&[22.1_f64, 24.7, 30.5]);
///   values.as_mut_slice::<K>().copy_from_slice(&[time, temperature]);
///   
///   flip(new_dictionary(keys, values))
/// }
/// 
/// #[no_mangle]
/// pub extern "C" fn create_keyed_table(dummy: K) -> K{
///   enkey(create_table2(dummy), 1)
/// }
/// ```
/// ```q
/// q)locker: libc_api_examples 2: (`create_keyed_table; 1);
/// q)locker[]
/// time                         | temperature
/// -----------------------------| -----------
/// 2003.10.10D02:24:19.167018272| 22.1       
/// 2006.05.24D06:16:49.419710368| 24.7       
/// 2008.08.12D23:12:24.018691392| 30.5  
/// ```
#[inline]
pub fn enkey(table: K, n: J) -> K{ 
  match unsafe{(*table).qtype}{
    qtype::TABLE => unsafe{native::knt(n, table)},
    _ => unsafe{native::krr(null_terminated_str_to_const_S("not a table\0"))}
  }
}

//%% Reference Count %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Decrement reference count of the q object. The decrement must be done when `k` function gets an error
///  object whose type is `qtype::ERROR` and when you created an object but do not intend to return it to
///  q side. See details on [the reference page](https://code.kx.com/q/interfaces/c-client-for-q/#managing-memory-and-reference-counting).
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn agriculture(_: K)->K{
///   // Produce an apple.
///   let fruit=new_symbol("apple");
///   // Sow the apple seed.
///   decrement_reference_count(fruit);
///   // Return null.
///   KNULL
/// }
/// ```
/// ```q
/// q)do_something: `libapi_examples 2: (`agriculture; 1);
/// q)do_something[]
/// q)
/// ```
#[inline]
pub fn decrement_reference_count(qobject: K) -> V{
  unsafe{native::r0(qobject)}
}

/// Increment reference count of the q object. Increment must be done when you passed arguments
///  to Rust function and intends to return it to q side or when you pass some `K` objects to `k`
///  function and intend to use the argument after the call.
///  See details on [the reference page](https://code.kx.com/q/interfaces/c-client-for-q/#managing-memory-and-reference-counting).
/// # Example
/// ```no_run
/// #[macro_use]
/// extern crate kdbplus;
/// use kdbplus::api::*;
/// 
/// fn eat(apple: K){
///   println!("おいしい！");
/// }
/// 
/// #[no_mangle]
/// pub extern "C" fn satisfy_5000_men(apple: K) -> K{
///   for _ in 0..10{
///     eat(apple);
///   }
///   unsafe{native::k(0, str_to_S!("eat"), increment_reference_count(apple), KNULL);}
///   increment_reference_count(apple)  
/// }
/// ```
/// ```q
/// q)eat:{[apple] show "Collect the clutter of apples!";}
/// q)bread_is_a_sermon: libc_api_examples 2: (`satisfy_5000_men; 1);
/// q)bread_is_a_sermon[`green_apple]
/// おいしい！
/// おいしい！
/// おいしい！
/// おいしい！
/// おいしい！
/// おいしい！
/// おいしい！
/// おいしい！
/// おいしい！
/// おいしい！
/// "Collect the clutter of apples!"
/// ```
#[inline]
pub fn increment_reference_count(qobject: K) -> K{
  unsafe{native::r1(qobject)}
}

//%% Callback %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Remove callback from the associated kdb+ socket and call `kclose`.
///  Return null if the socket is invalid or not the one which had been registered by `sd1`.
/// # Note
/// A function which calls this function must be executed at the exit of the process.
#[inline]
pub fn destroy_socket(socket: I){
  unsafe{
    native::sd0(socket);
  }
}

/// Remove callback from the associated kdb+ socket and call `kclose` if the given condition is satisfied.
///  Return null if the socket is invalid or not the one which had been registered by `sd1`.
/// # Note
/// A function which calls this function must be executed at the exit of the process.
#[inline]
pub fn destroy_socket_if(socket: I, condition: bool){
  unsafe{
    native::sd0x(socket, condition as I);
  }
}

/// Register callback to the associated kdb+ socket.
/// ```no_run
/// #[macro_use]
/// extern crate kdbplus;
/// use kdbplus::api::*;
/// use kdbplus::qtype;
/// 
/// static mut PIPE:[I; 2]=[-1, -1];
///
/// // Callback for some message queue.
/// extern "C" fn callback(socket: I)->K{
///   let mut buffer: [K; 1]=[0 as K];
///   unsafe{libc::read(socket, buffer.as_mut_ptr() as *mut V, 8)};
///   // Call `shout` function on q side with the received data.
///   let result=error_to_string(unsafe{native::k(0, str_to_S!("shout"), buffer[0], KNULL)});
///   if result.get_type() == qtype::ERROR{
///     eprintln!("Execution error: {}", result.get_symbol().unwrap());
///     decrement_reference_count(result);
///   };
///   KNULL
/// }
/// 
/// #[no_mangle]
/// pub extern "C" fn plumber(_: K) -> K{
///   if 0 != unsafe{libc::pipe(PIPE.as_mut_ptr())}{
///     return new_error("Failed to create pipe\0");
///   }
///   if KNULL == register_callback(unsafe{PIPE[0]}, callback){
///     return new_error("Failed to register callback\0");
///   }
///   // Lock symbol in a worker thread.
///   pin_symbol();
///   let handle=std::thread::spawn(move ||{
///     let mut precious=new_simple_list(qtype::SYMBOL_LIST, 3);
///     let precious_array=precious.as_mut_slice::<S>();
///     precious_array[0]=internalize(null_terminated_str_to_S("belief\0"));
///     precious_array[1]=internalize(null_terminated_str_to_S("love\0"));
///     precious_array[2]=internalize(null_terminated_str_to_S("hope\0"));
///     unsafe{libc::write(PIPE[1], std::mem::transmute::<*mut K, *mut V>(&mut precious), 8)};
///   });
///   handle.join().unwrap();
///   unpin_symbol();
///   KNULL
/// }
/// ```
/// ```q
/// q)shout:{[precious] -1 "What are the three largest elements?: ", .Q.s1 precious;};
/// q)fall_into_pipe: `libc_api_example 2: (`plumber; 1);
/// q)fall_into_pipe[]
/// What are the three largest elements?: `belief`love`hope
/// ```
#[inline]
pub fn register_callback(socket: I, function: extern fn(I) -> K) -> K{
  unsafe{
    native::sd1(socket, function)
  }
}

//%% Miscellaneous %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Apply a function to q list object `.[func; args]`.
/// # Example
/// See the example of [`error_to_string`](fn.error_to_string.html).
#[inline]
pub fn apply(func: K, args: K) -> K{
  unsafe{native::dot(func, args)}
}

/// Lock a location of internalized symbol in remote threads.
///  Returns the previously set value.
/// # Example
/// See the example of [`register_callback`](fn.register_callback.html).
#[inline]
pub fn pin_symbol() -> I{
  unsafe{
    native::setm(1)
  }
}

/// Unlock a location of internalized symbol in remote threads. 
/// # Example
/// See the example of [`register_callback`](fn.register_callback.html).
#[inline]
pub fn unpin_symbol() -> I{
  unsafe{
    native::setm(0)
  }
}

/// Drop Rust object inside q. Passed as the first element of a foreign object.
/// # Parameters
/// - `obj`: List of (function to free the object; foreign object).
/// # Example
/// See the example of [`load_as_q_function`](fn.load_as_q_function.html).
pub extern "C" fn drop_q_object(obj: K) -> K{
  let obj_slice=obj.as_mut_slice::<K>();
  // Take ownership of `K` object from a raw pointer and drop at the end of this scope.
  unsafe{Box::from_raw(obj_slice[1])};
  // Fill the list with null.
  obj_slice.copy_from_slice(&[KNULL, KNULL]);
  obj
}

/// Load C function as a q function (`K` object).
/// # Parameters
/// - `func`: A function takes a C function that would take `n` `K` objects as arguments and returns a `K` object.
/// - `n`: The number of arguments for the function.
/// # Example
/// ```no_run
/// #[macro_use]
/// extern crate kdbplus;
/// use kdbplus::api::*;
/// use kdbplus::qtype;
/// 
/// #[derive(Clone, Debug)]
/// struct Planet{
///   name: String,
///   population: i64,
///   water: bool
/// }
/// 
/// impl Planet{
///   /// Constructor of `Planet`.
///   fn new(name: &str, population: i64, water: bool) -> Self{
///     Planet{
///       name: name.to_string(),
///       population: population,
///       water: water
///     }
///   }
/// 
///   /// Description of the planet.
///   fn description(&self)->String{
///     let mut desc=format!("The planet {} is a beautiful planet where {} people reside.", self.name, self.population);
///     if self.water{
///       desc+=" Furthermore water is flowing on the surface of it.";
///     }
///     desc
///   }
/// }
/// 
/// /// Example of `set_type`.
/// #[no_mangle]
/// pub extern "C" fn eden(_: K) -> K{
///   let earth=Planet::new("earth", 7500_000_000, true);
///   let mut foreign=new_simple_list(qtype::COMPOUND_LIST, 2);
///   let foreign_slice=foreign.as_mut_slice::<K>();
///   foreign_slice[0]=drop_q_object as K;
///   foreign_slice[1]=Box::into_raw(Box::new(earth)) as K;
///   // Set as foreign object.
///   foreign.set_type(qtype::FOREIGN);
///   foreign
/// }
/// 
/// extern "C" fn invade(planet: K, action: K) -> K{
///   let obj=planet.as_mut_slice::<K>()[1] as *const Planet;
///   println!("{:?}", unsafe{obj.as_ref()}.unwrap());
///   let mut desc=unsafe{obj.as_ref()}.unwrap().description();
///   if action.get_bool().unwrap(){
///     desc+=" You shall not curse what God blessed.";
///   }
///   else{
///     desc+=" I perceived I could find favor of God by blessing them.";
///   }
///   new_string(&desc)
/// }
/// 
/// /// Example of `load_as_q_function`.
/// #[no_mangle]
/// pub extern "C" fn probe(planet: K)->K{
///   // Return monadic function
///   unsafe{native::k(0, str_to_S!("{[func; planet] func[planet]}"), load_as_q_function(invade as *const V, 2), planet, KNULL)}
/// }
/// ```
/// ```q
/// q)eden: libc_api_example 2: (`eden; 1);
/// q)earth: eden[]
/// q)type earth
/// 112h
/// q)probe: libc_api_example 2: (`probe; 1);
/// q)invade: probe[earth];
/// q)\c 25 200
/// q)invade 1b
/// "The planet earth is a beautiful planet where 7500000000 people reside. Furthermore water is flowing on the surface of it. You shall not curse what God blessed."
/// ```
#[inline]
pub fn load_as_q_function(func: *const V, n: J) -> K{
  unsafe{
    native::dl(func, n)
  }
}

/// Convert ymd to the number of days from `2000.01.01`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// fn main(){
/// 
///   let days=ymd_to_days(2020, 4, 1);
///   assert_eq!(days, 7396);
/// 
/// }
/// ```
#[inline]
pub fn ymd_to_days(year: I, month: I, date:I) -> I{
  unsafe{
    native::ymd(year, month, date)
  }
}

/// Convert the number of days from `2000.01.01` to a number expressed as `yyyymmdd`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// 
/// fn main(){
/// 
///   let number=days_to_ymd(7396);
///   assert_eq!(number, 20200401);
/// 
/// }
/// ```
#[inline]
pub fn days_to_ymd(days: I) -> I{
  unsafe{
    native::dj(days)
  }
}

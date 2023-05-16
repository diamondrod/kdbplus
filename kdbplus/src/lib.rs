//! As Rust is becoming a popular programming language for its performance and type safety, the desire to use
//!  it with still a maniac time-series database kdb+ is brewing. The aspiration is understandable since we know
//!  kdb+ is fast and its interface or a shared library should be fast as well. This interface was created to
//!  satisfy such a natural demand, furthermore, in a manner users do not feel any pain to use. The notrious
//!  ethoteric function names of the q/kdb+ C API is not an interest of Rust developers.
//!
//! *"Give us a **Rust** interface!!"*
//!
//! Here is your choice.
//!
//! This interface provides two features:
//!
//! - IPC interface (Rust client of q/kdb+ process)
//! - API (build a shared library for q/kdb+)
//!
//! You can find detail descriptions of each feature under corresponding module page.

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Global Variables
//++++++++++++++++++++++++++++++++++++++++++++++++++//

pub mod qtype {
    //! This module provides a list of q types. The motivation to contain them in a module is to
    //!  tie them up as related items rather than scattered values. Hence user should use these
    //!  indicators with `qtype::` prefix, e.g., `qtype::BOOL_LIST`.
    //! # Note
    //! In order to facilitate type check without overflow this module defines atom type indicator
    //!  as well as list type indicators (We don't need to compeletely mirror the C API).

    use libc::c_schar;

    /// Type indicator of q error
    pub const ERROR: c_schar = -128;
    /// Type indicator of q enum atom.
    pub const ENUM_ATOM: c_schar = -20;
    /// Type indicator of q time atom.
    pub const TIME_ATOM: c_schar = -19;
    /// Type indicator of q second atom.
    pub const SECOND_ATOM: c_schar = -18;
    /// Type indicator of q minute atom.
    pub const MINUTE_ATOM: c_schar = -17;
    /// Type indicator of q timespan atom.
    pub const TIMESPAN_ATOM: c_schar = -16;
    /// Type indicator of q datetime atom.
    pub const DATETIME_ATOM: c_schar = -15;
    /// Type indicator of q date atom.
    pub const DATE_ATOM: c_schar = -14;
    /// Type indicator of q month atom.
    pub const MONTH_ATOM: c_schar = -13;
    /// Type indicator of q timestamp atom.
    pub const TIMESTAMP_ATOM: c_schar = -12;
    /// Type indicator of q symbol atom.
    pub const SYMBOL_ATOM: c_schar = -11;
    /// Type indicator of q char atom.
    pub const CHAR: c_schar = -10;
    /// Type indicator of q float atom.
    pub const FLOAT_ATOM: c_schar = -9;
    /// Type indicator of q real atom.
    pub const REAL_ATOM: c_schar = -8;
    /// Type indicator of q long atom.
    pub const LONG_ATOM: c_schar = -7;
    /// Type indicator of q int atom.
    pub const INT_ATOM: c_schar = -6;
    /// Type indicator of q short atom.
    pub const SHORT_ATOM: c_schar = -5;
    /// Type indicator of q byte atom.
    pub const BYTE_ATOM: c_schar = -4;
    /// Type indicator of q GUID atom.
    pub const GUID_ATOM: c_schar = -2;
    /// Type indicator of q bool atom.
    pub const BOOL_ATOM: c_schar = -1;
    /// Type indicator of q mixed list list. Slice access type: `K`, i.e., `obj.as_mut_sice::<K>()`.
    pub const COMPOUND_LIST: c_schar = 0;
    /// Type indicator of q bool list list. Slice access type: `G`, i.e., `obj.as_mut_sice::<G>()`.
    pub const BOOL_LIST: c_schar = 1;
    /// Type indicator of q GUID list. Slice access type: `U`, i.e., `obj.as_mut_sice::<U>()`.
    pub const GUID_LIST: c_schar = 2;
    /// Type indicator of q byte list. Slice access type: `G`, i.e., `obj.as_mut_sice::<G>()`.
    pub const BYTE_LIST: c_schar = 4;
    /// Type indicator of q short list. Slice access type: `H`, i.e., `obj.as_mut_sice::<H>()`.
    pub const SHORT_LIST: c_schar = 5;
    /// Type indicator of q int list. Slice access type: `I`, i.e., `obj.as_mut_sice::<I>()`.
    pub const INT_LIST: c_schar = 6;
    /// Type indicator of q long list. Slice access type: `J`, i.e., `obj.as_mut_sice::<J>()`.
    pub const LONG_LIST: c_schar = 7;
    /// Type indicator of q real list. Slice access type: `E`, i.e., `obj.as_mut_sice::<E>()`.
    pub const REAL_LIST: c_schar = 8;
    /// Type indicator of q float list. Slice access type: `F`, i.e., `obj.as_mut_sice::<F>()`.
    pub const FLOAT_LIST: c_schar = 9;
    /// Type indicator of q string (char list). Slice access type: `C`, i.e., `obj.as_mut_sice::<C>()`.
    pub const STRING: c_schar = 10;
    /// Type indicator of q symbol list. Slice access type: `S`, i.e., `obj.as_mut_sice::<S>()`.
    pub const SYMBOL_LIST: c_schar = 11;
    /// Type indicator of q timestamp list. Slice access type: `J`, i.e., `obj.as_mut_sice::<J>()`.
    pub const TIMESTAMP_LIST: c_schar = 12;
    /// Type indicator of q month list. Slice access type: `I`, i.e., `obj.as_mut_sice::<I>()`.
    pub const MONTH_LIST: c_schar = 13;
    /// Type indicator of q date list. Slice access type: `I`, i.e., `obj.as_mut_sice::<I>()`.
    pub const DATE_LIST: c_schar = 14;
    /// Type indicator of q datetime list. Slice access type: `F`, i.e., `obj.as_mut_sice::<F>()`.
    pub const DATETIME_LIST: c_schar = 15;
    /// Type indicator of q timespan list. Slice access type: `J`, i.e., `obj.as_mut_sice::<J>()`.
    pub const TIMESPAN_LIST: c_schar = 16;
    /// Type indicator of q minute list. Slice access type: `I`, i.e., `obj.as_mut_sice::<I>()`.
    pub const MINUTE_LIST: c_schar = 17;
    /// Type indicator of q second list. Slice access type: `I`, i.e., `obj.as_mut_sice::<I>()`.
    pub const SECOND_LIST: c_schar = 18;
    /// Type indicator of q time list. Slice access type: `I`, i.e., `obj.as_mut_sice::<I>()`.
    pub const TIME_LIST: c_schar = 19;
    /// Type indicator of q enum list. Slice access type: `J`, i.e., `obj.as_mut_sice::<J>()`.
    pub const ENUM_LIST: c_schar = 20;
    /// Type indicator of q table.
    pub const TABLE: c_schar = 98;
    /// Type indicator of q dictionary. Slice access type: `K`, i.e., `obj.as_mut_sice::<K>()`.
    /// - `obj.as_mut_sice::<K>()[0]`: keys
    /// - `obj.as_mut_sice::<K>()[1]`: values
    pub const DICTIONARY: c_schar = 99;
    /// Type indicator of q general null
    pub const NULL: c_schar = 101;
    /// Type indicator of q foreign object.
    pub const FOREIGN: c_schar = 112;
    /// Type indicator of q sorted dictionary. Slice access type: `K`, i.e., `obj.as_mut_sice::<K>()`.
    /// - `obj.as_mut_sice::<K>()[0]`: keys
    /// - `obj.as_mut_sice::<K>()[1]`: values
    pub const SORTED_DICTIONARY: c_schar = 127;
}

pub mod qattribute {
    //! This module provides a list of q attributes. The motivation to contain them in a module is to
    //!  tie them up as related items rather than scattered values. Hence user should use these
    //!  indicators with `qattribute::` prefix, e.g., `qattribute::UNIQUE`.

    use std::os::raw::c_char;
    /// Indicates no attribute is appended on the q object.
    pub const NONE: c_char = 0;
    /// Sorted attribute, meaning that the q list is sorted in ascending order.
    pub const SORTED: c_char = 1;
    /// Unique attribute, meaning that each element in the q list has a unique value within the list.
    pub const UNIQUE: c_char = 2;
    /// Parted attribute, meaning that all the elements with the same value in the q object appear in a chunk.
    pub const PARTED: c_char = 3;
    /// Grouped attribute, meaning that the elements of the q list are grouped with their indices by values implicitly.
    pub const GROUPED: c_char = 4;
}

pub mod qnull_base {
    //! This module provides a list of underlying null values of q objects. The motivation to contain
    //!  them in a module is to tie them up as related items rather than scattered values. Hence user
    //!  should use these indicators with `qnull::` prefix, e.g., `qnull_base::F`.
    //!
    //! These values are mainly used to construct `K` object for `api` module but underlying values are
    //!  same for `ipc` module for simple types. For `ipc` module, proper null values of each type are
    //!  provided under [`qnull`](../ipc/qnull/index.html) namespace.

    use libc::{c_double, c_float, c_int, c_longlong, c_short, c_uchar};

    /// Null value of GUID.
    #[cfg_attr(feature = "api", doc = r##"# Example"##)]
    #[cfg_attr(feature = "api", doc = r##"```no_run"##)]
    #[cfg_attr(feature = "api", doc = r##"use kdbplus::*;"##)]
    #[cfg_attr(feature = "api", doc = r##"use kdbplus::api::*;"##)]
    #[cfg_attr(feature = "api", doc = r##""##)]
    #[cfg_attr(feature = "api", doc = r##"#[no_mangle]"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"pub extern "C" fn guid_border(_: K) -> K{"##
    )]
    #[cfg_attr(feature = "api", doc = r##"  new_guid(qnull_base::U)"##)]
    #[cfg_attr(feature = "api", doc = r##"}"##)]
    #[cfg_attr(feature = "api", doc = r##"```"##)]
    #[cfg_attr(feature = "api", doc = r##"```q"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"q)guid_border: `libapi_examples 2: (`guid_border; 1);"##
    )]
    #[cfg_attr(feature = "api", doc = r##"q)guid_border[]"##)]
    #[cfg_attr(feature = "api", doc = r##"0Ng"##)]
    #[cfg_attr(feature = "api", doc = r##"```"##)]
    pub const U: [c_uchar; 16] = [0; 16];

    /// Null value of short.
    #[cfg_attr(feature = "api", doc = r##"# Example"##)]
    #[cfg_attr(feature = "api", doc = r##"```no_run"##)]
    #[cfg_attr(feature = "api", doc = r##"use kdbplus::*;"##)]
    #[cfg_attr(feature = "api", doc = r##"use kdbplus::api::*;"##)]
    #[cfg_attr(feature = "api", doc = r##""##)]
    #[cfg_attr(feature = "api", doc = r##"#[no_mangle]"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"pub extern "C" fn short_borders(_: K) -> K{"##
    )]
    #[cfg_attr(
        feature = "api",
        doc = r##"  let shorts=new_list(qtype::SHORT_LIST, 3);"##
    )]
    #[cfg_attr(
        feature = "api",
        doc = r##"  let shorts_slice=unsafe{shorts.as_mut_slice::<H>()};"##
    )]
    #[cfg_attr(feature = "api", doc = r##"  shorts_slice[0]=qnull_base::H;"##)]
    #[cfg_attr(feature = "api", doc = r##"  shorts_slice[1]=qinf_base::H;"##)]
    #[cfg_attr(feature = "api", doc = r##"  shorts_slice[2]=qninf_base::H;"##)]
    #[cfg_attr(feature = "api", doc = r##"  shorts"##)]
    #[cfg_attr(feature = "api", doc = r##"}"##)]
    #[cfg_attr(feature = "api", doc = r##"```"##)]
    #[cfg_attr(feature = "api", doc = r##"```q"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"q)short_borders: `libapi_examples 2: (`short_borders; 1);"##
    )]
    #[cfg_attr(feature = "api", doc = r##"q)short_borders[]"##)]
    #[cfg_attr(feature = "api", doc = r##"0N 0W -0Wh"##)]
    #[cfg_attr(feature = "api", doc = r##"```"##)]
    pub const H: c_short = c_short::MIN;

    /// Null value of int family, i.e., int, month, date, minute, second and time.
    #[cfg_attr(feature = "api", doc = r##"# Example"##)]
    #[cfg_attr(feature = "api", doc = r##"```no_run"##)]
    #[cfg_attr(feature = "api", doc = r##"use kdbplus::*;"##)]
    #[cfg_attr(feature = "api", doc = r##"use kdbplus::api::*;"##)]
    #[cfg_attr(feature = "api", doc = r##""##)]
    #[cfg_attr(feature = "api", doc = r##"#[no_mangle]"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"pub extern "C" fn int_borders(_: K) -> K{"##
    )]
    #[cfg_attr(feature = "api", doc = r##"  let ints=new_list(qtype::INT_LIST, 3);"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"  let ints_slice=unsafe { ints.as_mut_slice::<I>() };"##
    )]
    #[cfg_attr(feature = "api", doc = r##"  ints_slice[0]=qnull_base::I;"##)]
    #[cfg_attr(feature = "api", doc = r##"  ints_slice[1]=qinf_base::I;"##)]
    #[cfg_attr(feature = "api", doc = r##"  ints_slice[2]=qninf_base::I;"##)]
    #[cfg_attr(feature = "api", doc = r##"  ints"##)]
    #[cfg_attr(feature = "api", doc = r##"}"##)]
    #[cfg_attr(feature = "api", doc = r##"```"##)]
    #[cfg_attr(feature = "api", doc = r##"```q"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"q)int_borders: `libapi_examples 2: (`int_borders; 1);"##
    )]
    #[cfg_attr(feature = "api", doc = r##"q)int_borders[]"##)]
    #[cfg_attr(feature = "api", doc = r##"0N 0W -0Wi"##)]
    #[cfg_attr(feature = "api", doc = r##"```"##)]
    pub const I: c_int = c_int::MIN;

    /// Null value of long family, i.e., long, timestamp and timespan.
    #[cfg_attr(
        feature = "api",
        doc = r##"
    #[cfg_attr( feature = "api", doc = r##"# Example"##
    )]
    #[cfg_attr(feature = "api", doc = r##"```no_run"##)]
    #[cfg_attr(feature = "api", doc = r##"use kdbplus::*;"##)]
    #[cfg_attr(feature = "api", doc = r##"use kdbplus::api::*;"##)]
    #[cfg_attr(feature = "api", doc = r##""##)]
    #[cfg_attr(feature = "api", doc = r##"#[no_mangle]"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"pub extern "C" fn long_borders(_: K) -> K{"##
    )]
    #[cfg_attr(
        feature = "api",
        doc = r##"  let timestamps=new_list(qtype::TIMESTAMP_LIST, 3);"##
    )]
    #[cfg_attr(
        feature = "api",
        doc = r##"  let timestamps_slice=unsafe{timestamps.as_mut_slice::<J>()};"##
    )]
    #[cfg_attr(feature = "api", doc = r##"  timestamps_slice[0]=qnull_base::J;"##)]
    #[cfg_attr(feature = "api", doc = r##"  timestamps_slice[1]=qinf_base::J;"##)]
    #[cfg_attr(feature = "api", doc = r##"  timestamps_slice[2]=qninf_base::J;"##)]
    #[cfg_attr(feature = "api", doc = r##"  timestamps"##)]
    #[cfg_attr(feature = "api", doc = r##"}"##)]
    #[cfg_attr(feature = "api", doc = r##"```"##)]
    #[cfg_attr(feature = "api", doc = r##"```q"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"q)timestamp_borders: `libapi_examples 2: (`long_borders; 1);"##
    )]
    #[cfg_attr(feature = "api", doc = r##"q)timestamp_borders[]"##)]
    #[cfg_attr(feature = "api", doc = r##"0N 0W -0Wp"##)]
    #[cfg_attr(feature = "api", doc = r##"```"##)]
    pub const J: c_longlong = c_longlong::MIN;

    /// Null value of real.
    #[cfg_attr(feature = "api", doc = r##"# Example"##)]
    #[cfg_attr(feature = "api", doc = r##"```no_run"##)]
    #[cfg_attr(feature = "api", doc = r##"use kdbplus::*;"##)]
    #[cfg_attr(feature = "api", doc = r##"use kdbplus::api::*;"##)]
    #[cfg_attr(feature = "api", doc = r##""##)]
    #[cfg_attr(feature = "api", doc = r##"#[no_mangle]"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"pub extern "C" fn real_borders(_: K) -> K{"##
    )]
    #[cfg_attr(
        feature = "api",
        doc = r##"  let reals=new_list(qtype::REAL_LIST, 3);"##
    )]
    #[cfg_attr(
        feature = "api",
        doc = r##"  let reals_slice=unsafe{reals.as_mut_slice::<E>()};"##
    )]
    #[cfg_attr(feature = "api", doc = r##"  reals_slice[0]=qnull_base::E;"##)]
    #[cfg_attr(feature = "api", doc = r##"  reals_slice[1]=qinf_base::E;"##)]
    #[cfg_attr(feature = "api", doc = r##"  reals_slice[2]=qninf_base::E;"##)]
    #[cfg_attr(feature = "api", doc = r##"  reals"##)]
    #[cfg_attr(feature = "api", doc = r##"}"##)]
    #[cfg_attr(feature = "api", doc = r##"```"##)]
    #[cfg_attr(feature = "api", doc = r##"```q"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"q)real_borders: `libapi_examples 2: (`real_borders; 1);"##
    )]
    #[cfg_attr(feature = "api", doc = r##"q)real_borders[]"##)]
    #[cfg_attr(feature = "api", doc = r##"0N 0W -0We"##)]
    #[cfg_attr(feature = "api", doc = r##"```"##)]
    pub const E: c_float = c_float::NAN;

    /// Null value of float family, i.e., float and datetime.
    #[cfg_attr(feature = "api", doc = r##"# Example"##)]
    #[cfg_attr(feature = "api", doc = r##"```no_run"##)]
    #[cfg_attr(feature = "api", doc = r##"use kdbplus::*;"##)]
    #[cfg_attr(feature = "api", doc = r##"use kdbplus::api::*;"##)]
    #[cfg_attr(feature = "api", doc = r##""##)]
    #[cfg_attr(feature = "api", doc = r##"#[no_mangle]"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"pub extern "C" fn float_borders(_: K) -> K{"##
    )]
    #[cfg_attr(
        feature = "api",
        doc = r##"  let datetimes=new_list(qtype::DATETIME_LIST, 3);"##
    )]
    #[cfg_attr(
        feature = "api",
        doc = r##"  let datetimes_slice=unsafe{datetimes.as_mut_slice::<F>()};"##
    )]
    #[cfg_attr(feature = "api", doc = r##"  datetimes_slice[0]=qnull_base::F;"##)]
    #[cfg_attr(feature = "api", doc = r##"  datetimes_slice[1]=qinf_base::F;"##)]
    #[cfg_attr(feature = "api", doc = r##"  datetimes_slice[2]=qninf_base::F;"##)]
    #[cfg_attr(feature = "api", doc = r##"  datetimes"##)]
    #[cfg_attr(feature = "api", doc = r##"}"##)]
    #[cfg_attr(feature = "api", doc = r##"```"##)]
    #[cfg_attr(feature = "api", doc = r##"```q"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"q)datetime_borders: `libapi_examples 2: (`float_borders; 1);"##
    )]
    #[cfg_attr(feature = "api", doc = r##"q)datetime_borders[]"##)]
    #[cfg_attr(feature = "api", doc = r##"0N 0W -0Wz"##)]
    #[cfg_attr(feature = "api", doc = r##"```"##)]
    pub const F: c_double = c_double::NAN;

    /// Null value of char.
    #[cfg_attr(feature = "api", doc = r##"#Example"##)]
    #[cfg_attr(feature = "api", doc = r##"```no_run"##)]
    #[cfg_attr(feature = "api", doc = r##"use kdbplus::*;"##)]
    #[cfg_attr(feature = "api", doc = r##"use kdbplus::api::*;"##)]
    #[cfg_attr(feature = "api", doc = r##""##)]
    #[cfg_attr(feature = "api", doc = r##"#[no_mangle]"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"pub extern "C" fn char_borders(_: K) -> K{"##
    )]
    #[cfg_attr(feature = "api", doc = r##"  new_char(qnull_base::C)"##)]
    #[cfg_attr(feature = "api", doc = r##"}"##)]
    #[cfg_attr(feature = "api", doc = r##"```"##)]
    #[cfg_attr(feature = "api", doc = r##"```q"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"q)char_borders: `libapi_examples 2: (`char_borders; 1);"##
    )]
    #[cfg_attr(feature = "api", doc = r##"q)char_borders[]"##)]
    #[cfg_attr(feature = "api", doc = r##"" ""##)]
    #[cfg_attr(feature = "api", doc = r##"q)null char_borders[]"##)]
    #[cfg_attr(feature = "api", doc = r##"1b"##)]
    #[cfg_attr(feature = "api", doc = r##"```"##)]
    pub const C: char = ' ';

    /// Null value of string family (symbol, string).
    #[cfg_attr(feature = "api", doc = r##"# Example"##)]
    #[cfg_attr(feature = "api", doc = r##"```no_run"##)]
    #[cfg_attr(feature = "api", doc = r##"use kdbplus::*;"##)]
    #[cfg_attr(feature = "api", doc = r##"use kdbplus::api::*;"##)]
    #[cfg_attr(feature = "api", doc = r##""##)]
    #[cfg_attr(feature = "api", doc = r##"#[no_mangle]"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"pub extern "C" fn string_borders(_: K) -> K{"##
    )]
    #[cfg_attr(
        feature = "api",
        doc = r##"  let compound=new_list(qtype::COMPOUND_LIST, 2);"##
    )]
    #[cfg_attr(
        feature = "api",
        doc = r##"  let compound_slice=unsafe{compound.as_mut_slice::<K>()};"##
    )]
    #[cfg_attr(
        feature = "api",
        doc = r##"  compound_slice[0]=new_symbol(qnull_base::S);"##
    )]
    #[cfg_attr(
        feature = "api",
        doc = r##"  compound_slice[1]=new_string(qnull_base::S);"##
    )]
    #[cfg_attr(feature = "api", doc = r##"  compound"##)]
    #[cfg_attr(feature = "api", doc = r##"}"##)]
    #[cfg_attr(feature = "api", doc = r##"```"##)]
    #[cfg_attr(feature = "api", doc = r##"```q"##)]
    #[cfg_attr(
        feature = "api",
        doc = r##"q)string_borders: `libapi_examples 2: (`string_borders; 1);"##
    )]
    #[cfg_attr(feature = "api", doc = r##"q)string_borders[]"##)]
    #[cfg_attr(feature = "api", doc = r##"`"##)]
    #[cfg_attr(feature = "api", doc = r#""""#)]
    #[cfg_attr(feature = "api", doc = r##"q)null each string_borders[]"##)]
    #[cfg_attr(feature = "api", doc = r##"1b"##)]
    #[cfg_attr(feature = "api", doc = r##"`boolean$()"##)]
    #[cfg_attr(feature = "api", doc = r##"```"##)]
    pub const S: &str = "";
}

pub mod qinf_base {
    //! This module provides a list of q null values. The motivation to contain them in a module is to
    //!  tie them up as related items rather than scattered values. Hence user should use these
    //!  indicators with `qnull::` prefix, e.g., `qinf_base::J`.
    //!
    //! These values are mainly used to construct `K` object for `api` module but underlying values are
    //!  same for `ipc` module for simple types. For `ipc` module, proper infinity values of each type
    //!  are provided under [`qinf`](../ipc/qinf/index.html) namespace.

    use libc::{c_double, c_float, c_int, c_longlong, c_short};
    /// Infinity value of short.
    /// # Example
    /// See the example of [`qnull_base::H`](../qnull_base/constant.H.html).
    pub const H: c_short = c_short::MAX;
    /// Infinity value of int family, i.e., int, month, date, minute, second and time.
    /// # Example
    /// See the example of [`qnull_base::I`](../qnull_base/constant.I.html).
    pub const I: c_int = c_int::MAX;
    /// Infinity value of long family, i.e., long, timestamp and timespan.
    /// # Example
    /// See the example of [`qnull_base::J`](../qnull_base/constant.J.html).
    pub const J: c_longlong = c_longlong::MAX;
    /// Infinity value of real.
    /// # Example
    /// See the example of [`qnull_base::E`](../qnull_base/constant.E.html).
    pub const E: c_float = c_float::INFINITY;
    /// Infinity value of float family, i.e., float and datetime.
    /// # Example
    /// See the example of [`qnull_base::F`](../qnull_base/constant.F.html).
    pub const F: c_double = c_double::INFINITY;
}

pub mod qninf_base {
    //! This module provides a list of q null values. The motivation to contain them in a module is to
    //!  tie them up as related items rather than scattered values. Hence user should use these
    //!  indicators with `qnull::` prefix, e.g., `qninf_base::I`.
    //!
    //! These values are mainly used to construct `K` object for `api` module but underlying values are
    //!  same for `ipc` module for simple types. For `ipc` module, proper negative infinity values of
    //!  each type are provided under [`qninf`](../ipc/qninf/index.html) namespace.

    use libc::{c_double, c_float, c_int, c_longlong, c_short};
    /// Negative infinity value of short.
    /// # Example
    /// See the example of [`qnull_base::H`](../qnull_base/constant.H.html).
    pub const H: c_short = -c_short::MAX;
    /// Negative infinity value of int family, i.e., int, month, date, minute, second and time.
    /// # Example
    /// See the example of [`qnull_base::I`](../qnull_base/constant.I.html).
    pub const I: c_int = -c_int::MAX;
    /// Negative infinity value of long family, i.e., long, timestamp and timespan.
    /// # Example
    /// See the example of [`qnull_base::J`](../qnull_base/constant.J.html).
    pub const J: c_longlong = -c_longlong::MAX;
    /// Negative infinity value of real.
    /// # Example
    /// See the example of [`qnull_base::E`](../qnull_base/constant.E.html).
    pub const E: c_float = c_float::NEG_INFINITY;
    /// Negative infinity value of float family, i.e., float and datetime.
    /// # Example
    /// See the example of [`qnull_base::F`](../qnull_base/constant.F.html).
    pub const F: c_double = c_double::NEG_INFINITY;
}

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Export Modules
//++++++++++++++++++++++++++++++++++++++++++++++++++//

#[cfg(feature = "api")]
pub mod api;

#[cfg(feature = "ipc")]
pub mod ipc;

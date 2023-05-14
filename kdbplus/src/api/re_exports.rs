use crate::str_to_S;

use super::*;

//++++++++++++++++++++++++++++++++++++++++++++++++++//
// >> Re-export
//++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Constructor %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

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
pub fn new_bool(boolean: I) -> K {
    unsafe { native::kb(boolean) }
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
pub fn new_guid(guid: [G; 16]) -> K {
    unsafe { native::ku(U::new(guid)) }
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
pub fn new_byte(byte: I) -> K {
    unsafe { native::kg(byte) }
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
pub fn new_short(short: I) -> K {
    unsafe { native::kh(short) }
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
pub fn new_int(int: I) -> K {
    unsafe { native::ki(int) }
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
pub fn new_long(long: J) -> K {
    unsafe { native::kj(long) }
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
pub fn new_real(real: F) -> K {
    unsafe { native::ke(real) }
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
pub fn new_float(float: F) -> K {
    unsafe { native::kf(float) }
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
pub fn new_char(character: char) -> K {
    unsafe { native::kc(character as I) }
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
pub fn new_symbol(symbol: &str) -> K {
    unsafe { native::ks(str_to_S!(symbol)) }
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
pub fn new_timestamp(nanoseconds: J) -> K {
    unsafe { native::ktj(qtype::TIMESTAMP_ATOM as I, nanoseconds) }
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
pub fn new_month(months: I) -> K {
    unsafe {
        let month = native::ka(qtype::MONTH_ATOM as I);
        (*month).value.int = months;
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
pub fn new_date(days: I) -> K {
    unsafe { native::kd(days) }
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
pub fn new_datetime(days: F) -> K {
    unsafe { native::kz(days) }
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
pub fn new_timespan(nanoseconds: J) -> K {
    unsafe { native::ktj(qtype::TIMESPAN_ATOM as I, nanoseconds) }
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
pub fn new_minute(minutes: I) -> K {
    unsafe {
        let minute = native::ka(qtype::MINUTE_ATOM as I);
        (*minute).value.int = minutes;
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
pub fn new_second(seconds: I) -> K {
    unsafe {
        let second = native::ka(qtype::SECOND_ATOM as I);
        (*second).value.int = seconds;
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
pub fn new_time(milliseconds: I) -> K {
    unsafe { native::kt(milliseconds) }
}

/// Constructor of q enum object. This is a complememtal constructor of
///  missing second type.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
///
/// #[no_mangle]
/// pub extern "C" fn create_enum(source: K, index: K) -> K{
///   // Error if the specified enum source does not exist or it is not a symbol list or the index is out of enum range
///   new_enum(source.get_str().unwrap(), index.get_long().unwrap())
/// }
/// ```
/// ```q
/// q)enumerate: libc_api_examples 2: (`create_enum; 2);
/// q)sym: `a`b`c
/// q)enumerate["sym"; 1]
/// `sym$`b
/// q)enumerate["sym"; 3]
/// 'index out of enum range
///   [0]  enumerate["sym"; 3]
///        ^
/// q)enumerate["som"; 0]
/// 'som
/// [1]  som
///      ^
/// q))\
/// q)som:til 3
/// q)enumerate["som"; 0]
/// 'enum must be cast to symbol list
///   [0]  enumerate["som"; 0]
///        ^
/// q)som:`a`b
/// q)enumerate["som"; 0]
/// `som$`a
/// ```
#[inline]
pub fn new_enum(source: &str, index: J) -> K {
    let sym = unsafe { native::k(0, str_to_S!(source), KNULL) };
    if unsafe { (*sym).qtype } == qtype::ERROR {
        // Error. Specified sym does not exist
        sym
    } else if unsafe { (*sym).qtype } != qtype::SYMBOL_LIST {
        // sym is not a symbol list
        unsafe {
            native::r0(sym);
            native::krr(null_terminated_str_to_const_S(
                "enum must be cast to symbol list\0",
            ))
        }
    } else if unsafe { (*sym).value.list.n } <= index {
        // Index is out of sym range
        unsafe {
            native::r0(sym);
            native::krr(null_terminated_str_to_const_S("index out of enum range\0"))
        }
    } else {
        let function = format!("{{`{}${} x}}", source, source);
        unsafe {
            native::r0(sym);
            native::k(0, str_to_S!(function.as_str()), native::kj(index), KNULL)
        }
    }
}

/// Constructor of q simple list.
/// # Example
/// See the example of [`new_dictionary`](fn.new_dictionary.html).
#[inline]
pub fn new_list(qtype: i8, length: J) -> K {
    unsafe { native::ktn(qtype as I, length) }
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
pub fn new_string(string: &str) -> K {
    unsafe { native::kp(str_to_S!(string)) }
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
pub fn new_string_n(string: &str, length: J) -> K {
    unsafe { native::kpn(str_to_S!(string), length) }
}

/// Constructor of q dictionary object.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
/// use kdbplus::qtype;
///
/// #[no_mangle]
/// pub extern "C" fn create_dictionary() -> K{
///   let keys=new_list(qtype::INT_LIST, 2);
///   keys.as_mut_slice::<I>()[0..2].copy_from_slice(&[0, 1]);
///   let values=new_list(qtype::COMPOUND_LIST, 2);
///   let date_list=new_list(qtype::DATE_LIST, 3);
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
///
/// # Safety
/// inputs must be valid pointers
#[inline]
pub fn new_dictionary(keys: K, values: K) -> K {
    new_dictionary_unsafe(keys, values)
}
/// # Safety
/// inputs must be valid pointers
#[inline(always)]
fn new_dictionary_unsafe(keys: K, values: K) -> K {
    unsafe { native::xD(keys, values) }
}

/// Constructor of q general null.
/// # Example
/// ```no_run
/// use kdbplus::qtype;
/// use kdbplus::api::*;
///
/// #[no_mangle]
/// pub extern "C" fn nullify(_: K) -> K{
///   let nulls=new_list(qtype::COMPOUND_LIST, 3);
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
pub fn new_null() -> K {
    unsafe {
        let null = native::ka(qtype::NULL as I);
        (*null).value.byte = 0;
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
pub fn new_error(message: &str) -> K {
    unsafe { native::krr(null_terminated_str_to_const_S(message)) }
}

/// Similar to `new_error` but this function appends a system-error message to string `S` before passing it to internal `krr`.
///  The input must be null-terminated.
#[inline]
pub fn new_error_os(message: &str) -> K {
    unsafe { native::orr(null_terminated_str_to_const_S(message)) }
}

/// Convert an error object into usual `K` object which has the error string in the field `symbol`.
/// # Example
/// ```no_run
/// use kdbplus::*;
/// use kdbplus::api::*;
///
/// extern "C" fn no_panick(func: K, args: K) -> K{
///   let result=error_to_string(apply(func, args));
///   if let Ok(error) = result.get_error_string(){
///     println!("FYI: {}", error);
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
/// # Note
/// If you intend to use the error string only in Rust side and not to return the value, you need
///  to decrement the reference count of the error object created by `error_to_string` as shown above.
///  If you want to propagate the error to q side after some operation, you can just return it (See the
///  example of [`is_error`](fn.is_error.html)).
///
/// # Safety
/// In q, an error is a 0 pointer. This causes a problem of false positive by `error_to_string`, i.e.,
///  `KNULL` is also catched as an error object and its type is set `qtype::ERROR`. In such a case you must NOT
///  return the catched object because it causes segmentation fault. If you want to check if the catched object
///  is an error and then return if it is, you should use [`is_error`](fn.is_error.html). If you want to use the
///  underlying error string of the catched object, you should use [`get_error_string`](trait.KUtility.html#tymethod.get_error_string).
#[inline]
pub fn error_to_string(error: K) -> K {
    error_to_string_unsafe(error)
}
/// # Safety
/// input must be a valid pointer
#[inline(always)]
fn error_to_string_unsafe(error: K) -> K {
    unsafe { native::ee(error) }
}

/// Judge if a catched object by [`error_to_string`](fn.error_to_string.html) is a genuine error object of type
///  `qtype::ERROR` (This means false positive of the `KNULL` case can be eliminated).
/// # Examples
/// ```no_run
/// use kdbplus::*;
/// use kdbplus::api::*;
///
/// fn love_even(arg: K) -> K{
///   if let Ok(int) = arg.get_int(){
///     if int % 2 == 0{
///       // Silent for even value
///       KNULL
///     }
///     else{
///       // Shout against odd value
///       new_error("great is the even value!!\0")
///     }
///   }
///   else{
///     // Pass through
///     increment_reference_count(arg)
///   }
/// }
///
/// #[no_mangle]
/// pub extern "C" fn propagate(arg: K) -> K{
///   let result=error_to_string(love_even(arg));
///   if is_error(result){
///     // Propagate the error
///     result
///   }
///   else if result.get_type() == qtype::ERROR{
///     // KNULL
///     println!("this is KNULL");
///     decrement_reference_count(result);
///     KNULL
///   }
///   else{
///     // Other
///     new_symbol("sonomama")
///   }
/// }
/// ```
/// ```q
/// q)convey: `libapi_examples 2: (`propagate; 1);
/// q)convey[7i]
/// 'great is the even value!!
/// q)convey[12i]
/// this is KNULL
/// q)convey[5.5]
/// `sonomama
/// ```
/// # Note
/// In this example `KNULL` is used as a returned value of the function called by another function to demonstrate
///  how `is_error` works. However, `KNULL` should not be used in such a way in order to avoid this kind of complexity.
///  To return a general null for inner functions, use [`new_null`](fn.new_null.html) instead.
///
///  # Safety
///  The input must be a valid pointer.
#[inline]
pub fn is_error(catched: K) -> bool {
    is_error_unsafe(catched)
}
/// # Safety unsure K is a valid pointer.
fn is_error_unsafe(catched: K) -> bool {
    (unsafe { *catched }).qtype == qtype::ERROR && !(unsafe { (*catched).value.symbol }).is_null()
}

//%% Symbol %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Extract the first `n` chars from a character array and enumerate it internally.
///  This function must be used to add a character array as a symbol value to a symbol list.
///  The returned value is the same character array as the input.
/// # Example
/// See the example of [`flip`](fn.flip.html).
/// # Note
/// The reason why this function must be used is to enumerate the character array before handling
///  it as a q symbol type value. q/kdb+ is enumerating all symbol values to optimize comparison
///  or memory usage. On the other hand [`new_symbol`] does the enumeration internally and
///  therefore it does not need this function.
/// # Safety
/// The input must be a valid pointer.
#[inline]
pub fn enumerate_n(string: S, n: I) -> S {
    enumerate_n_unsafe(string, n)
}
/// # Safety
/// The input must be a valid pointer.
#[inline(always)]
fn enumerate_n_unsafe(string: S, n: I) -> S {
    unsafe { native::sn(string, n) }
}

/// Enumerate a null-terminated character array internally. This function must be used
///  to add a character array as a symbol value to a symbol list. The returned value is
///  the same character array as the input.
/// # Example
/// See the example of [`flip`](fn.flip.html).
/// # Note
/// The reason why this function must be used is to enumerate the character array before handling
///  it as a q symbol type value. q/kdb+ is enumerating all symbol values to optimize comparison
///  or memory usage. On the other hand [`new_symbol`] does the enumeration internally and
///  therefore it does not need this function.
/// # Safety
/// The input must be a valid pointer.
#[inline]
pub fn enumerate(string: S) -> S {
    enumerate_unsafe(string)
}
/// # Safety
/// The input must be a valid pointer.
#[inline(always)]
fn enumerate_unsafe(string: S) -> S {
    unsafe { native::ss(string) }
}

//%% Table %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

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
///   let keys=new_list(qtype::SYMBOL_LIST, 2);
///   let keys_slice=keys.as_mut_slice::<S>();
///   keys_slice[0]=enumerate(str_to_S!("time"));
///   keys_slice[1]=enumerate_n(str_to_S!("temperature_and_humidity"), 11);
///   
///   // Build values
///   let values=new_list(qtype::COMPOUND_LIST, 2);
///   let time=new_list(qtype::TIMESTAMP_LIST, 3);
///   // 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392
///   time.as_mut_slice::<J>().copy_from_slice(&[119067859167018272_i64, 201766609419710368, 271897944018691392]);
///   let temperature=new_list(qtype::FLOAT_LIST, 3);
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
/// # Safety
/// The input must be a valid pointer.
#[inline]
pub fn flip(dictionary: K) -> K {
    flip_unsafe(dictionary)
}
/// # Safety
/// The input must be a valid pointer.
#[inline(always)]
fn flip_unsafe(dictionary: K) -> K {
    match unsafe { (*dictionary).qtype } {
        qtype::DICTIONARY => unsafe { native::xT(dictionary) },
        _ => unsafe { native::krr(null_terminated_str_to_const_S("not a dictionary\0")) },
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
///   let keys=new_list(qtype::SYMBOL_LIST, 2);
///   let keys_slice=keys.as_mut_slice::<S>();
///   keys_slice[0]=enumerate(str_to_S!("time"));
///   keys_slice[1]=enumerate_n(str_to_S!("temperature_and_humidity"), 11);
///   
///   // Build values
///   let values=new_list(qtype::COMPOUND_LIST, 2);
///   let time=new_list(qtype::TIMESTAMP_LIST, 3);
///   // 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392
///   time.as_mut_slice::<J>().copy_from_slice(&[119067859167018272_i64, 201766609419710368, 271897944018691392]);
///   let temperature=new_list(qtype::FLOAT_LIST, 3);
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
///
/// # Safety
/// input must be a valid pointer
#[inline]
pub fn unkey(keyed_table: K) -> K {
    unkey_unsafe(keyed_table)
}
/// # Safety
/// input must be a valid pointer
#[inline(always)]
fn unkey_unsafe(keyed_table: K) -> K {
    match unsafe { (*keyed_table).qtype } {
        qtype::DICTIONARY => unsafe { native::ktd(keyed_table) },
        _ => unsafe { native::krr(null_terminated_str_to_const_S("not a keyed table\0")) },
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
///   let keys=new_list(qtype::SYMBOL_LIST, 2);
///   let keys_slice=keys.as_mut_slice::<S>();
///   keys_slice[0]=enumerate(str_to_S!("time"));
///   keys_slice[1]=enumerate_n(str_to_S!("temperature_and_humidity"), 11);
///   
///   // Build values
///   let values=new_list(qtype::COMPOUND_LIST, 2);
///   let time=new_list(qtype::TIMESTAMP_LIST, 3);
///   // 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392
///   time.as_mut_slice::<J>().copy_from_slice(&[119067859167018272_i64, 201766609419710368, 271897944018691392]);
///   let temperature=new_list(qtype::FLOAT_LIST, 3);
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
///
/// # Safety
/// input must be a valid pointer
#[inline]
pub fn enkey(table: K, n: J) -> K {
    enkey_unsafe(table, n)
}
/// # Safety
/// input must be a valid pointer
#[inline(always)]
fn enkey_unsafe(table: K, n: J) -> K {
    match unsafe { (*table).qtype } {
        qtype::TABLE => unsafe { native::knt(n, table) },
        _ => unsafe { native::krr(null_terminated_str_to_const_S("not a table\0")) },
    }
}

//%% Reference Count %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvv/

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
///
/// # Safety
/// input must be a valid pointer
#[inline]
pub fn decrement_reference_count(qobject: K) -> V {
    decrement_reference_count_unsafe(qobject)
}
/// # Safety
/// input must be a valid pointer
#[inline(always)]
fn decrement_reference_count_unsafe(qobject: K) -> V {
    unsafe { native::r0(qobject) }
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
///
/// # Safety
/// input must be a valid pointer
#[inline]
pub fn increment_reference_count(qobject: K) -> K {
    increment_reference_count_unsafe(qobject)
}
/// # Safety
/// input must be a valid pointer
#[inline(always)]
fn increment_reference_count_unsafe(qobject: K) -> K {
    unsafe { native::r1(qobject) }
}

//%% Callback %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Remove callback from the associated kdb+ socket and call `kclose`.
///  Return null if the socket is invalid or not the one which had been registered by `sd1`.
/// # Note
/// A function which calls this function must be executed at the exit of the process.
#[inline]
pub fn destroy_socket(socket: I) {
    unsafe {
        native::sd0(socket);
    }
}

/// Remove callback from the associated kdb+ socket and call `kclose` if the given condition is satisfied.
///  Return null if the socket is invalid or not the one which had been registered by `sd1`.
/// # Note
/// A function which calls this function must be executed at the exit of the process.
#[inline]
pub fn destroy_socket_if(socket: I, condition: bool) {
    unsafe {
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
///     let mut precious=new_list(qtype::SYMBOL_LIST, 3);
///     let precious_array=precious.as_mut_slice::<S>();
///     precious_array[0]=enumerate(null_terminated_str_to_S("belief\0"));
///     precious_array[1]=enumerate(null_terminated_str_to_S("love\0"));
///     precious_array[2]=enumerate(null_terminated_str_to_S("hope\0"));
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
pub fn register_callback(socket: I, function: extern "C" fn(I) -> K) -> K {
    unsafe { native::sd1(socket, function) }
}

//%% Miscellaneous %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Apply a function to q list object `.[func; args]`.
/// # Example
/// See the example of [`error_to_string`](fn.error_to_string.html).
///
/// # Safety
/// inputs must be valid pointers
#[inline]
pub fn apply(func: K, args: K) -> K {
    apply_unsafe(func, args)
}
/// # Safety
/// inputs must be valid pointers
#[inline(always)]
fn apply_unsafe(func: K, args: K) -> K {
    unsafe { native::dot(func, args) }
}

/// Enable the remote threads to refer to the sym list in the main thread so that enumeration
///  of remotely created symbol values reain valid in the main thread after joining the
///  remote threads. This function must be used before starting any other threads if the
///  threads create symbol values. The previously set value is returned.
/// # Example
/// See the example of [`register_callback`](fn.register_callback.html).
#[inline]
pub fn pin_symbol() -> I {
    unsafe { native::setm(1) }
}

/// Unlock the symbol list in the main thread. This function should be called after joining
///  threads.
/// # Example
/// See the example of [`register_callback`](fn.register_callback.html).
#[inline]
pub fn unpin_symbol() -> I {
    unsafe { native::setm(0) }
}

/// Drop Rust object inside q. Passed as the first element of a foreign object.
/// # Parameters
/// - `obj`: List of (function to free the object; foreign object).
/// # Example
/// See the example of [`load_as_q_function`](fn.load_as_q_function.html).
pub fn drop_q_object(obj: K) -> K {
    let obj_slice = obj.as_mut_slice::<K>();
    // Take ownership of `K` object from a raw pointer and drop at the end of this scope.
    unsafe { Box::from_raw(obj_slice[1]) };
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
///   let mut foreign=new_list(qtype::COMPOUND_LIST, 2);
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
///
/// # Safety
/// input `func` must be a valid pointer to a C function that takes `n` `K` objects as arguments and returns a `K` object.
#[inline]
pub fn load_as_q_function(func: *const V, n: J) -> K {
    load_as_q_function_unsafe(func, n)
}
/// # Safety
/// input must be a valid pointer
#[inline(always)]
fn load_as_q_function_unsafe(func: *const V, n: J) -> K {
    unsafe { native::dl(func, n) }
}

/// Convert ymd to the number of days from `2000.01.01`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
///
/// let days=ymd_to_days(2020, 4, 1);
/// assert_eq!(days, 7396);
/// ```
#[inline]
pub fn ymd_to_days(year: I, month: I, date: I) -> I {
    unsafe { native::ymd(year, month, date) }
}

/// Convert the number of days from `2000.01.01` to a number expressed as `yyyymmdd`.
/// # Example
/// ```no_run
/// use kdbplus::api::*;
///
/// let number=days_to_ymd(7396);
/// assert_eq!(number, 20200401);
/// ```
#[inline]
pub fn days_to_ymd(days: I) -> I {
    unsafe { native::dj(days) }
}

/// Convert a simple list to a compound list. Expected usage is to concatinate a simple list
///  with a different type of list.
/// # Example
/// ```no_run
/// use kdbplus::*;
/// use kdbplus::api::*;
///
/// #[no_mangle]
/// pub extern "C" fn drift(_: K)->K{
///   let simple=new_list(qtype::INT_LIST, 2);
///   simple.as_mut_slice::<I>().copy_from_slice(&[12, 34]);
///   let extra=new_list(qtype::COMPOUND_LIST, 2);
///   extra.as_mut_slice::<K>().copy_from_slice(&[new_symbol("vague"), new_int(-3000)]);
///   // Convert an integer list into a compound list
///   let mut compound = simple_to_compound(simple, "");
///   compound.append(extra).unwrap()
/// }
///
/// #[no_mangle]
/// pub extern "C" fn drift2(_: K)->K{
///   let simple=new_list(qtype::ENUM_LIST, 2);
///   simple.as_mut_slice::<J>().copy_from_slice(&[0_i64, 1]);
///   // Convert an enum indices into a compound list while creating enum values from the indices which are tied with
///   //  an existing enum variable named "enum", i.e., Enum indices [0, 1] in the code are cast into `(enum[0]; enum[1])`.
///   let mut compound = simple_to_compound(simple, "enum");
///   // Add `enum2[2]`.
///   compound.push(new_enum("enum2", 2)).unwrap();
///   compound.push(new_month(3)).unwrap();
///   compound
/// }
/// ```
/// ```q
/// q)drift: LIBPATH_ (`drift; 1);
/// q)drift2: LIBPATH_ (`drift2; 1);
/// q)drift[]
/// 12i
/// 34i
/// `vague
/// -3000i
/// q)enum: `mashroom`broccoli`cucumber
/// q)enum2: `mackerel`swordfish`tuna
/// q)drift2[]
/// `enum$`mashroom
/// `enum$`broccoli
/// `enum2$`tuna
/// 2000.04m
/// ```
/// # Note
/// - To convert a list provided externally (i.e., passed from a q process), apply
///  [`increment_reference_count`](fn.increment_reference_count.html) before converting the list.
/// - Enum elements from different enum sources must be contained in a compound list. Therefore
///  this function intentionally restricts the number of enum sources to one so that user switches
///  a simple list to a compound list when the second enum sources are provided.
pub fn simple_to_compound(simple: K, enum_source: &str) -> K {
    let size = simple.len() as usize;
    let compound = new_list(qtype::COMPOUND_LIST, size as J);
    let compound_slice = compound.as_mut_slice::<K>();
    match simple.get_type() {
        qtype::BOOL_LIST => {
            let simple_slice = simple.as_mut_slice::<G>();
            for i in 0..size {
                compound_slice[i] = new_bool(simple_slice[i] as I);
            }
        }
        qtype::GUID_LIST => {
            let simple_slice = simple.as_mut_slice::<U>();
            for i in 0..size {
                compound_slice[i] = new_guid(simple_slice[i].guid);
            }
        }
        qtype::BYTE_LIST => {
            let simple_slice = simple.as_mut_slice::<G>();
            for i in 0..size {
                compound_slice[i] = new_byte(simple_slice[i] as I);
            }
        }
        qtype::SHORT_LIST => {
            let simple_slice = simple.as_mut_slice::<H>();
            for i in 0..size {
                compound_slice[i] = new_short(simple_slice[i] as I);
            }
        }
        qtype::INT_LIST => {
            let simple_slice = simple.as_mut_slice::<I>();
            for i in 0..size {
                compound_slice[i] = new_int(simple_slice[i]);
            }
        }
        qtype::LONG_LIST => {
            let simple_slice = simple.as_mut_slice::<J>();
            for i in 0..size {
                compound_slice[i] = new_long(simple_slice[i]);
            }
        }
        qtype::REAL_LIST => {
            let simple_slice = simple.as_mut_slice::<E>();
            for i in 0..size {
                compound_slice[i] = new_real(simple_slice[i] as F);
            }
        }
        qtype::FLOAT_LIST => {
            let simple_slice = simple.as_mut_slice::<F>();
            for i in 0..size {
                compound_slice[i] = new_float(simple_slice[i]);
            }
        }
        qtype::STRING => {
            let simple_slice = simple.as_mut_slice::<G>();
            for i in 0..size {
                compound_slice[i] = new_char(simple_slice[i] as char);
            }
        }
        qtype::SYMBOL_LIST => {
            let simple_slice = simple.as_mut_slice::<S>();
            for i in 0..size {
                compound_slice[i] = new_symbol(S_to_str(simple_slice[i]));
            }
        }
        qtype::TIMESTAMP_LIST => {
            let simple_slice = simple.as_mut_slice::<J>();
            for i in 0..size {
                compound_slice[i] = new_timestamp(simple_slice[i]);
            }
        }
        qtype::DATE_LIST => {
            let simple_slice = simple.as_mut_slice::<I>();
            for i in 0..size {
                compound_slice[i] = new_date(simple_slice[i]);
            }
        }
        qtype::TIME_LIST => {
            let simple_slice = simple.as_mut_slice::<I>();
            for i in 0..size {
                compound_slice[i] = new_time(simple_slice[i]);
            }
        }
        qtype::ENUM_LIST => {
            let simple_slice = simple.as_mut_slice::<J>();
            for i in 0..size {
                compound_slice[i] = new_enum(enum_source, simple_slice[i]);
            }
        }
        _ => {
            decrement_reference_count(compound);
            return new_error("not a simple list\0");
        }
    }
    // Free simple list
    decrement_reference_count(simple);
    compound
}

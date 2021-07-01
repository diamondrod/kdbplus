/
* @file test.q
* @overview Tests of C API examples. The artefact of `api_examples` is loaded
* and functions are called from q side.
\

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                           Inital Setting     			                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// Fetch shared object from build directory.
//system "cp ../target/debug/libapi_examples.so .";

// Load test helper functions.
\l test_helper_function.q

// Function to load shared library.
LIBPATH_: `libapi_examples 2:

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	Load Libraries     			                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// These function list can be checked against `nm -D libc_api_examples.so | awk '$2 ~/T/ {print $3}'`.

// decrement_reference_count
.api.agriculture: LIBPATH_ (`agriculture; 1);
// ee
.api.catchy: LIBPATH_ (`catchy; 2);
// qnull_base::C
.api.char_border: LIBPATH_ (`char_border; 1);
// jv
.api.concat_list: LIBPATH_ (`concat_list; 2);
// b9
.api.conceal: LIBPATH_ (`conceal; 1);
// append
.api.concat_list2: LIBPATH_ (`concat_list2; 2);
// kb
.api.create_bool: LIBPATH_ (`create_bool; 1);
// kb
.api.create_bool2: LIBPATH_ (`create_bool2; 1);
// kg
.api.create_byte: LIBPATH_ (`create_byte; 1);
// ku
.api.create_guid: LIBPATH_ (`create_guid; 1);
// ki
.api.create_int: LIBPATH_ (`create_int; 1);
// kj
.api.create_long: LIBPATH_ (`create_long; 1);
// kc
.api.create_char: LIBPATH_ (`create_char; 1);
// new_char
.api.create_char2: LIBPATH_ (`create_char2; 1);
// jk
.api.create_compound_list: LIBPATH_ (`create_compound_list; 1);
// push
.api.create_compound_list2: LIBPATH_ (`create_compound_list2; 1);
// kd
.api.create_date: LIBPATH_ (`create_date; 1);
// kz
.api.create_datetime: LIBPATH_ (`create_datetime; 1);
// xD
.api.create_dictionary: LIBPATH_ (`create_dictionary; 1);
// kf
.api.create_float: LIBPATH_ (`create_float; 1);
// knt
.api.create_keyed_table: LIBPATH_ (`create_keyed_table; 1);
// enkey
.api.create_keyed_table2: LIBPATH_ (`create_keyed_table2; 1);
// new_minute
.api.create_minute: LIBPATH_ (`create_minute; 1);
// new_month
.api.create_month: LIBPATH_ (`create_month; 1);
// ke
.api.create_real: LIBPATH_ (`create_real; 1);
// kh
.api.create_short: LIBPATH_ (`create_short; 1);
// new_second
.api.create_second: LIBPATH_ (`create_second; 1);
// ja
.api.create_simple_list: LIBPATH_ (`create_simple_list; 1);
// push_raw
.api.create_simple_list2: LIBPATH_ (`create_simple_list2; 1);
// kp
.api.create_string: LIBPATH_ (`create_string; 1);
// kpn
.api.create_string2: LIBPATH_ (`create_string2; 1);
// ks
.api.create_symbol: LIBPATH_ (`create_symbol; 1);
// new_symbol
.api.create_symbol2: LIBPATH_ (`create_symbol2; 1);
// js
.api.create_symbol_list: LIBPATH_ (`create_symbol_list; 1);
// push_symbol
.api.create_symbol_list2: LIBPATH_ (`create_symbol_list2; 1);
// xT
.api.create_table: LIBPATH_ (`create_table; 1);
// flip
.api.create_table2: LIBPATH_ (`create_table2; 1);
// kt
.api.create_time: LIBPATH_ (`create_time; 1);
// ktj
.api.create_timespan: LIBPATH_ (`create_timespan; 1);
// new_timespan
.api.create_timespan2: LIBPATH_ (`create_timespan2; 1);
// ktj
.api.create_timestamp: LIBPATH_ (`create_timestamp; 1);
// new_timestamp
.api.create_timestamp2: LIBPATH_ (`create_timestamp2; 1);
// dj
.api.days_to_date: LIBPATH_ (`days_to_date; 1);
// q_ipc_decode
.api.decrypt: LIBPATH_ (`decrypt; 1);
// k
.api.dictionary_list_to_table: LIBPATH_ (`dictionary_list_to_table; 1);
// set_qtype
.api.eden: LIBPATH_ (`eden; 1);
// q_ipc_encode
.api.encrypt: LIBPATH_ (`encrypt; 1);
// qnull_base::F
.api.float_borders: LIBPATH_ (`float_borders; 1);
// qnull_base::U
.api.guid_border: LIBPATH_ (`guid_border; 1);
// get_dictionary
.api.hidden_key: LIBPATH_ (`hidden_key; 1);
// r0
.api.idle_man: LIBPATH_ (`idle_man; 1);
// qnull_base::I
.api.int_borders: LIBPATH_ (`int_borders; 1);
// new_error
.api.keep_out: LIBPATH_ (`keep_out; 1);
// ktd
.api.keyed_to_simple_table: LIBPATH_ (`keyed_to_simple_table; 1);
// unkey
.api.keyed_to_simple_table2: LIBPATH_ (`keyed_to_simple_table2; 1);
// set_attribute
.api.labeling: LIBPATH_ (`labeling; 1);
// qnull_base::J
.api.long_borders: LIBPATH_ (`long_borders; 1);
// as_mut_slice
.api.modify_long_list_a_bit: LIBPATH_ (`modify_long_list_a_bit; 1);
// get_attribute
.api.murmur: LIBPATH_ (`murmur; 1);
// str_to_const_S
.api.must_be_int: LIBPATH_ (`must_be_int; 1);
// len
.api.numbers: LIBPATH_ (`numbers; 1);
// error_to_string
.api.no_panick: LIBPATH_ (`no_panick; 2);
// new_null
.api.nullify: LIBPATH_ (`nullify; 1);
// setm
.api.parallel_sym_change: LIBPATH_ (`parallel_sym_change; 1);
// r1
.api.pass_through_cave: LIBPATH_ (`pass_through_cave; 1);
// str_to_S
.api.pingpong: LIBPATH_ (`pingpong; 1);
// null_terminated_str_to_S
.api.pingpong2: LIBPATH_ (`pingpong2; 1);
// register_callback
.api.plumber: LIBPATH_ (`plumber; 1);
// get_bool
.api.print_bool: LIBPATH_ (`print_bool; 1);
// get_byte
.api.print_byte: LIBPATH_ (`print_byte; 1);
// get_char
.api.print_char: LIBPATH_ (`print_char; 1);
// get_float
.api.print_float: LIBPATH_ (`print_float; 1);
// get_guid
.api.print_guid: LIBPATH_ (`print_guid; 1);
// get_int
.api.print_int: LIBPATH_ (`print_int; 1);
// get_long
.api.print_long: LIBPATH_ (`print_long; 1);
// get_real
.api.print_real: LIBPATH_ (`print_real; 1);
// get_short
.api.print_short: LIBPATH_ (`print_short; 1);
// get_str
.api.print_string: LIBPATH_ (`print_string; 1);
// get_string
.api.print_string2: LIBPATH_ (`print_string2; 1);
// S_to_str
.api.print_symbol: LIBPATH_ (`print_symbol; 1);
// get_symbol
.api.print_symbol2: LIBPATH_ (`print_symbol2; 1);
// load_as_q_function
.api.probe: LIBPATH_ (`probe; 1);
// error_to_string
.api.propagate: LIBPATH_ (`propagate; 1);
// qnull_base::E
.api.real_borders: LIBPATH_ (`real_borders; 1);
// d9
.api.reveal: LIBPATH_ (`reveal; 1);
// dot
.api.rust_parse: LIBPATH_ (`rust_parse; 2);
// increment_reference_count
.api.satisfy_5000_men: LIBPATH_ (`satisfy_5000_men; 1);
// qnull_base::H
.api.short_borders: LIBPATH_ (`short_borders; 1);
// qnull_base::S
.api.string_borders: LIBPATH_ (`string_borders; 1);
// krr
.api.thai_kick: LIBPATH_ (`thai_kick; 1);
// KNULL
.api.vanity: LIBPATH_ (`vanity; 1);
// ymd
.api.ymd_to_days: LIBPATH_ (`ymd_to_days; 1);

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	  Tests    	        		                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Global Variable %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// KNULL
.test.ASSERT_EQ["KNULL"; .api.vanity[]; (::)]
// qnull_base::U
.test.ASSERT_EQ["qnull_base::U"; .api.guid_border[]; 0Ng]
// qnull_base::H
.test.ASSERT_EQ["qnull_base::H"; .api.short_borders[]; (0Nh; 0Wh; -0Wh)]
// qnull_base::I
.test.ASSERT_EQ["qnull_base::I"; .api.int_borders[]; (0Ni; 0Wi; -0Wi)]
// qnull_base::J
.test.ASSERT_EQ["qnull_base::J"; .api.long_borders[]; (0Np; 0Wp; -0Wp)]
// qnull_base::E
.test.ASSERT_EQ["qnull_base::E"; .api.real_borders[]; (0Ne; 0We; -0We)]
// qnull_base::F
.test.ASSERT_EQ["qnull_base::F"; .api.float_borders[]; (0Nz; 0Wz; -0Wz)]
// qnull_base::C
.test.ASSERT_EQ["qnull_base::C"; .api.char_border[]; " "]
// qnull_base::S
.test.ASSERT_EQ["qnull_base::S"; .api.string_borders[]; (`; "")]

//%% Macros %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// str_to_S
ping:{[int] `$string[int], "_pong!!"};
.test.ASSERT_EQ["str_to_S"; .api.pingpong[]; `$"77_pong!!"]

//%% KUtility %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// as_mut_slice
// Assign to a variable to keep the result.
.test.ASSERT_EQ["as_mut_slice - success"; .api.modify_long_list_a_bit[list:1 2 3]; 1 30000 3]
// as_mut_slice (return error)
.test.ASSERT_ERROR["as_mut_slice - failure"; .api.modify_long_list_a_bit; enlist enlist 1; "this list is not long enough"]

// get_bool
.test.ASSERT_EQ["get_bool - true"; .api.print_bool[1b]; (::)]
// get_bool
.test.ASSERT_EQ["get_bool - false"; .api.print_bool[0b]; (::)]
// get_bool - failure
.test.ASSERT_ERROR["get_bool - failure"; .api.print_bool; enlist 100; "not a bool"]

// get_byte
.test.ASSERT_EQ["get_byte"; .api.print_byte[0xc4]; (::)]
// get_byte - failure
.test.ASSERT_ERROR["get_byte - failure"; .api.print_byte; enlist "c"; "not a byte"]

// get_guid
guid: first 1?0Ng;
.test.ASSERT_EQ["get_guid"; .api.print_guid[guid]; (::)]
// get_guid - failure
.test.ASSERT_ERROR["get_guid - failure"; .api.print_guid; enlist 0x7a; "not a GUID"]

// get_short
.test.ASSERT_EQ["get_short"; .api.print_short[10h]; (::)]
// get_short - failure
.test.ASSERT_ERROR["get_short - failure"; .api.print_short; enlist 10; "not a short"]

// get_int
.test.ASSERT_EQ["get_int"; .api.print_int[42i]; (::)]
// get_int - month
.test.ASSERT_EQ["get_int - month"; .api.print_int[2010.03m]; (::)]
// get_int - date
.test.ASSERT_EQ["get_int - date"; .api.print_int[2020.02.01]; (::)]
// get_int - minute
.test.ASSERT_EQ["get_int - minute"; .api.print_int[12:03]; (::)]
// get_int - second
.test.ASSERT_EQ["get_int - second"; .api.print_int[03:57:20]; (::)]
// get_int - time
.test.ASSERT_EQ["get_int - time"; .api.print_int[00:34:16.636]; (::)]
// get_int - error
.test.ASSERT_ERROR["get_int - failure1"; .api.print_int; enlist `error; "not an int"]
// get_int - error
.test.ASSERT_ERROR["get_int - failure2"; .api.print_int; enlist 10000; "not an int"]

// get_long
.test.ASSERT_EQ["get_long"; .api.print_long[-109210]; (::)]
// get_long - timestamp
.test.ASSERT_EQ["get_long - timestamp"; .api.print_long[2000.01.01D12:00:00.123456789]; (::)]
// get_long - timespan
.test.ASSERT_EQ["get_long - timespan"; .api.print_long[-3D18:23:09.000000021]; (::)]
// get_long - error
.test.ASSERT_ERROR["get_long - failure"; .api.print_long; enlist 1b; "not a long"]

// get_real
.test.ASSERT_EQ["get_real"; .api.print_real[193810.32e]; (::)]
// get_real - error
.test.ASSERT_ERROR["get_real - failure"; .api.print_real; enlist 100f; "not a real"]

// get_float
.test.ASSERT_EQ["get_float"; .api.print_float[-37017.0933]; (::)]
// get_float - datetime
.test.ASSERT_EQ["get_float - datetime"; .api.print_float[2002.01.12T10:03:45.332]; (::)]
// get_float - error
.test.ASSERT_ERROR["get_float - failure"; .api.print_float; enlist .z.p; "not a float"]

// get_char
.test.ASSERT_EQ["get_char"; .api.print_char["k"]; (::)]
// get_char - error
.test.ASSERT_ERROR["get_char - failure1"; .api.print_char; enlist "devour"; "not a char"]
// get_char - error
.test.ASSERT_ERROR["get_char - failure2"; .api.print_char; enlist 1b; "not a char"]

// get_symbol
.test.ASSERT_EQ["get_symbol"; .api.print_symbol2[`locust]; (::)]
// get_symool - error
.test.ASSERT_ERROR["get_symbol - failure"; .api.print_symbol2; enlist "attack!"; "not a symbol"]

// get_str
.test.ASSERT_EQ["get_str"; .api.print_string["gnat"]; (::)]

// get_string
.test.ASSERT_EQ["get_string"; .api.print_string2["grasshopper"]; (::)]
// get_string - error
.test.ASSERT_ERROR["get_string - failure"; .api.print_string2; enlist (1 2; `a`b); "not a string"]

// get_dictionary
.test.ASSERT_EQ["get_string"; .api.hidden_key[([] t: `timestamp$.z.p+1e9*til 9; chr:"ljppkgfgs"; is: 7 8 12 14 21 316 400 1000 6000i)]; -8!`t`chr`is]
// get_dictionary - error
.test.ASSERT_ERROR["get_dictionary - failure"; .api.hidden_key; enlist 777; "not a table"]

// get_attribute - sorted
.test.ASSERT_EQ["get_attribute - sorted"; .api.murmur[`s#1 2 3]; "Clean"]
// get_attribute - unique
.test.ASSERT_EQ["get_attribute - unique"; .api.murmur[`u#1 2 3]; `Alone]
// get_attribute - parted
.test.ASSERT_EQ["get_attribute - parted"; .api.murmur[`p#1 2 3]; (::)]

// append
.test.ASSERT_EQ["append - compound"; .api.concat_list2[(::; `metals; `fire); ("clay"; 316)]; (::; `metals; `fire; "clay"; 316)]
.test.ASSERT_EQ["append - long"; .api.concat_list2[1 2 3; 4 5]; 1 2 3 4 5]
.test.ASSERT_EQ["append - symbol"; .api.concat_list2[`a`b`c; `d`e]; `a`b`c`d`e]
// append - error
.test.ASSERT_ERROR["append - failure"; .api.concat_list2; (1 2 3; "45"); "not a list or types do not match"]

// push
.test.ASSERT_EQ["push"; .api.create_compound_list2[5i]; (til 5), 5i]

// push_raw
.test.ASSERT_EQ["push_raw"; .api.create_simple_list2[]; 2000.01.01+til 5]

// push_symbol
.test.ASSERT_EQ["push_symbol"; .api.create_symbol_list2[]; `Abraham`Isaac`Jacob`Joseph]

// len - general null
.test.ASSERT_EQ["len general null"; .api.numbers (::); "1 people are in numbers"]
// len - atom
.test.ASSERT_EQ["len atom"; .api.numbers first 1?0Ng; "1 people are in numbers"]
// len - list
.test.ASSERT_EQ["len list"; .api.numbers til 5; "5 people are in numbers"]
// len - dictionary
.test.ASSERT_EQ["len dictionary"; .api.numbers `a`b!("many"; `split`asunder); "2 people are in numbers"]
// len - table
.test.ASSERT_EQ["len table"; .api.numbers ([] x: til 10); "10 people are in numbers"]

// set_type
planet: .api.eden[];
.test.ASSERT_EQ["set_qtype"; type planet; 112h]

// set_attribute
.test.ASSERT_EQ["set_attribute"; .api.labeling 1 2 3; `s#1 2 3]
// set_attribute - failure
.test.ASSERT_ERROR["set_attribute - failure"; .api.labeling; enlist 777; "not a simple list"]

// q_ipc_encode
list: (til 3; "abc"; 2018.02.18D04:30:00.000000000; `revive);
.test.ASSERT_EQ["q_ipc_encode"; .api.encrypt[list]; bytes:-8!list]

// q_ipc_encode - compress
long_list: 1000#/: ("long"; `list);
.test.ASSERT_EQ["q_ipc_encode - compress"; .api.encrypt[long_list]; long_bytes:-8!long_list]

// q_ipc_decode
.test.ASSERT_EQ["q_ipc_decode"; .api.decrypt[bytes]; list]

// q_ipc_decode - decompress
.test.ASSERT_EQ["q_ipc_decode - decompress"; .api.decrypt[long_bytes]; long_list]

// q_ipc_decode - failure
.test.ASSERT_ERROR["q_ipc_decode - failure"; .api.decrypt; enlist `hello; "not bytes"]

// q_ipc_decode - failure2
.test.ASSERT_ERROR["q_ipc_decode - failure2"; .api.decrypt; enlist 0x00aa12345678; "failed to decode"]

//%% Constructors %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// kb
.test.ASSERT_EQ["kb"; .api.create_bool[]; 1b]

// ku
.test.ASSERT_EQ["ku"; .api.create_guid[]; "G"$"1e11170c-4224-252c-1c14-1e224d3d4624"]

// kg
.test.ASSERT_EQ["kg"; .api.create_byte[]; 0x3c]

// kh
.test.ASSERT_EQ["kh"; .api.create_short[]; -144h]

// ki
.test.ASSERT_EQ["ki"; .api.create_int[]; 86400000i]

// kj
.test.ASSERT_EQ["kj"; .api.create_long[]; -668541276001729000]

// ke
.test.ASSERT_EQ["ke"; .api.create_real[]; 0.00324e]

// kf
.test.ASSERT_EQ["kf"; .api.create_float[]; -6302.620]

// kc
.test.ASSERT_EQ["kc"; .api.create_char[]; "q"]

// ks
.test.ASSERT_EQ["ks"; .api.create_symbol[]; `symbolism]

// ktj - timestamp
.test.ASSERT_EQ["ktj - timestamp"; .api.create_timestamp[]; 2015.03.16D00:00:00:00.000000000]

// ktj - timespan
.test.ASSERT_EQ["ktj - timespan"; .api.create_timespan[]; -1D01:30:00.001234567]

// kd
.test.ASSERT_EQ["kd"; .api.create_date[]; 1999.12.25]

// kz
.test.ASSERT_EQ["kz"; .api.create_datetime[]; 2015.03.16T12:00:00:00]

// kt
.test.ASSERT_EQ["kz"; .api.create_time[]; -01:30:00.123]

// kp
.test.ASSERT_EQ["kp"; .api.create_string[]; "this is a text."]

// kpn
.test.ASSERT_EQ["kpn"; .api.create_string2[]; "The meeting was too long"]

// xT
.test.ASSERT_EQ["xT"; .api.create_table[]; table:([] time: 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392; temperature: 22.1, 24.7, 30.5)]

// ktd
.test.ASSERT_EQ["xT"; .api.keyed_to_simple_table[]; table]

// xD
.test.ASSERT_EQ["xD"; .api.create_dictionary[]; 0 1i!(2000.01.01 2000.01.02 2000.01.03; "I'm afraid I would crash the application...")]

// knt
.test.ASSERT_EQ["xT"; .api.create_keyed_table[]; 1!table]

// krr
.test.ASSERT_ERROR["krr"; .api.thai_kick; enlist (::); "Thai kick unconditionally!!"]

// ja
.test.ASSERT_EQ["ja"; .api.create_simple_list[]; 2000.01.01D00:00:00 2000.01.02D00:00:00 2000.01.03D00:00:00 2000.01.04D00:00:00 2000.01.05D00:00:00 ]

// jv
.test.ASSERT_EQ["jv - compound"; .api.concat_list[(::; `metals; `fire); ("clay"; 316)]; (::; `metals; `fire; "clay"; 316)]
.test.ASSERT_EQ["jv - long"; .api.concat_list[1 2 3; 4 5]; 1 2 3 4 5]
.test.ASSERT_EQ["jv - symbol"; .api.concat_list[`a`b`c; `d`e]; `a`b`c`d`e]

// jk
.test.ASSERT_EQ["jk"; .api.create_compound_list[]; (`1st; 2i; "3rd")]

// js
.test.ASSERT_EQ["js"; .api.create_symbol_list[]; `Abraham`Isaac`Jacob`Joseph]

// ee
.test.ASSERT_EQ["ee - success"; .api.catchy[$; ("S"; "rust")]; `rust]
// ee (print error to stdout)
.test.ASSERT_EQ["ee - failure"; .api.catchy[+; (2; "rust")]; (::)]

//%% IPC Functions %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// k
.test.ASSERT_EQ[enlist "k"; .api.dictionary_list_to_table[]; ([] a: 0 10 20i; b: 0 100 200i)]

// b9
.test.ASSERT_EQ["b9"; .api.conceal["Look! HE has come!!"]; -8!"Look! HE has come!!"]

// d9
.test.ASSERT_EQ["d9"; .api.reveal[-8!(`contact`from; "space"; 12)]; (`contact`from; "space"; 12)]

//%% Reference count %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// r0
.test.ASSERT_EQ["r0"; .api.idle_man[]; (::)]

// r1
get_item1:{[man] "a basket of summer fruit"};
get_item2:{[man] "boiling pot, facing away from the north"}
.test.ASSERT_EQ["r1"; .api.pass_through_cave[`son_of_man]; `son_of_man]

//%% Miscellaneous %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// dot
.test.ASSERT_EQ["dot"; .api.rust_parse[$; ("J"; "42")]; 42]

// setm
.test.ASSERT_EQ["dot"; .api.parallel_sym_change[`a`b]; `replaced`symbolbol]

// ymd
.test.ASSERT_EQ["ymd"; .api.ymd_to_days[]; 7396i]

// dj
.test.ASSERT_EQ["dj"; .api.days_to_date[7396i]; 20200401i]

//%% Utility Functions %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// S_to_str (print value to stdout)
.test.ASSERT_EQ["S_to_str"; .api.print_symbol[`rust]; (::)]

// null_terminated_str_to_S
.test.ASSERT_EQ["null_terminated_str_to_S"; .api.pingpong2[]; `$"77_pong!!"]

// null_terminated_str_to_const_S
.test.ASSERT_ERROR["str_to_const_S"; .api.must_be_int; enlist 10000; "not an int"]

//%% Re-Export %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// new_char
.test.ASSERT_EQ["new_char"; .api.create_char2[]; "t"]

// new_symbol
.test.ASSERT_EQ["new_symbol"; .api.create_symbol2[]; `symbolic]

// new_timestamp
.test.ASSERT_EQ["new_timestamp"; .api.create_timestamp2[]; 2015.03.16D00:00:00:00.000000000]

// new_month
.test.ASSERT_EQ["new_month"; .api.create_month[]; 2010.07m]

// new_timespan
.test.ASSERT_EQ["new_timespan"; .api.create_timespan2[]; -1D01:30:00.001234567]

// new_minute
.test.ASSERT_EQ["new_minute"; .api.create_minute[]; 10:40]

// new_second
.test.ASSERT_EQ["new_second"; .api.create_second[]; -02:00:00]

// new_null
.test.ASSERT_EQ["new_second"; .api.nullify[]; (::; "null is not a general null"; ::)]

// new_error
.test.ASSERT_ERROR["new_error"; .api.keep_out; enlist (::); "No means no"]

// error_to_string
.test.ASSERT_EQ["error_to_string"; .api.no_panick[sum; enlist til 10]; 45]
// error_to_string - failure
.test.ASSERT_EQ["error_to_string - failure"; .api.no_panick[sum; enlist `cannot`add`symbol]; (::)]
// error_to_string - positive true
.test.ASSERT_ERROR["error_to_string - positive true"; .api.propagate; enlist 7i; "great is the even value!!"]
// error_to_string - positive false
.test.ASSERT_EQ["error_to_string - positive false"; .api.propagate[12i]; (::)]

// flip
.test.ASSERT_EQ["flip"; .api.create_table2[]; table:([] time: 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392; temperature: 22.1, 24.7, 30.5)]

// unkey
.test.ASSERT_EQ["unkey"; .api.keyed_to_simple_table2[]; table]

// key
.test.ASSERT_EQ["enkey"; .api.create_keyed_table2[]; 1!table]

// load_as_q_function
invade: .api.probe[planet];
.test.ASSERT_EQ["load_as_q_function"; invade 1b; "The planet earth is a beautiful planet where 7500000000 people reside. Furthermore water is flowing on the surface of it. You shall not curse what God blessed."]
.test.ASSERT_EQ["load_as_q_function"; invade 0b; "The planet earth is a beautiful planet where 7500000000 people reside. Furthermore water is flowing on the surface of it. I perceived I could find favor of God by blessing them."]

// decrement_reference_count
.test.ASSERT_EQ["decrement_eference_count"; .api.agriculture[]; (::)]

// increment_reference_count
eat:{[apple] show "Collect the clutter of apples!";};
.test.ASSERT_EQ["increment_eference_count"; .api.satisfy_5000_men[`green_apple]; `green_apple]

// register_callback
shout:{[precious] -1 "What are the three largest elements?: ", .Q.s1 precious;};
.test.ASSERT_EQ["register_callback"; .api.plumber[]; (::)]

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	  Result   	        		                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// Show result.
.test.DISPLAY_RESULT[]

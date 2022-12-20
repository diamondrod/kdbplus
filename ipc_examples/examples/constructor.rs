use kdbplus::qattribute;
use kdbplus::ipc::*;
use chrono::prelude::*;
use chrono::Duration;

fn main(){
  let boolean=K::new_bool(true);
  println!("boolean: {}", boolean);

  let bool_list=K::new_bool_list(vec![true, false, false], qattribute::NONE);
  println!("bool list: {}", bool_list);

  let string=K::new_string(String::from("something"), qattribute::NONE);
  println!("string: {}", string);

  let timestamp=K::new_timestamp_list(vec![NaiveDate::from_ymd_opt(2009, 5, 28).unwrap().and_hms_nano_opt(17, 30, 0, 188906523).unwrap().and_local_timezone(Utc).unwrap()], qattribute::NONE);
  println!("timestamp_list: {}", timestamp);

  let timespan_list=K::new_timespan_list(vec![Duration::minutes(30), Duration::seconds(15), Duration::nanoseconds(123456)], qattribute::NONE);
  println!("timespan list: {}", timespan_list);

  let short_list=K::new_short_list(vec![12_i16, -7, 1440], qattribute::NONE);
  let string=K::new_string("kdbplus".to_string(), qattribute::UNIQUE);
  let symbol_list=K::new_symbol_list(vec![String::from("David"), String::from("Solomon")], qattribute::NONE);
  let datetime_list=K::new_datetime_list(vec![NaiveDate::from_ymd_opt(2017, 4, 9).unwrap().and_hms_milli_opt(16, 51, 30, 97).unwrap().and_local_timezone(Utc).unwrap(), NaiveDate::from_ymd_opt(2020, 11, 3).unwrap().and_hms_milli_opt(10, 20, 8, 103).unwrap().and_local_timezone(Utc).unwrap()], qattribute::SORTED);
  let long=K::new_long(530000_i64);
  let compound_list=K::new_compound_list(vec![short_list, string, symbol_list, long, datetime_list]);
  println!("{}", compound_list);

  let keys=K::new_int_list(vec![1, 2, 3], qattribute::SORTED);
  let values=K::new_symbol_list(vec![String::from("a"), String::from("bb"), String::from("ccc")], qattribute::NONE);
  let dictionary=K::new_dictionary(keys, values).unwrap();
  println!("dictionary: {}", dictionary);

  if let Err(original)=dictionary.flip(){
    println!("Woops, mistake! {}", original);
  }

  let keys=K::new_symbol_list(vec![String::from("fruit"), String::from("country"), String::from("weight")], qattribute::NONE);
  let values=K::new_compound_list(vec![
    K::new_symbol_list(vec![String::from("banana"), String::from("melon"), String::from("strawberry")], qattribute::NONE),
    K::new_string(String::from("PCN"), qattribute::NONE),
    K::new_float_list(vec![400.19, 652.0, 85.37], qattribute::NONE)
  ]);
  let dictionary=K::new_dictionary(keys, values).unwrap();
  let table=dictionary.flip().unwrap();
  println!("table: {}", table);

}

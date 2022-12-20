use kdbplus::qattribute;
use kdbplus::ipc::*;
use chrono::prelude::*;

fn main(){

  let mut string=K::new_string(String::from("something"), qattribute::NONE);
  string.as_mut_string().unwrap().push('!');
  println!("modified string: {}", string);

  let mut timestamp_list=K::new_timestamp_list(vec![NaiveDate::from_ymd_opt(2021, 3, 9).unwrap().and_hms_nano_opt(12, 5, 40, 67824).unwrap().and_local_timezone(Utc).unwrap()], qattribute::NONE);
  
  // Push timestamp
  timestamp_list.push(&NaiveDate::from_ymd_opt(2021, 3, 13).unwrap().and_hms_nano_opt(5, 47, 2, 260484387).unwrap().and_local_timezone(Utc).unwrap()).unwrap();
  timestamp_list.set_attribute(qattribute::SORTED);
  println!("modified timestamp list: {}", timestamp_list);

  // Pop timestamp
  let mut last=timestamp_list.pop_timestamp().unwrap();
  println!("popped timestamp: {}", last);
  println!("modified timestamp list: {}", timestamp_list);

  // Insert timestamp
  timestamp_list.insert(0, &NaiveDate::from_ymd_opt(2020, 4, 19).unwrap().and_hms_nano_opt(5, 40, 45, 850935582).unwrap().and_local_timezone(Utc).unwrap()).unwrap();
  println!("modified timestamp list: {}", timestamp_list);

  // Remove timestamp
  last=timestamp_list.remove_timestamp(1).unwrap();
  println!("removed timestamp: {}", last);
  println!("modified timestamp list: {}", timestamp_list);

  // Pop timestamp as `K`
  let last_k=timestamp_list.pop().unwrap();
  println!("popped timestamp: {}", last_k);
  println!("modified timestamp list: {}", timestamp_list);

  let keys=K::new_int_list(vec![0, 1, 2], qattribute::NONE);
  let values=K::new_date_list(vec![NaiveDate::from_ymd_opt(2000, 1, 9).unwrap(), NaiveDate::from_ymd_opt(2001, 4, 10).unwrap(), NaiveDate::from_ymd_opt(2015, 3, 16).unwrap()], qattribute::NONE);
  let mut q_dictionary=K::new_dictionary(keys, values).unwrap();
  println!("dictionary: {}", q_dictionary);

  // Try to insert wrong type element
  let _=q_dictionary.push_pair(&3, &String::from("woops"));
  println!("dictionary: {}", q_dictionary);

  // Add correct type element
  q_dictionary.push_pair(&3, &NaiveDate::from_ymd_opt(2020, 8, 9).unwrap()).unwrap();
  println!("modified dictionary: {}", q_dictionary);
  
}
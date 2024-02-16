/*
    There are many compound collection datatypes in std::collection module
    some of them are HashMap, BTreeMap, HashSet, BTreeSet; to use these they
    have to be imported.

    HashMap is a dictionary comprising of Key:Value pairs.
    Creating a Hashmap.
    let mut hash_map_id1 : HashMap<key-DtTyp, val-DtTyp> = HashMap::new();
        or
    let mut hash_map_id1 = HashMap::<key-DtTyp, val-DtTyp>::new();

    hash_map_id1.insert(k,v) - value is over-written if key exists else a new k:v pair is inserted.
    hash_map_id1.entry(k).or_insert(val) - Key:Value pair is inserted only if the key does not exist.
    hash_map_id1.get(key) - will return Option-Enum.

    to iterate over the elements inside HashMap use '&'
*/

use std::collections::HashMap;

pub fn demo_hm() {
    let mut _hm_1: HashMap<u8, String> = HashMap::new();
    let mut _hm_2 = HashMap::<String, u8>::new();

    _hm_1.insert(0, String::from("A"));
    _hm_1.insert(1, String::from("B"));
    _hm_1.insert(2, String::from("C"));
    _hm_1.insert(3, String::from("D"));
    _hm_1.insert(4, String::from("E"));

    _hm_2.entry(String::from("A")).or_insert(0);
    _hm_2.entry(String::from("B")).or_insert(1);
    _hm_2.entry(String::from("C")).or_insert(2);
    _hm_2.entry(String::from("D")).or_insert(3);
    _hm_2.entry(String::from("E")).or_insert(4);

    for kv in &_hm_2 {
        println!("Key: {:?}\nVal: {:?}", kv.0, kv.1);
    }

    let opt_val: Option<&u8> = _hm_2.get("C");
    match opt_val {
        Some(v) => println!("{:?}", v),
        None => println!("None"),
    }

    let opt_val: Option<&String> = _hm_1.get(&3);
    match opt_val {
        Some(v) => println!("{:?}", v),
        None => println!("None"),
    }
}

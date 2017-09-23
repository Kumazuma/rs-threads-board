extern crate chrono;
extern crate serde_derive;
extern crate mysql;
extern crate serde;
extern crate serde_json;
use self::serde::ser::{Serialize, Serializer, SerializeStruct};
use self::chrono::NaiveDate;
use mysql::prelude::*;
use std::sync::Arc;
use std::error::Error;
pub trait Model{
     fn get_threads_list(&mut self)->Vec<ThreadSummary>;
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadSummary{
    uid:i32,
    subject:String,
    opener_uid:i32,
    opener_nickname:String,
    created_datetime:NaiveDate
}

impl Model for mysql::PooledConn {
    fn get_threads_list(&mut self)->Vec<ThreadSummary>{
        let sql = "SELECT uid, subject, opener_uid, opener_nickname, created_datetime FROM tb_threads";
        let params:&[&ToValue] = &[];
        return self.prep_exec(sql,params).unwrap().map(|row|{
            let mut row = row.unwrap();
            let is_in_home:i32 = row.take("user_is_in_home").unwrap();
            let had_retired:i32 = row.take("user_had_retired").unwrap();
            ThreadSummary{
                uid:row.take("uid").unwrap(),
                subject:row.take("subject").unwrap(),
                opener_uid:row.take("opener_uid").unwrap(),
                opener_nickname:row.take("opener_nickname").unwrap(),
                created_datetime:row.take("created_datetime").unwrap()
            }
        }).collect();
    }
    // add code here
}
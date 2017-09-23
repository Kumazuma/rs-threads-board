extern crate chrono;
extern crate serde_derive;
extern crate mysql;
extern crate serde;
extern crate serde_json;
extern crate crypto;
use self::serde::ser::{Serialize, Serializer, SerializeStruct};
use self::chrono::NaiveDateTime;
use mysql::prelude::*;
use std::sync::Arc;
use self::crypto::digest::Digest;
use std::error::Error;
pub trait Model{
     fn get_threads_list(&mut self,offset:usize, count:usize)->Vec<Thread>;
     
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Thread{
    uid:i32,
    subject:String,
    opener:User,
    created_datetime:NaiveDateTime,
    recent_update_datetime:NaiveDateTime
}

impl Thread{
    pub fn get_subject(&self)->&str{
        self.subject.as_str()
    }
    pub fn get_uid(&self)->i32{
        self.uid
    }
    pub fn get_opener(&self)->&User{
        &self.opener
    }
    pub fn get_created_datetime(&self)->&NaiveDateTime{
        &self.created_datetime
    }
    pub fn get_recent_update_datetime(&self)->&NaiveDateTime{
        &self.recent_update_datetime
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    uid:i32,
    nickname:String,
    email:String,
    password:String
}
impl User {
    // add code here
    pub fn get_uid(&self)->i32{
        self.uid
    }
    pub fn get_nickname(&self)->&str{
        self.nickname.as_str()
    }
    pub fn get_email(&self)->&str{
        self.email.as_str()
    }
    pub fn get_password(&self)->&str{
        self.password.as_str()
    }
    pub fn get_gravatar_url(&self)->String{
        let mut md5 = crypto::md5::Md5::new();
        md5.input_str(self.email.as_str());
        format!("https://www.gravatar.com/avatar/{}?s=40", md5.result_str())
    }
}
impl Model for mysql::PooledConn {
    fn get_threads_list(&mut self,offset:usize, count:usize)->Vec<Thread>{
        let sql =format!("SELECT * FROM v_thread_list LIMIT {}, {}", offset, count);
        let params:&[&ToValue] = &[];
        return self.prep_exec(sql,params).unwrap().map(|row|{
            let mut row = row.unwrap();
     
            Thread{
                uid:row.take("uid").expect("uid"),
                subject:row.take("subject").expect("uid"),
                opener:User{
                    uid:row.take("opener_uid").expect("opener_uid"),
                    nickname:row.take("opener_nickname").expect("opener_nickname"),
                    email:row.take("opener_email").expect("opener_email"),
                    password:String::from("")
                },
                created_datetime:row.take("created_datetime").expect("created_datetime"),
                recent_update_datetime:row.take("recent_update").expect("recent_update")

            }
        }).collect();
    }
    // add code here
}
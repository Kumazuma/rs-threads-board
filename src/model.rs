extern crate chrono;
extern crate serde_derive;
extern crate postgres;
extern crate serde;
extern crate serde_json;
extern crate crypto;
use self::serde::ser::{Serialize, Serializer, SerializeStruct};
use self::chrono::NaiveDateTime;
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
        format!("https://www.gravatar.com/avatar/{}?s=24", md5.result_str())
    }
}
impl Model for postgres::Connection {
    fn get_threads_list(&mut self,offset:usize, count:usize)->Vec<Thread>{
        println!("{}",line!());
        let sql =format!("SELECT * FROM v_thread_list LIMIT {} OFFSET {}", offset, count);
        //let params:&[&ToValue] = &[];
        return  self.query(&sql,&[]).expect("query error").iter().map(|row|{
            Thread{
                uid:row.get("uid"),
                subject:row.get("subject"),
                opener:User{
                    uid:row.get("opener_uid"),
                    nickname:row.get("opener_nickname"),
                    email:row.get("opener_email"),
                    password:String::from("")
                },
                created_datetime:row.get("created_datetime"),
                recent_update_datetime:row.get("recent_update")

            }
        }).collect();
    }
    // add code here
}
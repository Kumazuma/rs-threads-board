extern crate chrono;
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate crypto;
use self::serde::ser::{Serialize, Serializer, SerializeStruct};
use self::chrono::NaiveDateTime;

use std::sync::Arc;
use self::crypto::digest::Digest;
use std::error::Error;
pub enum ConditionUserFind{
    ByEMail(String),
    ByNickname(String)
}
pub enum ModelError{
    CollapseInsertData(String),
    IncorrectThread,
    IncorrectUser
}
pub trait Model{
     fn get_threads_list(&mut self,offset:usize, count:usize)->Vec<Thread>;
     fn get_user(&mut self,condition:ConditionUserFind)->Option<User>;
     fn add_new_user(&mut self, user:User)->Result<(), ModelError>;
     fn get_thread(&mut self, thread_uid:i32)->Option<ThreadBody>;
     fn add_new_comment(&mut self, thread_uid:i32, user:User, content:String)->Result<(), ModelError>;
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Comment{
    uid:i32,
    user:User,
    write_datetime:NaiveDateTime,
    content:String
}
impl Comment{
    pub fn new(uid:i32, user:User, write_datetime:NaiveDateTime, content:String)->Comment{
        Comment{
            uid:uid,
            user:user,
            write_datetime:write_datetime,
            content:content
        }
    }
    pub fn get_uid(&self)->i32{
        return self.uid;
    }
    pub fn get_user(&self)->&User{
        return &self.user;
    }
    pub fn get_writed_datetime(&self)->&NaiveDateTime{
        return &self.write_datetime;
    }
    pub fn get_content(&self)->&String{
        return &self.content;
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadBody{
    uid:i32,
    subject:String,
    open_datetime:NaiveDateTime,
    comments:Vec<Comment>
}
 impl ThreadBody{
    pub fn new(uid:i32, subject:String, open_datetime:NaiveDateTime, comments:Vec<Comment>)->ThreadBody{
        ThreadBody{
            uid:uid,
            subject:subject,
            open_datetime:open_datetime,
            comments:comments
        }
    }
    pub fn get_subject(&self)->&String{
        &self.subject
    }
    pub fn get_open_datetime(&self)->&NaiveDateTime{
        &self.open_datetime
    }
    pub fn get_comments(&self)->&Vec<Comment>{
        &self.comments
    }
    pub fn get_uid(&self)->i32{
        return self.uid;
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Thread{
    uid:i32,
    subject:String,
    opener:User,
    recent_update_datetime:NaiveDateTime
}

impl Thread{
    pub fn new(uid:i32, subject:String, recent_update_datetime:NaiveDateTime, opener:User)->Thread{
        Thread{
            uid:uid,
            subject:subject,
            recent_update_datetime:recent_update_datetime,
            opener:opener
        }
    }
    pub fn get_subject(&self)->&str{
        self.subject.as_str()
    }
    pub fn get_uid(&self)->i32{
        self.uid
    }
    pub fn get_opener(&self)->&User{
        &self.opener
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
    pub fn new(uid:i32, nickname:String, email:String, password:Option<String>)->User{
        User{
            uid:uid,
            nickname:nickname,
            email:email,
            password:password.unwrap_or(String::new())
        }
    }
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
    pub fn get_gravatar_url(&self, size:Option<u32>)->String{
        let mut md5 = crypto::md5::Md5::new();
        md5.input_str(self.email.as_str());
        if let None = size{
            return format!("https://www.gravatar.com/avatar/{}", md5.result_str());
        }
        else{
            return format!("https://www.gravatar.com/avatar/{}?s={}", md5.result_str(), size.unwrap());
        }
    }
}

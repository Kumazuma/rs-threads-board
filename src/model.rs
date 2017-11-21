extern crate chrono;
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate crypto;
extern crate mysql;
use mysql::prelude::*;
use self::serde::ser::{Serialize, Serializer, SerializeStruct};
use self::chrono::NaiveDateTime;

use std::sync::Arc;
use self::crypto::digest::Digest;
use std::error::Error;

use std::io::{Write;
use std::fmt::Display;
trait ToHtml{
    fn to_html(&self, writer:&mut Write)->Result<(), std::io::Error>;
}
impl<T:Display> ToHtml for Option<T>{
    fn to_html(&self, writer:&mut Write)->Result<(), std::io::Error>{
        if let &Some(ref v) = self{
            return write!(writer, "{}", v);
        }
        else{
            return write!(writer, "NULL");
        }
    }
}

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
     fn add_new_user(&mut self, user:User)->Result<(), ModelError>;
     fn get_thread(&mut self, thread_uid:i32)->Option<Thread>;
     fn add_new_comment(&mut self, thread_uid:i32, user:User, content:String)->Result<(), ModelError>;
     fn add_thread(&mut self, subject:&String, user:User,first_comment:&String)->Result<Thread,()>;
     fn get_comments(&mut self, thread_uid:i32)->Option<Vec<Comment>>;

}
#[derive(Serialize, Deserialize, Debug)]
pub struct Comment{
    uid:u32,
    user:User,
    write_datetime:NaiveDateTime,
    content:String
}
impl Comment{
    pub fn new(uid:u32, user:User, write_datetime:NaiveDateTime, content:String)->Comment{
        Comment{
            uid:uid,
            user:user,
            write_datetime:write_datetime,
            content:content
        }
    }
    pub fn get_uid(&self)->u32{
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
    pub fn get(conn:&mut mysql::PooledConn, uid:u32)->Option<Self>{
        let sql ="SELECT * FROM v_comments WHERE uid = ?";
        let params:&[&ToValue] = &[&uid];
        let row  = conn.first_exec(sql,params).unwrap();
        match row{
            Some(mut row)=>{
                let comment = Comment::new(
                    row.take("uid").expect("uid"),
                    User::new(
                        row.take("user_uid").expect("user_uid"),
                        row.take("user_nickname").expect("user_nickname"),
                        row.take("user_email").expect("user_email"),
                        None
                    ),
                    row.take("write_datetime").expect("write_datetime"),
                    row.take("comment").expect("comment")
                );
                return Some(comment);
            }, 
            None=>return None
        }
    }
    pub fn delete(self, conn:&mut mysql::PooledConn){
        let sql ="DELETE FROM tb_comments WHERE uid = ?";
        let params:&[&ToValue] = &[&self.uid];
        conn.first_exec(sql,params).unwrap();
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Thread{
    uid:i32,
    subject:String,
    opener:User,
    recent_update_datetime:NaiveDateTime,
    open_datetime:NaiveDateTime
}

impl Thread{
    pub fn new(uid:i32, subject:String, recent_update_datetime:NaiveDateTime, open_datetime:NaiveDateTime,  opener:User)->Thread{
        Thread{
            uid:uid,
            subject:subject,
            recent_update_datetime:recent_update_datetime,
            opener:opener,
            open_datetime:open_datetime
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
    pub fn get_open_datetime(&self)->&NaiveDateTime{
        &self.open_datetime
    }


    pub fn get(conn:&mut mysql::PooledConn, uid:u32)->Option<Self>{
        let sql ="SELECT * FROM v_thread_list WHERE uid = ?";
        let params:&[&ToValue] = &[&uid];
        let row  = conn.first_exec(sql,params).unwrap();
        match row{
            Some(mut row)=>{
                let thread = Thread::new(
                    row.take("uid").expect("uid"),
                    row.take("subject").expect("uid"),
                    row.take("recent_update").expect("recent_update"),
                    row.take("created_datetime").expect("created_datetime"),
                    User::new(
                        row.take("opener_uid").expect("opener_uid"),
                        row.take("opener_nickname").expect("opener_nickname"),
                        row.take("opener_email").expect("opener_email"),
                        None
                    )
                );
                return Some(thread);
            }, 
            None=>return None
        }
    }
    pub fn delete(self, conn:&mut mysql::PooledConn){
        let sql ="DELETE FROM tb_threads WHERE uid = ?";
        let params:&[&ToValue] = &[&self.uid];
        conn.first_exec(sql,params).unwrap();
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
    pub fn update(&self, conn:&mut mysql::PooledConn){
        let param:&[&ToValue] = &[&self.nickname, &self.password, &self.uid];
        conn.prep_exec("UPDATE tb_users SET nickname = ?, password = ? WHERE uid = ?",param);
    }
    pub fn find_by_email(conn:&mut mysql::PooledConn, email:&str)->Option<Self>{
        let param:&[&ToValue] = &[&email];
        let res = conn.first_exec("SELECT * FROM tb_users WHERE email = ?", param).unwrap();
        if let Some(mut v)=res{
            let res;
            res = User::new(
                v.take("uid").unwrap(),
                v.take("nickname").unwrap(),
                v.take("email").unwrap(),
                Some(v.take("password").unwrap())
            );
            return Some(res);
        }
        return None;
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag{
    name:String,
    threads:Option<Vec<Thread>>,
    thread_count:Option<usize>
}
impl Tag{
    pub fn new(name:String)->Tag{
        return Tag{
            name:name,
            threads:None,
            thread_count:None
        };
    }
    pub fn with_threads(mut self, threads:Vec<Thread>)->Self{
        self.threads = Some(threads);
        return self;
    }
    pub fn with_thread_count(mut self, count:usize)->Self{
        self.thread_count = Some(count);
        return self;
    }
    
    pub fn get_name(&self)->&String{
        return &self.name;
    }
    pub fn get_threads(&self)->&Option<Vec<Thread>>{
        return &self.threads;
    }
    pub fn get_thread_count(&self)->usize{
        if let Some(v) = self.thread_count{
            return v;
        }
        else if let Some(ref v) =self.threads{
            return v.len();
        }
        else{
            return 0;
        }
    }
}


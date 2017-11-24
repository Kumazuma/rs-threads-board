extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate crypto;
extern crate mysql;
use mysql::prelude::*;
use self::serde::ser::{Serialize, Serializer, SerializeStruct};
use self::chrono::NaiveDateTime;

use std::io::{Write,Error};
use std::io::Result as IOResult;
use std::fmt::Display;

use user::User;

pub trait ToHTML{
    fn to_html(&self, writer:&mut Write)->IOResult<()>;
}

impl<T:Display> ToHTML for Option<T> {
    fn to_html(&self, out: &mut Write) -> IOResult<()> {
        if let &Some(ref v) = self{
            let mut buf = Vec::new();
            use std::io::Write;
            write!(buf, "{}", v);
            return out.write_all(&buf.into_iter().fold(Vec::new(), |mut v, c| {
                match c {
                    b'<' => v.extend_from_slice(b"&lt;"),
                    b'>' => v.extend_from_slice(b"&gt;"),
                    b'&' => v.extend_from_slice(b"&amp;"),
                    c => v.push(c),
                };
                v
            }));
        }
        return Ok(());
        
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
                    User::new()
                        .uid(row.take("user_uid").expect("user_uid"))
                        .nickname(row.take("user_nickname").expect("user_nickname"))
                        .email(row.take("user_email").expect("user_email")),
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
    pub fn list(conn:&mut mysql::PooledConn, thread_uid:u32)->Vec<Self>{
        let sql ="SELECT * FROM v_comments WHERE thread_uid = ?";
        let params:&[&ToValue] = &[&thread_uid];
        let comments:Vec< _ >  = conn.prep_exec(sql,params).unwrap().map(|row|{
            let mut row = row.unwrap();
            Comment::new(
                row.take("uid").expect("uid"),
                User::new()
                .nickname(row.take("user_nickname").expect("user_nickname"))
                    .uid(row.take("user_uid").expect("user_uid"))
                    .email(row.take("user_email").expect("user_email")),
                row.take("write_datetime").expect("write_datetime"),
                row.take("comment").expect("comment")
            )
        }).collect();
        return comments;
    }
    pub fn upload(conn:&mut mysql::PooledConn, thread:&Thread, user:&User, comment:&str){
        let mut stmt = conn.prepare(r"INSERT INTO tb_comments
                                       (thread_uid, writer_uid, write_datetime, comment)
                                   VALUES
                                       (:thread_uid, :writer_uid, NOW(), :comment)").unwrap();
        stmt.execute(params!{
            "thread_uid" => thread.get_uid(),
            "writer_uid" => user.get_uid(),
            "comment" => comment,
        });
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
    
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Tag{
    name:String,
    thread_count:Option<usize>,
}
impl Tag{
    pub fn new(name:String)->Tag{
        return Tag{
            name:name,
            thread_count:None
        };
    }
    pub fn with_thread_count(mut self, count:usize)->Self{
        self.thread_count = Some(count);
        return self;
    }
    
    pub fn get_name(&self)->&String{
        return &self.name;
    }
    pub fn get_thread_count(&self)->usize{
        if let Some(v) = self.thread_count{
            return v;
        }
        return 0;
    }
    pub fn get_thread_list(&self, conn:&mut mysql::PooledConn)->Vec<Thread>{
        let sql = "SELECT thread_uid FROM v_tags where tag_name=?";
        let params:&[&ToValue] = &[&self.name];

        let mut threads:Vec<Thread> = Vec::new();
        let thread_uids:Vec<u32> = 
        conn.prep_exec(sql,params).unwrap().map(|row|{
            let row = row.unwrap();
            //eprintln!("{:?}",row);
            return row.get(0).unwrap();
        }).collect();
        //.into_iter()
        
        for uid in thread_uids{
            threads.push(match Thread::get( conn, uid){
                Some(v)=>v,
                None=>continue
            });
        }
        return threads;
    }
}


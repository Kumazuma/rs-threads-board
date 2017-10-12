extern crate mysql;
use mysql::prelude::*;

use model::*;

impl Model for mysql::PooledConn {
    fn get_threads_list(&mut self,offset:usize, count:usize)->Vec<Thread>{
        let sql =format!("SELECT * FROM v_thread_list LIMIT {}, {}", offset, count);
        let params:&[&ToValue] = &[];
        return self.prep_exec(sql,params).unwrap().map(|row|{
            let mut row = row.unwrap();
            Thread::new(
                row.take("uid").expect("uid"),
                row.take("subject").expect("uid"),
                row.take("recent_update").expect("recent_update"),
                User::new(
                    row.take("opener_uid").expect("opener_uid"),
                    row.take("opener_nickname").expect("opener_nickname"),
                    row.take("opener_email").expect("opener_email"),
                    None
                )
            )
        }).collect();
    }
    fn get_user(&mut self,condition:ConditionUserFind)->Option<User>{
        let res;
        res = match condition{
            ConditionUserFind::ByEMail(val)=>{
                let param:&[&ToValue] = &[&val];
                self.first_exec("SELECT * FROM tb_users WHERE email = ?", param).unwrap()
            },
            ConditionUserFind::ByNickname(val)=>{
                let param:&[&ToValue] = &[&val];
                self.first_exec("SELECT * FROM tb_users WHERE nickname = ?", param).unwrap()
            }
        };
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
    fn add_new_user(&mut self, user:User)->Result<(), ModelError>{
        let mut stmt = self.prepare(r"INSERT INTO tb_users
                                       (email, nickname, password)
                                   VALUES
                                       (:email, :nickname, :password)").unwrap();
        if let Err(e) = stmt.execute(params!{
            "email" => user.get_email(),
            "nickname" => user.get_nickname(),
            "password" => user.get_password(),
        }){
            return Err(ModelError::CollapseInsertData(String::from("E-Mail")));
        }
        return Ok(());
    }
    fn get_thread(&mut self, thread_uid:i32)->Option<ThreadBody>{
        let sql =format!("SELECT * FROM v_thread_list WHERE uid = ? LIMIT 1");
        let params:&[&ToValue] = &[&thread_uid];
        let mut thread = match self.first_exec(sql, params).unwrap(){
            Some(v)=>v,
            None=>return None
        };
        let sql ="SELECT * FROM v_comments WHERE thread_uid = ?";
        let comments:Vec< _ >  = self.prep_exec(sql,params).unwrap().map(|row|{
            let mut row = row.unwrap();
            Comment::new(
                row.take("uid").expect("uid"),
                User::new(
                    row.take("user_uid").expect("user_uid"),
                    row.take("user_nickname").expect("user_nickname"),
                    row.take("user_email").expect("user_email"),
                    None
                ),
                row.take("write_datetime").expect("write_datetime"),
                row.take("comment").expect("comment")
            )
        }).collect();
        let res = ThreadBody::new(
            thread_uid,
            thread.take("subject").expect("uid"),
            thread.take("created_datetime").expect("created_datetime"),
            comments
        );
        return Some(res);
    }
    // add code here
}
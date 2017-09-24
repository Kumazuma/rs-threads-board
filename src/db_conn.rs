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
    // add code here
}
extern crate mysql;
use mysql::prelude::*;
use std;
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
                row.take("created_datetime").expect("created_datetime"),
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
    fn get_thread(&mut self, thread_uid:i32)->Option<Thread>{
        let sql =format!("SELECT * FROM v_thread_list WHERE uid = ? LIMIT 1");
        let params:&[&ToValue] = &[&thread_uid];
        let mut thread = match self.first_exec(sql, params).unwrap(){
            Some(v)=>v,
            None=>return None
        };

        
        let res = Thread::new(
            thread.take("uid").expect("uid"),
            thread.take("subject").expect("uid"),
            thread.take("recent_update").expect("recent_update"),
            thread.take("created_datetime").expect("created_datetime"),
            User::new(
                thread.take("opener_uid").expect("opener_uid"),
                thread.take("opener_nickname").expect("opener_nickname"),
                thread.take("opener_email").expect("opener_email"),
                None
            )
        );
        return Some(res);
    }
    fn get_comments(&mut self, thread_uid:i32)->Option<Vec<Comment>>{
        let sql ="SELECT * FROM v_comments WHERE thread_uid = ?";
        let params:&[&ToValue] = &[&thread_uid];
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
        if comments.len() == 0{
            return None;
        }
        return Some(comments);
    }
    fn add_new_comment(&mut self, thread_uid:i32, user:User, content:String)->Result<(), ModelError>{
        let mut stmt = self.prepare(r"INSERT INTO tb_comments
                                       (thread_uid, writer_uid, write_datetime, comment)
                                   VALUES
                                       (:thread_uid, :writer_uid, NOW(), :comment)").unwrap();
        if let Err(e) = stmt.execute(params!{
            "thread_uid" => thread_uid,
            "writer_uid" => user.get_uid(),
            "comment" => content,
        }){
            return Err(ModelError::CollapseInsertData(String::from("E-Mail")));
        }
        Ok(())
    }
    fn add_thread(&mut self, subject:&String, user:User,first_comment:&String)->Result<Thread, ()>{
        use mysql::IsolationLevel;
        let uid:i32;
        {
            let mut transaction = self.start_transaction(false, Some(IsolationLevel::Serializable), Some(false)).unwrap();
            
            
            /*
            `uid` INT(11) NOT NULL AUTO_INCREMENT,
        `opener_uid` INT(11) NOT NULL DEFAULT '0',
        `opener_nickname` VARCHAR(32) NOT NULL,
        `subject` VARCHAR(64) NOT NULL COLLATE 'utf8mb4_unicode_ci',
        `created_datetime` DATETIME NOT NULL,
            */
            let params:&[&ToValue] = &[&user.get_uid(), &user.get_nickname(), subject];
            transaction.prep_exec("INSERT INTO tb_threads (opener_uid, opener_nickname, subject, created_datetime) VALUES (?,?,?,now())",params);
            let row = transaction.first("SELECT LAST_INSERT_ID() FROM tb_threads").unwrap();
            let row = row.unwrap();
            uid = row.get(0).unwrap();
            transaction.prep_exec(r"INSERT INTO tb_comments
                                       (thread_uid, writer_uid, write_datetime, comment)
                                   VALUES
                                       (:thread_uid, :writer_uid, NOW(), :comment)",params!{
                "thread_uid" => uid,
                "writer_uid" => user.get_uid(),
                "comment" => first_comment,
            }).unwrap();
            transaction.commit();
            
        }
        return match self.get_thread(uid){
            Some(v)=>Ok(v),
            None=>Err(())
        };
    }
    // add code here
}

pub trait TagController{
    fn get(model:&mut mysql::PooledConn, name:&str)->Tag;
    fn put(&mut self,model:&mut mysql::PooledConn, thread:&Thread);
    fn delete(&mut self,model:&mut mysql::PooledConn, thread:&Thread);
}
impl TagController for Tag{
    
    fn get(model:&mut mysql::PooledConn, name:&str)->Tag{
        let sql = "SELECT thread_uid FROM tb_tags where tag_name=?";
        let params:&[&ToValue] = &[&name];

        let mut threads:Vec<Thread> = Vec::new();
        let thread_uids:Vec<i32> = 
        model.prep_exec(sql,params).unwrap().map(|row|{
            let row = row.unwrap();
            return row.get(0).unwrap();
        }).collect();
        //.into_iter()
        for uid in thread_uids{
            threads.push(match model.get_thread(uid){
                Some(v)=>v,
                None=>continue
            });
        }
        return Tag::new(name.to_string(), threads);
    }
    fn put(&mut self,model:&mut mysql::PooledConn, thread:&Thread){
        let sql = "INSERT INTO tb_tags VALUES (?, ?)";
        let param:&[&ToValue] = &[self.get_name(), &thread.get_uid()];
        model.prep_exec(sql, param).unwrap();
    }
    fn delete(&mut self,model:&mut mysql::PooledConn, thread:&Thread){
        let sql = "DELETE FROM tb_tags WHERE thread_uid = ?";
        let param:&[&ToValue] = &[&thread.get_uid()];
        model.prep_exec(sql, param).unwrap();
    }
}
extern crate mysql;
use mysql::prelude::*;
use std;
use model::*;
use user::*;
impl Tag{
    pub fn list(model:&mut mysql::PooledConn, q:Option<&str>)->Vec<Tag>{
        if let Some(v) = q{
            let sql = "SELECT * FROM v_tag_threads_count_list where tag_name LIKE ?";
            let params:&[&ToValue] = &[&format!("%{}%", v)];
            //let thread_uids:Vec<i32> = 
            model.prep_exec(sql,params).unwrap().map(|row|{
                let row = row.unwrap();
                return Tag::new(row.get(0).unwrap()).with_thread_count(row.get(1).unwrap());
            }).collect()
        }
        else{

            let sql = "SELECT * FROM v_tag_threads_count_list";
            let params:&[&ToValue] = &[];
            model.prep_exec(sql,params).unwrap().map(|row|{
                let row = row.unwrap();
                return Tag::new(row.get(0).unwrap()).with_thread_count(row.get(1).unwrap());
            }).collect()
        }
        
    }
    pub fn put(&self,model:&mut mysql::PooledConn, thread:&Thread){
        let sql = "INSERT INTO tb_tags VALUES (?, ?)";
        let param:&[&ToValue] = &[self.get_name(), &thread.get_uid()];
        model.prep_exec(sql, param).unwrap();
    }
    pub fn delete(&self,model:&mut mysql::PooledConn, thread:&Thread){
        let sql = "DELETE FROM tb_tags WHERE thread_uid = ?";
        let param:&[&ToValue] = &[&thread.get_uid()];
        model.prep_exec(sql, param).unwrap();
    }
}
impl Thread{
    pub fn upload(conn:&mut mysql::PooledConn, subject:&String, user:User,first_comment:&String)->Result<Self,()>{
        use mysql::IsolationLevel;
        let thread_uid:u32;
        let comment_uid:u32;
        {
            let mut transaction = conn.start_transaction(false, Some(IsolationLevel::Serializable), Some(false)).unwrap();
            let params:&[&ToValue] = &[&user.get_uid(),  subject];
            {
                let result = transaction.prep_exec("INSERT INTO tb_threads (opener_uid,  subject, created_datetime) VALUES (?,?,now())",params).unwrap();
                thread_uid = result.last_insert_id() as u32;
            }
            {
                let result = transaction.prep_exec(r"INSERT INTO tb_comments
                                        (thread_uid, writer_uid, write_datetime, comment)
                                    VALUES
                                        (:thread_uid, :writer_uid, NOW(), :comment)",params!{
                    "thread_uid" => thread_uid,
                    "writer_uid" => user.get_uid(),
                    "comment" => first_comment,
                }).unwrap();
                comment_uid = result.last_insert_id() as u32;
                
            }
            let params:&[&ToValue] = &[&comment_uid,  &thread_uid];
            transaction.prep_exec("UPDATE tb_threads SET first_comment = ? WHERE uid =?",params).unwrap();
            transaction.commit();
        }
        
        return match  Self::get(conn, thread_uid){
            Some(v)=>Ok(v),
            None=>Err(())
        };
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
                    User::new()
                        .uid(row.take("opener_uid").expect("opener_uid"))
                        .nickname(row.take("opener_nickname").expect("opener_nickname"))
                        .email(row.take("opener_email").expect("opener_email"))
                );
                return Some(thread);
            }, 
            None=>return None
        }
    }
    pub fn delete(self, conn:&mut mysql::PooledConn){
        use mysql::IsolationLevel;
        let mut transaction = conn.start_transaction(false, Some(IsolationLevel::Serializable), Some(false)).unwrap();
        let params:&[&ToValue] = &[&self.get_uid()];
        transaction.prep_exec("UPDATE tb_threads SET first_comment = NULL WHERE uid = ? ",params).unwrap();
        transaction.first_exec("DELETE FROM tb_threads WHERE uid = ?",params).unwrap();
        transaction.commit();
    }
    pub fn list(model:&mut mysql::PooledConn ,mut q: Option<String>, offset:usize, count:usize)->Vec<Thread>{
        //let sql =format!("SELECT * FROM v_thread_list WHERE subject like ?");
        let mut sql = String::from("SELECT * FROM v_thread_list ");
        use std::fmt::{ Write};
        return if let Some(ref q) = q{
            let params:&[&ToValue]= &[q];
            sql.push_str("WHERE INSTR(subject, ?)");

            write!(&mut sql, "LIMIT {}, {}", offset, count);
            model.prep_exec(sql,params).unwrap().map(|row|{
            let mut row = row.unwrap();
            Thread::new(
                row.take("uid").expect("uid"),
                row.take("subject").expect("uid"),
                row.take("recent_update").expect("recent_update"),
                row.take("created_datetime").expect("created_datetime"),
                User::new()
                .uid(row.take("opener_uid").expect("opener_uid"))
                .nickname(row.take("opener_nickname").expect("opener_nickname"))
                .email(row.take("opener_email").expect("opener_email"))
            )
            }).collect()
        }
        else{
            let params:&[&ToValue] = &[];
            write!(&mut sql, "LIMIT {}, {}", offset, count);
            model.prep_exec(sql,params).unwrap().map(|row|{
            let mut row = row.unwrap();
            Thread::new(
                row.take("uid").expect("uid"),
                row.take("subject").expect("uid"),
                row.take("recent_update").expect("recent_update"),
                row.take("created_datetime").expect("created_datetime"),
                User::new()
                .uid(row.take("opener_uid").expect("opener_uid"))
                .nickname(row.take("opener_nickname").expect("opener_nickname"))
                .email(row.take("opener_email").expect("opener_email"))
            )
            }).collect()
        };
    }
}
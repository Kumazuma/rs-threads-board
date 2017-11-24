extern crate serde;
extern crate serde_json;
extern crate crypto;
extern crate mysql;
use mysql::prelude::*;
use self::crypto::digest::Digest;
use std::io::Write;
#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    uid:Option<i32>,
    nickname:Option<String>,
    email:Option<String>,
    password:Option<String>
}
impl User {
    pub fn new()->Self{
        return Self{uid:None, nickname:None, email:None, password:None};
    }
    pub fn uid(mut self, uid:i32)->Self{
        self.uid = Some(uid);
        return self;
    }
    pub fn password(mut self, password:String)->Self{
        self.password = Some(password);
        return self;
    }
    pub fn email(mut self, email:String)->Self{
        self.email = Some(email);
        return self;
    }
    pub fn nickname(mut self, nickname:String)->Self{
        self.nickname = Some(nickname);
        return self;
    }
    // add code here
    pub fn get_uid(&self)->Option<&i32>{
        return self.uid.as_ref();
    }
    pub fn get_nickname(&self)->Option<&String>{
        return self.nickname.as_ref();
    }
    pub fn get_email(&self)->Option<&String>{
        return self.email.as_ref();
    }
    pub fn get_password(&self)->Option<&String>{
        return self.password.as_ref();
    }
    pub fn get_gravatar_url(&self, size:Option<u32>)->String{
        let mut s = Vec::new();
        if let Some(ref v) = self.email{
            let mut md5 = crypto::md5::Md5::new();
            md5.input_str(v.as_str());
            write!( s,"https://www.gravatar.com/avatar/{}", md5.result_str());
            if let Some(v) = size{
                write!( s,"?s={}",v);
            }
        } 
        return String::from_utf8(s).unwrap();
    }
    pub fn update(&self, conn:&mut mysql::PooledConn){
        if let None= self.uid{
            return;
        }
        let param:&[&ToValue] = &[&self.nickname, &self.password, &self.uid];
        conn.prep_exec("UPDATE tb_users SET nickname = ?, password = ? WHERE uid = ?",param);
    }
    pub fn find_by_email(&mut self, conn:&mut mysql::PooledConn)->Result<(), ()>{
        if let None = self.email{
            return Err(());
        }
        let res = conn.first_exec("SELECT * FROM tb_users WHERE email = :email",
        params!{
            "email"=>&self.email
        }).unwrap();
        if let Some(mut v)=res{
            self.uid = Some(v.take("uid").unwrap());
            self.nickname = Some(v.take("nickname").unwrap());
            self.email = Some(v.take("email").unwrap());
            self.password = Some(v.take("password").unwrap());
            return Ok(());
        }
        return Err(());
    }
    pub fn sign_up(mut self, conn:&mut mysql::PooledConn)->Result<User,()>{
        
        let mut stmt = conn.prepare(r"INSERT INTO tb_users
                                       (email, nickname, password)
                                   VALUES
                                       (:email, :nickname, :password);").unwrap();
        if let Ok(v) = stmt.execute(params!{
            "email" => self.get_email(),
            "nickname" => self.get_nickname(),
            "password" => self.get_password(),
        }){
            return Ok(self.uid(v.last_insert_id() as i32));
        }
        return Err(());
    }
}
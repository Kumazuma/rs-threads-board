extern crate serde;
extern crate serde_json;
extern crate base64;
use user::User;
use common::ServerSetting;
#[derive(Serialize, Deserialize, Debug)]
struct SignInfomation{
    uid:i32,
    email:String,
    nickname:String
}
pub struct Login{
    pub user:User
}
impl Login{
    pub fn try_login(user:User, password:&str)->Option<Login>{
        if user.get_password().unwrap() != password{
            return None;
        }
        return Some(Login{user:user});
    }
    pub fn token(token:&str, setting:&ServerSetting)->Option<User>{
        use crypto::aes::*;
        use crypto::blockmodes::*;
        use crypto::buffer::*;
        let mut decryptor = cbc_decryptor(KeySize::KeySize256, setting.aes_key.as_bytes(),setting.aes_iv.as_bytes(), PkcsPadding);
        let val = match base64::decode(token){
            Ok(v)=>v,
            Err(e)=>{
                eprintln!("{}",e);
                return None;
            }
        };
        let mut reader = RefReadBuffer::new(&val);
        let mut buffer:[u8;1024 * 4] = [0;1024*4];
        let mut len = 0usize;
        {
            let mut writer = RefWriteBuffer::new(buffer.as_mut());
            if let Err(e) = decryptor.decrypt(&mut reader,&mut writer,true){
                return None;
            }
            len = writer.position();
        }
        if let Ok(v) = serde_json::from_slice::<SignInfomation>(&buffer[0..len]){
            let s = User::new().uid(v.uid).email(v.email).nickname(v.nickname);
            return  Some(s);
        }
        return None;
    }
    pub fn get_token(&self, setting:&ServerSetting)->String{
        let s = SignInfomation{
            uid:*self.user.get_uid().unwrap() ,
            email:self.user.get_email().unwrap().clone(),
            nickname:self.user.get_nickname().unwrap().clone()
        };
        use crypto::aes::*;
        use crypto::blockmodes::*;
        use crypto::buffer::*;
        let mut encryptor = cbc_encryptor(KeySize::KeySize256, setting.aes_key.as_bytes(),setting.aes_iv.as_bytes(), PkcsPadding);
        let s = serde_json::to_vec(&s).unwrap();
        
        let mut reader = RefReadBuffer::new(&s);
        let mut buffer:[u8;1024 * 16] = [0;1024*16];
        let len;
        {
            let mut writer = RefWriteBuffer::new(buffer.as_mut());
            encryptor.encrypt(&mut reader,&mut writer,true).unwrap();
            len = writer.position();
            //println!("pos:{}, remain:{}",writer.position(), writer.remaining());
        }
        //eprintln!("{:?}",len);
        let r = base64::encode(&buffer[0..len]);
        //eprintln!("{:?}",r);
        return r;
    }
}
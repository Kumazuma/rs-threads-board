use super::rouille;
use super::mysql;
use common::ServerSetting;
use common::{error, to_sha3};
use common::ResponseContentType;
use login::Login;
use profile::view::*;
use user::User;
pub fn process(request:&rouille::Request, conn:&mut mysql::PooledConn, setting:&ServerSetting,ctype:ResponseContentType)->Option<rouille::Response>{
    router!(request,
    (GET) (/profile)=>{
        return Some(profile_view(ctype));
    },
    (PUT) (/profile)=>{
        let param =post_input!(request,{
            nickname:String,
            current_password:String,
            new_password:String,
            token:String  
        });
        let param = match param{
            Ok(v)=>v,
            Err( _ )=>{
                return Some(error("파라메터가 부정확합니다.",404));
            }
        };
        let user = match Login::token(&param.token,setting){
            Some(v)=>v,
            None=>{
                return Some(error("권한이 없습니다.",403));
            }
        };
        let mut nuser = User::new().email(user.get_email().unwrap().clone());
        if let Err(()) = nuser.find_by_email(conn){
            return Some(error("권한이 없습니다.",403));
        }
        if user.get_uid().unwrap() != nuser.get_uid().unwrap(){
            return Some(error("권한이 없습니다.",403));
        }

        let current_password = to_sha3(param.current_password.trim());
        let new_password = if param.new_password.trim().len() != 0{
            to_sha3(param.new_password.trim())
        } 
        else{
            current_password.clone()
        };
        if nuser.get_password().unwrap() == &current_password{
            let user = nuser.password(new_password).nickname(param.nickname);
            user.update(conn);
        }
        else{
            return Some(error("권한이 없습니다.",403));
        }
        return Some(ok_view(ctype));
    },
    _=>return None
    );
    return None;
}
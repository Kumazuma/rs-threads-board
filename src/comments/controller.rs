use super::rouille;
use super::mysql;
use common::ServerSetting;
use common::error;
use common::ResponseContentType;
use login::Login;
use model::Thread;
use comments::view::*;
use model::Comment;
pub fn process(request:&rouille::Request, conn:&mut mysql::PooledConn, setting:&ServerSetting,ctype:ResponseContentType)->Option<rouille::Response>{
    router!(request,
    (GET) (/threads/{uid:u32}/comments)=>{
        
        let r_etag:Option<&str> = match request.header("If-None-Match"){
            Some(v) if v.starts_with("\"") => Some(&v[1..v.len()-1]),
            Some(v)=>Some(v),
            _=>None
        };
        let etag = Comment::e_tag(conn, uid);

        match r_etag{
            Some(t) if t == &etag=>{
                
                return Some(rouille::Response::from_data("",vec![])
                .with_status_code(304)
                .with_etag_keep(etag));
            }
            _=>{
                let list = Comment::list( conn, uid);;
                return Some(comment_list_view(ctype, list).with_etag_keep(etag));
            }
        }
    },
    (POST) (/threads/{uid:u32}/comments)=>{
        let input = post_input!(request, {
            content:String,
            token:String  
        });
        let input = match input{
            Ok(v)=>v,
            Err( _ )=>{
                return Some(error("파라메터가 부정확합니다.",404));
            }
        };
        let user = match Login::token(&input.token,setting){
            Some(v)=>v,
            None=>{
                return Some(error("권한이 없습니다.",403));
            }
        };
        let thread = match Thread::get( conn, uid){
            Some(v)=>v,
            None=>return Some(error("스레드가 존재하지 않습니다.",404))
        };
        Comment::upload( conn,&thread,  &user, &input.content);
        return Some(comment_upload_view(ctype));
    },
    (DELETE)(/comments/{uid:u32})=>{
        let input = post_input!(request, {
            token: String
        });
        let input = match input{
            Ok(v)=>v,
            Err( _ )=>{
                return Some(error("파라메터가 부정확합니다.",404));
            }
        };
        let user = match Login::token(&input.token,setting){
            Some(v)=>v,
            None=>{
                return Some(error("권한이 없습니다.",403));
            }
        };
        let comment =  match Comment::get( conn, uid){
            None=>return Some(error("해당 코멘트는 존재하지 않습니다",404)),
            Some(v)=>v
        };
        if comment.get_user().get_uid() != user.get_uid(){
            return Some(error("권한이 없습니다.",403));
        }
        comment.delete( conn);
        return Some(comment_upload_view(ctype));
    },
    _=>return None
    );
    return None;
}
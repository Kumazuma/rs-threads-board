use super::rouille;
use super::serde_json;
use super::templates;
use common::{ ResponseContentType};
use model::Comment;

pub fn comment_list_view(content_type:ResponseContentType, comments:Vec<Comment>)->rouille::Response{
    return match content_type{
    ResponseContentType::Html=>{
        let mut s = Vec::with_capacity(1024 * 1024);
        templates::comments_view(&mut s,&comments).unwrap();
        rouille::Response::from_data("text/html;charset=utf-8", s)
    },
    ResponseContentType::Xml=>rouille::Response::html(""),
    ResponseContentType::Json=>{
        let v = try_or_400!(serde_json::to_vec(&comments));
        rouille::Response::from_data("application/json", v)
    }
    };
}
pub fn comment_upload_view(content_type:ResponseContentType)->rouille::Response{
    let v:Vec<u8> =b"{}".to_vec();
    rouille::Response::from_data("application/json", v)
}
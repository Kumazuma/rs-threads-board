use super::rouille;
use super::serde_json;
use super::templates;
use common::{ ResponseContentType};
use model::Tag;
use model::Thread;

pub fn tag_list_view(content_type:ResponseContentType, tag_list:Vec<Tag> )->rouille::Response{
    match content_type{
        ResponseContentType::Html | ResponseContentType::Xml=>{
            let mut buffer:Vec<u8> =vec![];
            templates::tags(&mut buffer,tag_list);
            return rouille::Response::from_data("text/html;charset=utf-8", buffer);
        },
        ResponseContentType::Json=>{
            let v = try_or_400!(serde_json::to_vec(&tag_list));
            return rouille::Response::from_data("application/json", v);
        }
    }
}
pub fn threadsin_tag_view(content_type:ResponseContentType, thread_list:Vec<Thread> )->rouille::Response{

}
pub fn ok_view(content_type:ResponseContentType)->rouille::Response{
    return rouille::Response::from_data("application/json", "{}");
}

use super::rouille;
use super::serde_json;
use super::templates;
use common::{ ResponseContentType};
use user::User;
pub fn profile_view(content_type:ResponseContentType)->rouille::Response{
    let mut s = Vec::new();
    templates::profile(&mut s).unwrap();
    rouille::Response::from_data("text/html; charset=utf-8", s)
}
pub fn ok_view(content_type:ResponseContentType)->rouille::Response{
    return rouille::Response::from_data("application/json", "{}");
}

extern crate mysql;
use mysql::prelude::*;
use std;
use model::*;


pub fn get_tags_in_thread(model: &mut mysql::PooledConn, thread:&Thread)->Vec<Tag>{
    let sql = "SELECT tag_name FROM tb_tags where thread_uid=?";
    let params:&[&ToValue] = &[&thread.get_uid()];

    return model.prep_exec(sql,params).unwrap().map(|row|{
        let row = row.unwrap();
        return Tag::new(row.get(0).unwrap());
    }).collect();
}
pub fn get_tags(model: &mut mysql::PooledConn)->Vec<Tag>{
    let sql = "SELECT DISTINCT tag_name FROM tb_tags";
    let params:&[&ToValue] = &[];

    return model.prep_exec(sql,params).unwrap().map(|row|{
        let row = row.unwrap();
        return Tag::new(row.get(0).unwrap());
    }).collect();
}
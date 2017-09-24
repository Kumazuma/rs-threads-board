$(document).ready(()=>{
    let ajax_args= {
        url:"/signin/check",
        method:"GET",
        Accept:"application/json",
        dataType:"json"
    };
    let on_done = (e)=>{
        if(e.is_signin == true){
            $("#sign-in-form").remove();
            
        }
    };
    $.ajax(ajax_args).done(done);
});

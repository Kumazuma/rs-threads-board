function checkSignin(){
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
    $.ajax(ajax_args).done(on_done);
}
$(document).ready(()=>{
    checkSignin();
    $("#sign-in-form").on("submit",(e)=>{
        let ajax_args = {
            url:e.currentTarget.action,
            method:"POST",
            Accept:"application/json",
            dataType:"json",
            data:$("#sign-in-form").serialize()
        };
        $.ajax(ajax_args).done((e)=>{
            //alert(e.msg);
            //window.location.href = "/";
            checkSignin();
        }).fail(( jqXHR, textStatus )=>{
            let json_res = jqXHR.responseJSON;
            alert(json_res.msg);
        });
        e.preventDefault();
    });
});

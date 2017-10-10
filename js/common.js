function checkSignin(){
    var nickname = document.cookie.replace(/(?:(?:^|.*;\s*)nickname\s*\=\s*([^;]*).*$)|^.*$/, "$1");
    var token = document.cookie.replace(/(?:(?:^|.*;\s*)token\s*\=\s*([^;]*).*$)|^.*$/, "$1");
    var gravatar = document.cookie.replace(/(?:(?:^|.*;\s*)gravatar\s*\=\s*([^;]*).*$)|^.*$/, "$1");
    if(token != ""){
        $("#sign-in-form").css("display","none");
        $("#user-info").css("display","");
        $("#user-gravatar").attr("src",gravatar.replace("s=24","s=34"));
        
    }
}
function signout(){
    document.cookie = "nickname=";
    document.cookie = "token=";
    document.cookie = "gravatar=";
    $("#sign-in-form").css("display","");
    $("#user-info").css("display","none");
}
$(document).ready(()=>{
    checkSignin();
    $("#btn-signout").on("click",(e)=>{
        signout();
    });
    $("#sign-in-form").on("submit",(e)=>{
        let ajax_args = {
            url:e.currentTarget.action,
            method:"POST",
            Accept:"application/json",
            dataType:"json",
            data:$("#sign-in-form").serialize()
        };
        $.ajax(ajax_args).done((e)=>{
            document.cookie = "nickname=" + e.nickname;
            document.cookie = "token=" + e.token;
            document.cookie = "gravatar=" + e.gravatar;
            checkSignin();
        }).fail(( jqXHR, textStatus )=>{
            
            let json_res = jqXHR.responseJSON;
            switch(json_res.code){
                case "ThereIsNotAccount":
                alert("계정이 존재하지 않습니다.");
                break;
                case "IncorrectPassword":
                alert("암호가 틀렸습니다.");
                break;
                case "InvalidParameter":
                alert("파라메터가 잘못되었습니다.");
                break;
            }
        });
        e.preventDefault();
    });
});

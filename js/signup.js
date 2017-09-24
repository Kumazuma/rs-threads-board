$(document).ready(()=>{
    $("#signup-form").on("submit",(e)=>{
        //console.log(e);
        let passwordConfirm = $("#signup-form #signup-password-confirm").val();
        let password = $("#signup-form #signup-password").val();
        if(password == passwordConfirm){
            let ajax_args = {
                url:e.currentTarget.action,
                method:"POST",
                Accept:"application/json",
                dataType:"json",
                data:$("#signup-form").serialize()
            };
            $.ajax(ajax_args).done((e)=>{
                alert(e.msg);
                window.location.href = "/";
            }).fail(( jqXHR, textStatus )=>{
                let json_res = jqXHR.responseJSON;
                alert(json_res.msg);
            });
            e.preventDefault();
            return false;
        }
        alert("재확인한 암호와 일치하지 않습니다. 다시 확인해 주십시요.");
        return false;
    });
    
    $("#signup-password-confirm").on("focusout",(e)=>{
        let passwordConfirm = $("#signup-form #signup-password-confirm").val();
        let password = $("#signup-form #signup-password").val();
        if(password != passwordConfirm){
            $(e.target).parent().parent().addClass("has-error");
        }
        else{
            $(e.target).parent().parent().removeClass("has-error");
        }
        
    });
    $("#signup-email").on("focusout",(e)=>{

    });
    $("#signup-nickname").on("focusout",(e)=>{
        
    });
});
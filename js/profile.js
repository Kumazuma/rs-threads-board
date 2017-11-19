$(document).ready(()=>{
    let nickname = getNickname();
    $("#profile-nickname").val(nickname);
    $("#profile-update").on("submit",(e)=>{
        let apiParameter = {
            nickname: $("#profile-nickname").val(),
            current_password: $("#profile-cur-password").val(),
            new_password: $("#profile-new-password").val(),
            token:getToken()
        };
        $.ajax("/profile",{
            method:"PUT",
            data:apiParameter,
            Accept:"application/json",
            dataType:"json",
        }).done((e)=>{
            document.cookie =`nickname=${encodeURIComponent($("#profile-nickname").val())};path=/`; 
            alert("완료되었습니다.");
            checkSignin();
        }).fail((a,b,c)=>{
            alert(a.status);
        });
        e.preventDefault();
    });
});
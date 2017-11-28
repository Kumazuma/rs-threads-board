$(document).ready(()=>{
    let nickname = getNickname();
    $("#profile-nickname").val(nickname);
    $("#profile-update").on("submit",(e)=>{
        e.preventDefault();
        if($("#profile-new-password").val() != $("#profile-new-password-confirm").val())
        {
            alert("새로 사용할 비밀번호를 다시 한 번 확인하세요.");
            return;
        }
        if( $("#profile-cur-password").val().trim() == "")
        {
            alert("프로필을 수정하려면 현재 비밀번호를 입력해 주세요.");
            return;
        }
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
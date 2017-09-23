$(document).ready(()=>{
    var query = document.querySelector;
    $.ajax("/users",{dataType:"json"}).done(function(res){
        window.users = res;
    });
    document.querySelector("#submit").addEventListener("click",function(){
        var idx = document.querySelector("input[name='user_idx']:checked").value;
        var user = window.users.filter((it)=>it.idx == idx)[0];
        var location = document.querySelector("input[name='location']").value;
        var date = new Date();
        date = date.toISOString();
        date = date.split("T")[0];
        var regular_shift =JSON.stringify({
            idx:0,
            date_start:date,
            date_end:null,
            location:location,
            user:user
        });
        var aparam = {
            url:"/regular",
            method:"PUT",
            data:{regular_shift:regular_shift}
        };
        $.ajax(aparam).done(function(res){
            alert("완료하였습니다.");
        });
        
    });
});
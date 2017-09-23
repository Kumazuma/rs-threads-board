$(document).ready(function(){
    $.ajax("/users",{dataType:"json"}).done(function(res){
        window.users = res;
    });
    $(document).on("click","#calender td",function(){
        $(this).toggleClass("shift-date");
    });

});
function register()
{
    var param = window.users.filter(function(user){
        return $("#"+user.idx + ":checked").length != 0;
    }).map((user)=>{
        return {
            user:user,
            status:"None"
        };
    });
    var dates =new Array();
    document.querySelectorAll("#calender td.shift-date").forEach(function(it){
        dates.push(it.id);
    },);
    param = JSON.stringify(param);
    dates = JSON.stringify(dates);
    $.ajax("/shift",{method:"PUT" , data:{list:param, dates:dates}, dataType:"json"}).done(function(res){
        alert("완료되었습니다.");
    }).fail(function(e){
        alert(e.responseJSON.msg);
    });
}
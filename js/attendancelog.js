$(document).ready(function(){
    $(document).on("click","#calender td",function(){
        $("#calender td").removeClass("click-date");
        $(this).addClass("click-date");
        $.ajax("/attendance/log/" + this.id,{dataType:"json"}).done(function(data){
            var list = $("#list tbody").html("");
            data.forEach(function(it){
                var t = document.querySelector('#table-row-list'),
                td = t.content.querySelectorAll("td");
                td[0].textContent = it.user.name;
                td[1].textContent = it.status;
                var clone2 = document.importNode(t.content, true);
                list.append(clone2);
            });
            var tds = $("#summary td");
            tds[0].textContent = data.length;
            tds[1].textContent = data.filter(function(it){
                return it.status == "출석";
            }).length;
            tds[2].textContent = 
            data.filter(function(it){
                return it.status == "불참(무단)" || it.status == "불참(연락)";
            }).
            map(function(it){
                return it.user.name;
            }).join(",");
        });
    });
});
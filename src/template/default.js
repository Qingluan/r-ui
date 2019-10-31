var timestamp = function (){
    var date = new Date();
    var timestamp = date.getTime();
    return timestamp;
}
var rpc = {
    invoke : function(arg) { 
        let a = JSON.stringify(arg);
        console.log(a);
        window.external.invoke(a); 
    },
    progress: function(arg){
        if ($("div#progressbar").attr("last") == null){
            $("div#progressbar").animate({
                left:"2%"
            },700);
        }
        setTimeout(function(){
            let last = $("div#progressbar").attr("last");
            let now = timestamp();
            if (now -last >= 1990){
                $("div#progressbar").animate({
                    left:"-3%"
                },1000);
                $("div#progressbar").attr("last",null);
            }
        },2000);
        $("div#progressbar").attr("last", timestamp());        
        $("div#progressbar > div#progressbar-now").animate({
            height: arg.progress + "%"
        },500 ,function(){
            $("div#progressbar").attr("height", arg.progress % 101 + "%");
        });
    },
    render : function(items) {
        let obj = JSON.parse(items);
        if (obj.progress != null){
            rpc.progress(obj)   
        }
        handle_json(obj);
        rpc.init();
    },
    render_list: function(items_str) {
        $(".list-group").remove();
        document.body.innerHTML += items_str;
        rpc.init();
    },
    init: function(){
        Ins = document.getElementsByTagName("input");
        for(i=0; i< Ins.length ; i++ ){
            if ($(Ins[i]).attr("listend") == null){
                Ins[i].addEventListener("input", function (evt) {
                    rpc.invoke({
                        tp: 'text',
                        content: this.value,
                        id: this.id
                    });
                })
                // Ins[i].listend = "input";
                $(Ins[i]).attr("listend", "input");
            }
            
        }
        Btns = document.getElementsByClassName("btn");
        for(i=0; i< Btns.length ; i++ ){
            if ($(Btns[i]).attr("listend") == null){
                Btns[i].addEventListener("click", function () {
                    rpc.invoke({
                        btn:{
                            content: this.value,
                            id: this.id
                        }
                    });
                })
                
                $(Btns[i]).attr("listend","btn");
            }
        }

        Listitems = document.getElementsByClassName("list-group-item");
        for(i=0; i< Listitems.length ; i++ ){
            if ($(Listitems[i]).attr("listend") == null){
                Listitems[i].addEventListener("click", function () {
                    click(this.id);
                })
                $(Listitems[i]).attr("listend","lclick");
            }
        }

        $(".list-group-item").dblclick(function(){
            dbclicklistener(this);
            click(this.id, 'db');
        });
        
    }
}

var handle_json = function(obj){
    console.log(obj);
}

var list_add = function(string){
    let list = document.getElementsByClassName("list-group-flush")[0];
    if (list != null){
        list.innerHTML =  '<li class="list-group-item" ><p>' + string+ '</p></li>' + list.innerHTML
    }
    
}

var list_add_all = function(ls){
    let list = document.getElementsByClassName("list-group-flush")[0];
    if (list != null){
        for(i=0; i < ls.length; i++ ){
            list.innerHTML = '<li class="list-group-item" ><p>' + ls[i] + '</p></li>' + list.innerHTML
        }
        
    }
}



var dbclicklistener = function(e){
    $(e).animate({
        "margin-left":"360px",
        "margin-top":"-500px",
        "opacity": 0.3
      }, 1000, function() {
        $(e).remove()
    });
}


function click(id, tp) {
    console.log("click id", id);
    let ele = document.getElementById(id);
    if (ele == null){
        return;
    }
    if (ele.id == null){
        ele.id = "no_id";
    }
    if (tp == null){
        rpc.invoke({
            tp:"btn",
            content:'click',
            id:ele.id
            }
        )
    }else{
        rpc.invoke({
            tp:"btn",
            content:tp,
            id:ele.id
            }
        )
    }   
}

window.onload = function() { rpc.init(); };

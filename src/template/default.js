
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


var handle_json = function(obj){
    console.log(obj);
}


var rpc = {
    invoke : function(arg) { window.external.invoke(JSON.stringify(arg)); },
    render : function(items) {
        let obj = JSON.parse(items);
        handle_json(obj);
    }
}

Ins = document.getElementsByTagName("input");
for(i=0; i< Ins.length ; i++ ){
    Ins[i].addEventListener("input", function (evt) {
        rpc.invoke({
            text:{
                content: this.value,
                id: this.id
            }
        });
    })
}
Btns = document.getElementsByClassName("btn");
for(i=0; i< Btns.length ; i++ ){
    Btns[i].addEventListener("click", function () {
        rpc.invoke({
            btn:{
                content: this.value,
                id: this.id
            }
        });
    })
}



function click(id) {
    let ele = document.getElementById(id);
    rpc.invoke({
        btn:{
            content:ele.value,
            id:ele.id
        }
    })
}
// Bts = document.getElementsByClassName("btn");
// for(i=0; i< Ins.length ; i++ ){
//     Ins[i].addEventListener(Ins[i].id, function (evt) {
//         rpc.invoke({
//             btn:{
//                 content: this.value,
//                 id: this.id
//             }
//         });
//     })
// }

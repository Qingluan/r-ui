// /* Rainbow v2.1.4 rainbowco.de | included languages: json */!function(e,t){"object"==typeof exports&&"undefined"!=typeof module?module.exports=t():"function"==typeof define&&define.amd?define(t):e.Rainbow=t()}(this,function(){"use strict";function e(){return"undefined"!=typeof module&&"object"==typeof module.exports}function t(){return"undefined"==typeof document&&"undefined"!=typeof self}function n(e){var t=e.getAttribute("data-language")||e.parentNode.getAttribute("data-language");if(!t){var n=/\blang(?:uage)?-(\w+)/,r=e.className.match(n)||e.parentNode.className.match(n);r&&(t=r[1])}return t?t.toLowerCase():null}function r(e,t,n,r){return(n!==e||r!==t)&&(n<=e&&r>=t)}function a(e){return e.replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/&(?![\w\#]+;)/g,"&amp;")}function o(e,t){for(var n=0,r=1;r<t;++r)e[r]&&(n+=e[r].length);return n}function i(e,t,n,r){return n>=e&&n<t||r>e&&r<t}function s(e){var t=[];for(var n in e)e.hasOwnProperty(n)&&t.push(n);return t.sort(function(e,t){return t-e})}function u(e,t,n,r){var a=r.substr(e);return r.substr(0,e)+a.replace(t,n)}function c(t,Prism){if(e())return global.Worker=require("webworker-threads").Worker,new Worker(__filename);var n=Prism.toString(),c=s.toString();c+=a.toString(),c+=r.toString(),c+=i.toString(),c+=u.toString(),c+=o.toString(),c+=n;var l=c+"\tthis.onmessage="+t.toString(),f=new Blob([l],{type:"text/javascript"});return new Worker((window.URL||window.webkitURL).createObjectURL(f))}function l(e){function t(){self.postMessage({id:n.id,lang:n.lang,result:a})}var n=e.data,r=new Prism(n.options),a=r.refract(n.code,n.lang);return n.isNode?(t(),void self.close()):void setTimeout(function(){t()},1e3*n.options.delay)}function f(){return(R||null===j)&&(j=c(l,Prism)),j}function d(e,t){function n(a){a.data.id===e.id&&(t(a.data),r.removeEventListener("message",n))}var r=f();r.addEventListener("message",n),r.postMessage(e)}function g(e,t,n){return function(r){e.innerHTML=r.result,e.classList.remove("loading"),e.classList.add("rainbow-show"),"PRE"===e.parentNode.tagName&&(e.parentNode.classList.remove("loading"),e.parentNode.classList.add("rainbow-show")),M&&M(e,r.lang),0===--t.c&&n()}}function m(e){return{patterns:C,inheritenceMap:S,aliases:T,globalClass:e.globalClass,delay:isNaN(e.delay)?0:e.delay}}function v(e,t){var n={};"object"==typeof t&&(n=t,t=n.language),t=T[t]||t;var r={id:A++,code:e,lang:t,options:m(n),isNode:R};return r}function p(e,t){for(var r={c:0},a=0,o=e;a<o.length;a+=1){var i=o[a],s=n(i);if(!i.classList.contains("rainbow")&&s){i.classList.add("loading"),i.classList.add("rainbow"),"PRE"===i.parentNode.tagName&&i.parentNode.classList.add("loading");var u=i.getAttribute("data-global-class"),c=parseInt(i.getAttribute("data-delay"),10);++r.c,d(v(i.innerHTML,{language:s,globalClass:u,delay:c}),g(i,r,t))}}0===r.c&&t()}function h(e){var t=document.createElement("div");t.className="preloader";for(var n=0;n<7;n++)t.appendChild(document.createElement("div"));e.appendChild(t)}function b(e,t){t=t||function(){},e=e&&"function"==typeof e.getElementsByTagName?e:document;for(var n=e.getElementsByTagName("pre"),r=e.getElementsByTagName("code"),a=[],o=[],i=0,s=n;i<s.length;i+=1){var u=s[i];h(u),u.getElementsByTagName("code").length?u.getAttribute("data-trimmed")||(u.setAttribute("data-trimmed",!0),u.innerHTML=u.innerHTML.trim()):a.push(u)}for(var c=0,l=r;c<l.length;c+=1){var f=l[c];o.push(f)}p(o.concat(a),t)}function w(e){M=e}function y(e,t,n){S[e]||(S[e]=n),C[e]=t.concat(C[e]||[])}function L(e){delete S[e],delete C[e]}function N(){for(var e=[],t=arguments.length;t--;)e[t]=arguments[t];if("string"==typeof e[0]){var n=v(e[0],e[1]);return void d(n,function(e){return function(t){e&&e(t.result,t.lang)}}(e[2]))}return"function"==typeof e[0]?void b(0,e[0]):void b(e[0],e[1])}function E(e,t){T[e]=t}var M,Prism=function Prism(e){function t(e,t){for(var n in h)if(n=parseInt(n,10),r(n,h[n],e,t)&&(delete h[n],delete p[n]),i(n,h[n],e,t))return!0;return!1}function n(t,n){var r=t.replace(/\./g," "),a=e.globalClass;return a&&(r+=" "+a),'<span class="'+r+'">'+n+"</span>"}function c(e){for(var t=s(p),n=0,r=t;n<r.length;n+=1){var a=r[n],o=p[a];e=u(a,o.replace,o["with"],e)}return e}function l(e){var t="";return e.ignoreCase&&(t+="i"),e.multiline&&(t+="m"),new RegExp(e.source,t)}function f(r,a,i){function c(e){return r.name&&(e=n(r.name,e)),p[w]={replace:m[0],"with":e},h[w]=y,!g&&{remaining:a.substr(y-i),offset:y}}function f(t){var a=m[t];if(a){var i=r.matches[t],s=i.language,c=i.name&&i.matches?i.matches:i,l=function(e,r,a){b=u(o(m,t),e,a?n(a,r):r,b)};if("string"==typeof i)return void l(a,a,i);var f,d=new Prism(e);if(s)return f=d.refract(a,s),void l(a,f);f=d.refract(a,v,c.length?c:[c]),l(a,f,i.matches?i.name:0)}}void 0===i&&(i=0);var d=r.pattern;if(!d)return!1;var g=!d.global;d=l(d);var m=d.exec(a);if(!m)return!1;!r.name&&r.matches&&"string"==typeof r.matches[0]&&(r.name=r.matches[0],delete r.matches[0]);var b=m[0],w=m.index+i,y=m[0].length+w;if(w===y)return!1;if(t(w,y))return{remaining:a.substr(y-i),offset:y};for(var L=s(r.matches),N=0,E=L;N<E.length;N+=1){var M=E[N];f(M)}return c(b)}function d(e,t){for(var n=0,r=t;n<r.length;n+=1)for(var a=r[n],o=f(a,e);o;)o=f(a,o.remaining,o.offset);return c(e)}function g(t){for(var n=e.patterns[t]||[];e.inheritenceMap[t];)t=e.inheritenceMap[t],n=n.concat(e.patterns[t]||[]);return n}function m(e,t,n){return v=t,n=n||g(t),d(a(e),n)}var v,p={},h={};this.refract=m},C={},S={},T={},x={},A=0,R=e(),k=t(),j=null;x={extend:y,remove:L,onHighlight:w,addAlias:E,color:N},R&&(x.colorSync=function(e,t){var n=v(e,t),r=new Prism(n.options);return r.refract(n.code,n.lang)}),R||k||document.addEventListener("DOMContentLoaded",function(e){x.defer||x.color(e)},!1),k&&(self.onmessage=l);var B=x;return B});
// Rainbow.extend("json",[{matches:{0:{name:"string",matches:{name:"constant.character.escape",pattern:/\\('|"){1}/g}}},pattern:/(\"|\')(\\?.)*?\1/g},{name:"constant.numeric",pattern:/\b(-?(0x)?\d*\.?[\da-f]+|NaN|-?Infinity)\b/gi},{name:"constant.language",pattern:/\b(true|false|null)\b/g}]);

var timestamp = function (){
    var date = new Date();
    var timestamp = date.getTime();
    return timestamp;
}
var registed_id = [];
var rpc = {
    invoke : function(arg) { 
        let a = JSON.stringify(arg);
        // console.log(a);
        window.external.invoke(a); 
    },
    log: function(msg, level) {
        rpc.invoke({
            tp: "log",
            id: level,
            content:msg
        })
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
            if (now -last >= 3990){
                $("div#progressbar").animate({
                    left:"-3%"
                },1000);
                $("div#progressbar").attr("last",null);
            }
        },4000);
        $("div#progressbar").attr("last", timestamp());        
        $("div#progressbar > div#progressbar-now").animate({
            height: arg.progress + "%"
        },500 ,function(){
            $("div#progressbar").attr("height", arg.progress % 101 + "%");
        });
    },
    render : function(items) {
        console.log(items);
        let b = Base64.decode(items);
        console.log(b);
        let obj = JSON.parse(b);
        if (obj.progress != null){
            rpc.progress(obj)   
        }
        // obj.content = (obj.content);
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
                if (registed_id.includes(Ins[i])){
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
            
        }
        Btns = document.getElementsByClassName("btn");
        for(i=0; i< Btns.length ; i++ ){
            if ($(Btns[i]).attr("listend") == null){
                if (registed_id.includes(Btns[i])){
                    Btns[i].addEventListener("click", function () {
                        rpc.invoke({
                            tp: 'btn',
                            content: this.value,
                            id: this.id
                            
                        });
                    })
                }
                
                $(Btns[i]).attr("listend","btn");
            }
        }

        Listitems = document.getElementsByClassName("list-group-item");
        for(i=0; i< Listitems.length ; i++ ){
            if ($(Listitems[i]).attr("listend") == null){
                if (registed_id.includes(Listitems[i])){
                    Listitems[i].addEventListener("click", function () {
                        click(this.id);
                    })
                    $(Listitems[i]).attr("listend","lclick");
                }
            }
        }

        $(".list-group-item").dblclick(function(){
            dbclicklistener(this);
            click(this.id, 'db');
            do_some("db", this.id)
        });
        
    }
}

var do_some = function(tp,id, content){
    console.log("empty do someing")
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
        // "margin-left":"360px",
        "margin-top":"-60px",
        "opacity": 0.3
      }, 500, function() {
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


#[macro_export]
macro_rules! ele {
    (L $id:tt $(( $($sub:tt)* )),*  ) => {
        {
            let mut contents = String::new();
            $(
                let e = ele!(@e  ( $($sub)* ));
                contents.push_str(&format!(r#"<li class="list-group-item" >{}</li>"#, e));
            )*
            format!(r#"<ul id="{}" class="list-group list-group-flush" >{}</ul>"#, $id , &contents)
        }
    };
    (@dom ($e:ident $id:tt $cls:tt $($style:tt)? )) => {
        {
            let mut pre = format!(r#"<{I} id="{id}"  class="{cls}""#, id=$id, cls=$cls, I= stringify!($e));
            $(pre = format!(r#"{} style="{}" ""#, pre, $style);)?
            pre = format!(r#"{pre}></{I}>"#, pre=pre, I= stringify!($e));
            pre
        }
    };
    // (@li () ) => {};
    
    (@e (P $text:tt $($style:tt)?  )) => {
        {
            let p_= ele!(@dom (p  "id-text" "text-p text" $($style)? ));
            p_.replace("><", &format!(">{}<", $text))
        }
    };
    (@e (I $id:tt $($style:tt)? )) => {
        ele!(@dom (input $id "text-input" $($style)? ))
    };
    (@e (B $id:tt $($style:tt)? )) => {
        {
            let mut pre = format!(r#"<button id="{id}"  class="btn btn-default""#, id=$id.replace(" ","-"));
            $(pre = format!(r#"{} style="{}" ""#, pre, $style);)?
            pre = format!(r#"{pre}>{name}</button>"#, pre=pre, name=$id, );
            pre
        }
    };
    ($divname:tt $(($ele:tt $($id:tt)* )),* ) => {
        {
            let mut html_ = String::new();
            
            $(
                let _t = ele!(@e ($ele  $($id)*) );
                html_.push_str(&_t);
            )*
            format!(r#"<div id="{}" class="container container-fluid" > {}</div>"#,$divname, &html_)
        }
    };
    ($divname:tt $(($ele:tt $id:tt)),* | $style:tt ) => {
        {
            let mut html_ = String::new();
            
            $(
                let _t = ele!(@e ($ele $id $style) );
                html_.push_str(&_t);
            )*
            format!(r#"<div id="{}" class="container-fluid" style="$style" >{}</div>"#, $divname , &html_)
        }
    }
}

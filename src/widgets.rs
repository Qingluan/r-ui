
#[macro_export]
macro_rules! ele {
    (@dom ($e:ident $id:tt $cls:tt $($style:tt)? )) => {
        {
            let mut pre = format!(r#"<{I} id="{id}"  class="{cls}""#, id=$id, cls=$cls, I= stringify!($e));
            $(pre = format!(r#"{} style="{}" ""#, pre, $style);)?
            pre = format!(r#"{pre}></{I}>"#, pre=pre, I= stringify!($e));
            pre
        }
    };
    (@e (P $id:tt $($style:tt)? )) => {
        ele!(@dom (p $id "text-p" $($style)? ))
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
            format!(r#"<div id="$divname" class="container" > {}</div>"#, &html_)
        }
    };
    ($divname:tt $(($ele:tt $id:tt)),* | $style:tt ) => {
        {
            let mut html_ = String::new();
            
            $(
                let _t = ele!(@e ($ele $id $style) );
                html_.push_str(&_t);
            )*
            format!(r#"<div id="{}" class="container" style="$style" >{}</div>"#, $divname , &html_)
        }
    }
}

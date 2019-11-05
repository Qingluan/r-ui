// #![feature(log_syntax)]
#[macro_export]

macro_rules! ele {
    // ((P)) => {
    //     let progressbar = include_str!("template/progress.html");
    //         progressbar
    // };
    (@ss  ) => {
        "".to_string()
    };
    // (@ss $pp:expr ) => {
    //     $pp.clone()
    // };
    (@ss $tag:ident $id:tt $($text:tt)? ) => {
        ele!(@e ($tag $id $($text)? ))
    };
    (@ss $text:expr ) => {
        $text;
    };
    (@ss $name:ident  $id:tt ( $txt:expr )  ) => {
        {

            let inner = $txt ;
            format!(r#"<{name} id="{id}">{text}</{name}>"#,name=stringify!($name), id=$id, text=inner)
        }
    };
    (@ss $name:ident  $id:tt ( $($txt:tt)* ) $($rest:tt)* ) => {
        {
            // log_syntax!($name, $id, "@ss");
            ele!(@element $name  $id ( $($txt)*)  $($rest)*  )
        }
    };
    (@ss  (  $($subs:tt)* )   )  => {
        {
            // log_syntax!($pp, "(@ss)");
            let mut b = String::new();
            b.push_str(& ele!(@ss  $($subs)* ) ) ;
            b
        }
    };
    // (@element ) => {
    //     $pp.to_string()
    // };
    (@element $name:ident $id:tt ($($text:tt)* )  $($rest:tt)*   ) => {
        {

            // let new_pp = format!("{}{}",$pp, "  ");
            // log_syntax!($pp, new_pp, "ele", $($text)*);
            let inner = ele!(@ss  $($text)*  );

            let mut b = format!(r#"<{name} id="{id}">{text}</{name}>"#,
                name=stringify!($name),
                id=$id,
                text=&inner);
            b.push_str(& ele!(@ss $($rest)* ) );
            b
        }
    };
    (RAW $name:ident $id:tt ( $($inner:tt)* )  $($rest:tt)* ) => {
        {

            ele!(@element  $name $id ($($inner)*) $($rest)*  )
        }
    };
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
    (@e (P $id:tt $($style:tt)? ) ) => {
        {
            r#"<div id="progressbar" class="pro-bar" style="height: 80%;width: 10px;background: white;
                                                left: -3%;
                                                top:20px;
                                                position: absolute;
                                                border: 2;
                                                border-radius: 8px;
                                                padding: 2px;
                                                box-shadow: aqua;
                                                border: 1px solid;
            "><div id="progressbar-now" class="pro-bar" style="bottom: 22;[[THEME]]height: 70%;border-radius: 9px;width: 100;"></div></div>"#
        }
    };
    (@e (M  $id:tt  $($text:tt)?  )) => {
        {
            let mut b = String::new();
            $(
                b.push_str($text);
            )?
            format!(r#"
<div class="modal" id="{ID}">
  <div class="modal-dialog">
    <div class="modal-content">
      <div class="modal-header" id="head-{ID}">
        <h4 class="modal-title" id="title-{ID}"></h4>
        <button type="button" class="close" data-dismiss="modal">&times;</button>
      </div>
      <div class="modal-body" id="body-{ID}">
      {TXT}
      </div>
      <div class="modal-footer" id="footer-{ID}">
        <button type="button" class="btn btn-danger" data-dismiss="modal">Close</button>
      </div>
    </div>
  </div>
</div>
            "#, ID=$id, TXT=&b)
        }
    };
    (@e (T $text:tt $($style:tt)?  )) => {
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
    ( $( ($ele:ident $id:tt )),* ) => {
        {
            let mut html_ = String::new();

            $(
                let _t = ele!(@e ($ele  $id ) );
                html_.push_str(&_t);
            )*
            // format!(r#"<div id="{}" class="container container-fluid" > {}</div>"#,$divname, &html_)
            html_
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

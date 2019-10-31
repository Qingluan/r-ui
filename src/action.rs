// use std::fmt::Display;

pub trait Click {
    fn click(&self, id:&str, content:&str);
    // fn drag(&self, id:&str, content:&str);
}

pub struct ListData {
    datas: Vec<String>,
    single_tmp:String,
}


impl Default for ListData{
    fn default()->Self{
        Self{
            single_tmp : r#"<li id="l[[id]]"  class="list-group-item" >[[DATA]]</li>"#.to_string(),
            datas: vec![],
        }
    }
    
}

impl ListData
{
    #[allow(dead_code)]
    fn render(&self, id:i32, c:&str)-> String{
        if self.single_tmp.contains("[[DATA]]"){
            self.single_tmp.clone().replace("[[DATA]]", c).replace("[[id]]", &id.to_string())
            
        }else{
            self.single_tmp.clone()
        }
    }
    #[allow(dead_code)]
    pub fn to_html(&self) -> String{
        let buf = String::from(r#"<ul id="list-data" class="list-group list-group-flush" >[[DATA]]</ul>"#);
        let v:Vec<String> = self.datas.iter().enumerate().map(|(i,x)| self.render(i as i32,&x)).collect();
        buf.replace("[[DATA]]", &v.join(""))
    }

    #[allow(dead_code)]
    fn add(&mut self, new:&str) -> usize{
        self.datas.push(new.to_string());
        self.datas.len()
    }

    #[allow(dead_code)]
    pub fn update(&mut self, new:&Vec<String>) -> usize{
        self.datas = new.clone();
        self.datas.len()
    }

    #[allow(dead_code)]
    fn remove(&mut self, id:i32) -> usize{
        if id  < self.datas.len() as i32{
            self.datas.remove(id as usize);
        }
        
        self.datas.len()
    }

}
// impl Click for ListData{

//     fn click(&self, id:&str, content:&str)
//     {

//     }
// }
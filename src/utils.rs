use walkdir::WalkDir;
use regex::Regex;
use std::fs::{metadata,Metadata};
use dirs;
use std::path::Path;
// type F = Fn(&str) -> bool;
// type H = Fn(&str);

// for entry in WalkDir::new("foo").into_iter().filter_map(|e| e.ok()) {
//     println!("{}", entry.path().display());
// }

pub struct FileSearcher {
    path: String, 
}


// pub trait FileSystem :Sized
// where F: Fn(&str) -> bool,
//       H: Fn(&str)
// {
//     fn ein(&self, filter: F) -> Vec<String>;
//     fn with(&self, filter:F, handler: H);
//     // fn match_with(&self, text:&str) -> bool;
//     fn re(&self) -> Option<Regex>;
//     fn info(&self) -> Option<Metadata>;
//     // fn to_regex(&self) -> impl FnOnce(&str)->bool;
// }



// impl FileSystem for FileSearcher
impl FileSearcher
{
    #[allow(dead_code)]
    pub fn info(&self)-> Option<Metadata>{
        metadata(&self.path).ok()
    }
    #[allow(dead_code)]
    pub fn re(&self) -> Option<Regex>
    {
        Regex::new(&self.path).ok()
    }

    #[allow(dead_code)]
    pub fn ein<F>(&self, filter: F ) -> Vec<String>
    where F: Fn(&str) -> bool + 'static
    {
        let mut files: Vec<String> = vec![];
        
        for entry in WalkDir::new(&self.path).into_iter().filter_map(|e| e.ok()) {
            // println!("{}", entry.path().display());
            let f = entry.path().to_str().unwrap().to_string();
            if filter(&f){
                files.push(f);
            }
        }
        files
    }
    #[allow(dead_code)]
    pub fn with<F,H>(&self, filter:F, handler: H)
    where F: Fn(&str) -> bool + 'static,
        H:Fn(&str)

    {
        for entry in WalkDir::new(&self.path).into_iter().filter_map(|e| e.ok()) {
            // println!("{}", entry.path().display());
            let f = entry.path().to_str().unwrap().to_string();
            if filter(&f){
                handler(&f);
            }
        }
    }

    // fn match_with(&self, text:&str) -> bool{
    //     if let Some(r) = self.re(){
    //         r.is_match(text)
    //     }else{
    //         false
    //     }
    // }

}

pub trait ToFils {
    fn as_file(&self) -> FileSearcher;
    fn re(&self) -> Regex;
    fn info(&self) -> Option<Metadata>;
    fn name(&self) -> &str;
}

impl <'a> ToFils for &'a str{
    fn name(&self) -> &str{
        Path::new(self).file_name().unwrap().to_str().unwrap()
    }

    fn as_file(&self) -> FileSearcher{
        if self.starts_with("~/"){
            FileSearcher{
                path:self.replace("~", dirs::home_dir().unwrap().to_str().unwrap())
            }
        }else{
            FileSearcher{
                path:self.to_string()
            }
        }
        
    }

    fn re(&self) -> Regex{
        Regex::new(self).unwrap()
    }

    fn info(&self)-> Option<Metadata>{
        metadata(self).ok()
    }
}

// impl <'a,F,H> FileSystem<F,H> for &'a str
// where F: Fn(&str) -> bool,
//       H: Fn(&str)
// {
//     fn info(&self)-> Option<Metadata>{
//         metadata(self).ok()
//     }

//     fn ein(&self, filter: F) -> Vec<String>
//     where F: Fn(&str) -> bool
//     {
//         self.to_searcher().ein(filter)
//     }

//     fn with(&self, filter:F, handler:H)
//     where F: Fn(&str) -> bool,
//         H: Fn(&str)
//     {
//         self.to_searcher().with(filter, handler);
//     }

//     fn re(&self) -> Option<Regex>{
//         self.to_searcher().re()
//     }
// }
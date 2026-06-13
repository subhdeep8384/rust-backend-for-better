pub struct  Store{

}

impl Store {
    pub fn  create_user(&self , name :String , email : String) -> String {
        print!("user create ");
        String::from("1")
    }
    pub fn create_website(&self) -> String{
        print!("create user called");
        String::from("1")
    }
    pub fn getUser(&self , name : String , email : String)-> String{
        print!("User found");
        String::from("subhdeep pal")
    }
}

fn main() {
    let user = User{
        id:1,
        name:"Ali".to_string(),
        last_name:"Hussaini".to_string()
    };
    let none_user = User::get(None);
    let some_user = User::get(Some(user));
    println!("{:?}",none_user);
    println!("{:?}",some_user)
    
}


#[derive(Debug)]
struct User{
    id: i32,
    name:String,
    last_name: String
}

impl User{
    fn get(user:Option<Self>)-> Option<Self>{
        if let Some(user)=user{
            return  Option::Some(Self { id: user.id, name: user.name.clone(), last_name: user.last_name.clone() });
        }
        None
    }

  
}
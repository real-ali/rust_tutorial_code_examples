fn main(){
    // take env value and turn in into Vec<String> Collection
    let mut args :Vec<String> = std::env::args().collect();
    if args.len()<=1{
        println!("");
    }
    else{
        args.remove(0);
        // create string buffer 
        let mut str_buffer = String::new();
        for arg in args{
            str_buffer.push_str(&arg.to_string());
            str_buffer.push_str(" ");
        }
        println!("{str_buffer}")
    }
}
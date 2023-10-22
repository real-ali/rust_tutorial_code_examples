fn main() {
   let res= division(5, 0);
   match res {
       Ok(res)=>{
        println!("{}",res)
       },
       Err(e)=>{
        println!("{}",e)
       }
   };
}

fn division(numer :i32,next: i32) -> Result<i32,String>{
    if next!=0{
        let dn = numer/next;
        return  Ok(dn);
    }
    Err("ٍError: Number divided by zero".to_string())
}
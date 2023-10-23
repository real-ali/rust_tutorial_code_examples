pub use MyResult::MyErr;
pub use MyResult::MyOk;
pub enum MyResult<T,E> {
     MyOk(T),
     MyErr(E)
}
impl <T,E> MyResult<T,E> {
    pub fn is_ok(&self,)->bool{
     match self {
        MyOk(_)=>true,
        MyErr(_)=>false
     }
    }

    pub fn is_err(&self,)->bool{
     match self {
        MyErr(_)=>true,
        MyOk(_)=>false,
      
     }
    }

    pub fn expect(self,msg:&str)->T{
     match self {
          MyOk(t)=>t,
          MyErr(_)=>panic!("{msg}"),
         
        
       }
    }
    
}


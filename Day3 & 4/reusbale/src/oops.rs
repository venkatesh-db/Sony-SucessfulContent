
pub struct Payment
{
    pub arr:[i32;5],
}

impl Payment
{
    pub fn Transaction(&self){

         println!(" transaction {:p}", &self.arr);
     }

}


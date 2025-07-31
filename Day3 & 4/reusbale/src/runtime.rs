
 pub trait Paymentreverse
{
    fn Retry(&self, nofr:i8);
}

pub struct ICIC
{
    noofretry:i8,
    transactionalert:bool,
}

 impl ICIC
{

   pub fn new(  noofr:i8 ,tran:bool )->ICIC{
   
     return  ICIC{
          noofretry:noofr,
          transactionalert:tran
      }

    }

}

 impl Paymentreverse for ICIC
{
     fn Retry(&self, nofr:i8)
     {
        if self.noofretry < 3 {
              
              println!("{}",self.noofretry);
        }

     }
}


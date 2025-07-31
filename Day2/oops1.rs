
trait Employees
{
  fn salary(&self);
  // virtual void salary();
}

struct govtjob
{
     basesalary:i32
}

impl  Employees for govtjob {

  fn salary(&self){
     println!("Employees.govtjob ")
  }

}

struct IT
{
       basesalary:i32
}

impl Employees for IT {

    fn salary(&self){
     println!("IT Employees ");
  }
}

fn copy( npci2:&IT ){

     println!("IT Employees  {} ",npci2.basesalary );
}

fn main(){

    let npci = IT{basesalary:190000 };
    npci.salary();

   // copy(npci,&npci);

copy(&npci);
// println!("main Employees {}",npci.basesalary);

}
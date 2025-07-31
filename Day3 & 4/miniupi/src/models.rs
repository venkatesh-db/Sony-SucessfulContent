use uuid::Uuid;
use chrono::Utc;

#[derive(Debug)]
pub struct User{
    pub id:Uuid,
    pub name:String,
    pub balance:f64,
}

impl User{

    pub fn new(name:&str,bal:f64)->Self{

        User{
            id:Uuid::new_v4(),
            name:name.to_string(),
            balance:bal,
        }

    }
}

#[derive(Debug)]
pub struct Transaction{
        pub id:Uuid,
        pub from:Uuid,
        pub to:Uuid,
        pub amount:f64,
        pub timestamp:String,
}

impl Transaction{


        pub fn new(from:Uuid ,to:Uuid ,amount:f64)->Self{
        Transaction{
            id:Uuid::new_v4(),
            from,  
            to,
            amount,
            timestamp:Utc::now().to_rfc3339(),
        }
    }

}
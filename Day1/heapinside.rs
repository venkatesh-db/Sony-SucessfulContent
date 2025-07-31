
#[derive(Debug)]
struct  Heapmem 
{
    smiles:i8,
    name:&'static str,
}

static javahero:Heapmem= Heapmem{ smiles:56, name:"smileing"};

fn main(){
    
    let stackobject = Heapmem{ smiles:56, name:"smileing"};
    let jamesbond = Box::new(Heapmem{ smiles:56, name:"sony smiles"} );  
    println!("{:?} {:?} ",stackobject, jamesbond);
    
}
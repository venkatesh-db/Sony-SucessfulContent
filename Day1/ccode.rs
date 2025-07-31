

smiles()
{
     auto int a = 5;
     printf("%u %d",&a,a);
}

cproblem( int *owner,int **borrower)
{
        printf("owner borrower address %u %u \n",&owner,&borrower);
      printf("cproblem %u %d \n",owner,*owner);
      printf("cproblem %u %u %d \n",borrower,*borrower,**borrower);
}

stack()
{
    
  int smiles=90;
  int * p = &smiles;
  printf("smiles  %u %d \n",&smiles,smiles);
  printf("pointer %u %u \n",&p,p);
  cproblem(p ,&p);
  
}



int main()
{
    smiles();
    stack();
}
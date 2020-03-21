import java.util.*;
 
class Main{
  public static void main(String[] args){
    Scanner sc=new Scanner(System.in);
    int n=sc.nextInt();
    int m=sc.nextInt();
    int[] a=new int[n];
    int p=0;
    for(int i=0;i<m;i++){
      int k=sc.nextInt();
      for(int j=0;j<k;j++){
        a[sc.nextInt()-1]+=pow(2,i);
      }
    }
    for(int i=0;i<m;i++){
      p+=sc.nextInt()*pow(2,i);
    }
    BAS bas=new BAS(n);
    int ans=0;
    boolean[] b=bas.a();
    do{
//      System.out.println(Arrays.toString(b));
      int sum=0;
      for(int i=0;i<n;i++){
        if(b[i]){
          sum=(sum^a[i]);
        }
      }
//      System.out.println(sum+":"+p);
      if(sum==p){
        ans++;
      }
    }while(bas.next());
    System.out.println(ans);
  }
  static int pow(int b,int p){
    int res=1;
    for(int i=0;i<p;i++){
      res*=b;
    }
    return res;
  }
  static class BAS{
    int n;
    boolean[] a;
    BAS(int n){
      this.n=n;
      a=new boolean[n];
    }
    boolean next(){
      for(int i=0;i<n;i++){
        if(a[i]){
          a[i]=false;
        }else{
          a[i]=true;
          return true;
        }
      }
      return false;
    }
    boolean[] a(){
      return a;
    }
  }
}

package Chapter10.Practice;
import java.util.*;

public class practice05{
    public static void main(String[] args){
        int data,baisuu=0,other=0;
        
        Scanner sc = new Scanner(System.in);

        while((data=sc.nextInt())!=0){
                if(data%3==0){
                    baisuu++;
                }else{
                    other++;
                }
        }
        System.out.println("3の倍数="+baisuu);
        System.out.println("その他="+other);
        
        sc.close();
    }
}
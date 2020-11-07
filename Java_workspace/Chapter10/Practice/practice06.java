package Chapter10.Practice;
import java.util.*;

public class practice06{
    public static void main(String[] args){
        int total=0,count=0,data;

        Scanner sc = new Scanner(System.in);

        while((data=sc.nextInt())!=0){
            if(data<100)
                total += data;
            else
                total += 100; count++;
        
        }
            System.out.println(total+ "/" +count);
        
            sc.close();
    }
}
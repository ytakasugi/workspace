package Chapter09.Test;
import java.util.*;

public class test03{
     public static void main(String[] args){
        int cmd;

        Scanner sc = new Scanner(System.in);
		
		do {
			System.out.println(Math.random());
			cmd = sc.nextInt();
			
		}while(cmd==1);

            sc.close();
		
    }

}
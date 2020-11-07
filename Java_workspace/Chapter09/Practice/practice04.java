package Chapter09.Practice;
import java.util.*;

public class practice04{
    public static void main(String[] args){
        int n, total=0;

        System.out.println("数値を入力してください");

        Scanner sc = new Scanner(System.in);
		
		do {
			n = sc.nextInt();
			total += n;
			
		}while(n>0);
		
        System.out.println("合計＝" +total);
        
        sc.close();
    }
}
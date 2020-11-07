package Chapter09.Practice;
import java.util.*;

public class practice03{

    public static void main(String[] args){

        int value, total=0, kensu=0;

        System.out.println("数値を入力してください");
        
        Scanner sc = new Scanner(System.in);

		while((value=sc.nextInt()) !=0) {
			total += value;
			kensu++;
		}	
			System.out.println("合計＝" + total);
            System.out.println("件数＝" + kensu);
            
            sc.close();

    }
}
package Chapter09.Test;
import java.util.*;

public class test02{
    public static void main(String[] args){
        double total=0, value;
        int kensu=0;

        System.out.println("数値を入力してください");
        
        Scanner sc = new Scanner(System.in);
		
		while((value=sc.nextDouble())!=0) {
			
			total += value;
			kensu++;
			
		}
		System.out.println("合計＝" +total);
		System.out.println("件数＝" +kensu);
        System.out.println("平均＝" +total/kensu );
        
        sc.close();
    }

}
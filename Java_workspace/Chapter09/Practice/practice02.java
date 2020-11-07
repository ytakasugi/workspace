package Chapter09.Practice;
import java.util.*;

public class practice02{
    public static void main(String[] args) {
        double x;

        System.out.println("数値を入力してください");

        Scanner sc = new Scanner(System.in);

        while((x=sc.nextDouble()) !=0) {
			System.out.println(Math.sqrt(x));
		}
        
            sc.close();

    }
}
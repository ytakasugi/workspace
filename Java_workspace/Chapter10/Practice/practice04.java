package Chapter10.Practice;
import java.util.*;

public class practice04 {
    public static void main(String[] args){
        int postage;
        Double weight;

        System.out.println("重さは？");

        Scanner sc = new Scanner(System.in);

        weight = sc.nextDouble();

        if(weight<1) {
			postage = 300;

		}else if(weight<5) {
			postage = 500;

		}else if(weight<10) {
			postage = 800;

		}else {
			postage = 1500;

		}

        System.out.println("料金は" +postage+ "です。");
        
        sc.close();

    }
}
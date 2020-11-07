package Chapter10.Practice;
import java.util.*;

public class practice03{
    public static void main(String[] args){
        int number;

        System.out.println("数字を入力してください");

        Scanner sc = new Scanner(System.in);

        number = sc.nextInt();

        if(number == 124){
            System.out.println("大当たり");

        }else if(number == 123 || number == 125){
            System.out.println("前後賞");

        }else{
            System.out.println("はずれ");

        }        
        
        sc.close();

    }
}

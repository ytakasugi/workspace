
package Chapter10.Practice;

import java.util.*;

public class practice01 {

    public static void main(String[] args){
        
        
        Scanner scanner = new Scanner(System.in);

        System.out.println("西暦を入力してください。");

        int year =scanner.nextInt();

        if(year%4==0 && year%100!=0 || year%400==0){
            System.out.println("うるう年です。");

        

        }else{
            System.out.println("うるう年ではありません。");

           
        }

        scanner.close();
    }
}



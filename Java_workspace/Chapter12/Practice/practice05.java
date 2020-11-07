package Chapter12.Practice;
import java.util.*;

public class practice05{
    public static void main(String[] args){
        Scanner sc = new Scanner(System.in);
        System.out.print("quantity:");
        int quantity = sc.nextInt();
        System.out.print("unit price:");
        int unitprice = sc.nextInt();

        sc.close();

        int amount = quantity * unitprice;
        double rate = discountrate(quantity);
        print(amount, rate);
        
    }
    public static double discountrate(int quantity){
        double rate;
        if(quantity<100){
            rate = 0;
        }else if(quantity<500){
            rate = 0.05;
        }else{
            rate = 0.1;
        }
        return rate;
    }
    public static void print(int amount, double rate){
        int discount = (int)(amount * rate);
        System.out.println("sales amount=" + amount + "yen");
        System.out.println("discount=" + discount + "yen");
        System.out.println("sales=" + (amount - discount) + "yen");
    }
}
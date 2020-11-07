package Chapter11.Test;
import java.util.*;
public class test01{
    public static void main(String[] args){
        System.out.print("How much is product?:");
        Scanner sc = new Scanner(System.in);
        int quantity =sc.nextInt();
        System.out.print("What is the product code?:");
        String code = sc.next();
        int price;
        switch(code){
        case    "a100":
        case    "b100":
            price = 100;
            break;
        case    "a110":
        case    "b110":
        case    "b120":
            price = 200;
            break;
        case    "c100":
        case    "c110":
            price = 300;
            break;
        case    "d100":
            price = 400;
            break;
        default:
            price = 500;
            break;
        }
        System.out.println("Total price=" + quantity * price + "yen");
        sc.close();
    }
}
package Chapter12.Practice;
import java.util.*;

public class practice03{
    public static void main(String[] args){
        Scanner sc = new Scanner(System.in);
        System.out.print("unit price:");
        int unitprice = sc.nextInt();
        System.out.print("tax rate:");
        double taxrate = sc.nextDouble();
        System.out.println("Tax Amount="+tax(unitprice, taxrate));
        sc.close();
    }
    public static int tax(int unitprice, double taxrate){
        return (int)(unitprice * taxrate);
    }
    
}
package Chapter12.Test;
import java.util.*;

public class test01{
    public static void main(String[] args){
        Scanner sc = new Scanner(System.in);
        System.out.print("How many miles?:");
        double mile = sc.nextDouble();
        sc.close();
        System.out.println(mile + "mile=" + mileTokm(mile) + "km");
    }
    public static double mileTokm(double mile){
        return mile * 1.609344;
    }
}
package Chapter12.Test;
import java.util.*;

public class test03{
    public static void main(String[] args){
        Scanner sc = new Scanner(System.in);
        System.out.print("What is the vartical length?:");
        double height = sc.nextDouble();
        System.out.print("What's the width?:");
        double width = sc.nextDouble();
        System.out.print("How long is the depth?:");
        double depth = sc.nextDouble();
        if(isOk(height,width,depth)){
            System.out.print("How much is it?:");
            double weight = sc.nextDouble();
            int fee = price(height,weight,depth,weight);
            System.out.println("fee=" + fee);
        }else{
            System.out.println("size over");
        }
        sc.close();
    }
    public static boolean isOk(double height, double width, double depth){
        return 180 >= height + width + depth;
    }
    public static int price(double height, double width, double depth, double weight){
        double length = height + width + depth;
        int fee;
        if(length<=90){
            if(weight<=5){
                fee = 500;
            }else if(weight<=10){
                fee = 1000;
            }else{
                fee = 1500;
            }
        }else{
            if(weight<=5){
                fee = 1000;
            }else if(weight<=10){
                fee = 2000;
            }else{
                fee = 3000;
            }
        }
        return fee;
    }
}
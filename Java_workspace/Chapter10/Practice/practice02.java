package Chapter10.Practice;
import java.util.*;

public class practice02{
    public static void main(String[] args){

        int nin,ryoukin;

        System.out.println("人数は？");

        Scanner sc = new Scanner(System.in);

        nin = sc.nextInt();

        ryoukin = nin * 850;

        if(nin>5)   ryoukin *= 0.7;

        System.out.println("料金は" + ryoukin + "円です。" );

        sc.close();
    }
}
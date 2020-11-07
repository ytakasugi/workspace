package Chapter10.Test;
import java.util.*;

public class test02{
    public static void main(String[] args){
        int dist,postage=0;

        System.out.println("距離は？");

        Scanner sc = new Scanner(System.in);

        dist = sc.nextInt();

        if(dist<50)             postage = 300;
        else if(dist<100)       postage = 500;
        else if(dist<500)       postage = 700;
        else                    postage = 1000;

        System.out.println("送料＝" + postage);
        sc.close();

    }
}

package Chapter15.Practice;
import java.util.Arrays;
//import java.util.Scanner;
import lib.Input;

public class example03 {
    public static void main(String[] args) {
        double[] data = new double[5];
        for(int i=0; i<data.length; i++) {
            //Scanner sc = new Scanner(System.in);
            //System.out.println("Value:");
            //double data = sc.nextDouble();
            //sc.close();
            data[i] = Input.getDouble("Value");
        }
        String list = Arrays.toString(data);
        System.out.println(list);
    }
}
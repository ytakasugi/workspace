package Chapter09.Practice;
import java.util.*;

public class practice01{
    public static void main(String[] args){
        int i=0;

        Scanner sc = new Scanner(System.in);

        while(i<3) {
			int n = sc.nextInt();
			System.out.println(n);
			
			i++;
		}
            sc.close();

    }
}
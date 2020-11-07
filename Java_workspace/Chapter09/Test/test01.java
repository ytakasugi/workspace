package Chapter09.Test;
import java.util.*;

public class test01{
    public static void main(String[] args){
        String str;

        System.out.println("文字列を入力してください");

        Scanner sc = new Scanner(System.in);

		while((str=sc.next())!=null) {
			System.out.print("<" +str.length() + ">");
			System.out.println(str);
		}
            sc.close();
    }

}
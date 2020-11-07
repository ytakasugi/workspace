package Chapter10.Test;
import java.util.*;

public class test01{

    public static void main(String[] args){

        String eword,jword;

        System.out.println("動物の種類は？");

        Scanner scanner = new Scanner(System.in);

        while((eword = scanner.next())!=null) {
			if(eword.equals("dog")){
					jword="いぬ";
		}else if(eword.equals("cat")){
					jword="ねこ";
		}else if(eword.equals("mouse")) {
					jword="ねずみ";
		}else if(eword.equals("rabbit")) {
					jword="うさぎ";
        }else{
            jword="?";
        
		    }

			System.out.println(jword);

        }
               
        scanner.close();

    }

}
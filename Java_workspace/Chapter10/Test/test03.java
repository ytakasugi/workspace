package Chapter10.Test;
import java.util.*;

public class test03{
    public static void main(String[] args){

        int month;
        String season;

        System.out.println("何月ですか？");

        Scanner sc = new Scanner(System.in);

        while((month = sc.nextInt())!=0){
            if(month==12||month<=2){
                season = "Winter";
            }else if(month<=5){
                season = "Spring";
            }else if(month<=8){
                season = "Summer";
            }else if(month<=11){
                season = "Autumn";
            }else{
                season = "?";
            }

            System.out.println("季節="+season);

        }
            sc.close();
    }
}
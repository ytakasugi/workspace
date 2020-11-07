package Chapter11.Practice;
import java.util.*;

public class practice01{
    public static void main(String[] args){
        String season;
        System.out.println("What month?:");
        Scanner sc = new Scanner(System.in);
        int month = sc.nextInt();
        switch(month){
        case    12:
            season = "Winter";
            break;
        case    1:
            season = "Winter";
            break;
        case    2:
            season = "Winter";
            break;
        case    3:
            season = "Spring";
            break;
        case    4:
            season = "Spring";
            break;
        case    5:
            season = "Spring";
            break;
        case    6:
            season = "Summer";
            break;
        case    7:
            season = "Summer";
            break;
        case    8:
            season = "Summer";
            break;
        case    9:
            season = "Autumn";
            break;
        case    10:
            season = "Autumn";
            break;
        case    11:
            season = "Autumn";
            break;
        default:
            season = "?";
         }
        System.out.println(season);
        sc.close();
    }
}
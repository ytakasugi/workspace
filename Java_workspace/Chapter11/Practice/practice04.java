package Chapter11.Practice;

public class practice04{
    public static void main(String[] args){
        String[] msg = {"Good", "Better", "Best", "Bad", "Worse", "Worst", "OK"};
        for(String s : msg){
            if(s.length()>4)
            continue;
            System.out.print(s);
        } 
    }
}
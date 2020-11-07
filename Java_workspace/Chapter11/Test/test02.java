package Chapter11.Test;

public class test02{
    public static void main(String[] args){
        int[] val = {10, -12, 5, -12, 12, 25};
        for(int value : val){
            if(value<0){
                System.out.println("Is it negative value");
                continue;
            }
            System.out.println(Math.sqrt(value));
        }
    }
}
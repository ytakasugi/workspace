package Chapter11.Practice;

public class practice03{
    public static void main(String[] args){
        double[] data = {2.5,3.3,7.0,-4.5,5.2};
        for(double x : data){
            if(x<0){
                System.out.println("Incorrect data:" +x);
                break;
            }
            System.out.println(Math.sqrt(x));
        }
    }
}
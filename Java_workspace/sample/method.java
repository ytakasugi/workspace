package sample;
public class method{
    public static void main(String[] args){
        System.out.println("***Calcuate BMI***");
        dispBmi(1.77, 60);
        System.out.println("***END***");
    }
    public static void dispBmi(double height, double weight){
        double bmi = weight / Math.pow(height, 2);
        System.out.println("BMI=" +bmi);
        return;
    }
}
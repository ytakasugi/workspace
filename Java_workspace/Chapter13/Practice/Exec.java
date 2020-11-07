package Chapter13.Practice;

public class Exec{
    public static void main(String[] args){
        Dice dice = new Dice();
        dice.val = 1;
        System.out.println("Number of eyes = " +dice.val);
        dice.play();
        System.out.println("Number of eyes = " +dice.val);

    }
}
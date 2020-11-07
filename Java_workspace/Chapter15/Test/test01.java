package Chapter15.Test;

public class test01 {
    public static void main(String[] args) {
        Dice dice1 = new Dice(6, "Black");
        Dice dice2 = new Dice(dice1.getVal(), dice1.getColor());
        dice2.play();
        System.out.println(dice1.getVal() + "/" + dice1.getColor());
        System.out.println(dice2.getVal() + "/" + dice2.getColor());
    }
}
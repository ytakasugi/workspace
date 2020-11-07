package Chapter13.Practice;

public class Dice{
    int val;
    public void play(){
        val = (int)(Math.random() * 6) +1 ;
    }
}
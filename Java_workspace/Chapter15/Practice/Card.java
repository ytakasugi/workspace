package Chapter15.Practice;

public class Card {
    private String suit;    //カードの種類
    private int number;     //カードの番号
    public void setSuit(String suit) {
        this.suit = suit;
    }
    public void setNumber(int number) {
        this.number = number;
    }
    public void disp(){
        System.out.println(suit + "/" + number + "\t");
    }
}

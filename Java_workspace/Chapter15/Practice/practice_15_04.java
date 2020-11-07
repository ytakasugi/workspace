package Chapter15.Practice;

public class practice_15_04 {
    public static void main(String[] args) {
        cardClass[] cards = {new cardClass(0,1), new cardClass(1,1), new cardClass(2,1) ,new cardClass(3,1)};
        for(cardClass card : cards) {
            System.out.println(card.toString() + "\t");
        }
    }
}
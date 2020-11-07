package Chapter15.Practice;

public class ExecuteCard {
    public static void main(String[] args){
        Card card1;
        Card card2 = new Card();
        card2.setSuit("Spade");
        card2.setNumber(1);
        card1 = card2;
        card1.setSuit("Diamond");
        card1.disp();
        card2.disp();
    }
}
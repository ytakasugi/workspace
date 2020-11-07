package Chapter15.Test;
import lib.Input;
public class test02 {
    public static void main(String[] args) {
        cardClass[] cards = new cardClass[5];
        for(int i=0; i<cards.length; i++){
            int cardType    = Input.getInt("Type(0～3)");
            int cardNum  = Input.getInt("Number(1～13)");
            cards[i] = new cardClass(cardType, cardNum);
        }
        for(cardClass card : cards){
            System.out.println(card.getCardTypeString()+card.getCardNum());
        }
    }
}
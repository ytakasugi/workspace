package Chapter14.Practice;
public class Card{
    String suit;    //カードの種類
    int number;     //カードの札番号
    public Card(String suit,int number){
        this.suit = suit;
        this.number = number;
    }
    public Card(String suit){
        this(suit,1);
    }
    public Card(int number){
        this("Spade",number);
    }
    public Card(){
        
    }
    public String face(){   //カードを表す文字列を返す
        return suit + "/" + number;
    }
}
package Chapter15.Practice;

public class example01 {
    public static void main (String[] args){
        Dice dice1 = new Dice();    //ダイス型オブジェクトの作成
        Dice dice2 = dice1;         //dice1をdice2に代入
        dice1.play();               //dice1の目数を変更する
        System.out.println("dice1.getVal() ="+dice1.getVal());
        System.out.println("dice2.getVal() ="+dice2.getVal());
    }
    
}
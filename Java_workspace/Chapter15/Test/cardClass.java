package Chapter15.Test;

public class cardClass {
    private int cardType;
    private int cardNum;

    public cardClass(int cardType, int cardNum) {
        this.cardType = cardType;
        this.cardNum = cardNum;
    }

    public cardClass(int a) {
        cardType = (a - 1) / 13;
        cardNum = (a - 1) % 13 + 1;
    }
    //カードの種類を返却する
    public int getCardType() {
        return cardType;
    }
    //カード番号を返却する
    public int getCardNum() {
        return cardNum;
    }
    public String getCardTypeString() {
        String[] typeName = {"スペード:", "ハート:", "クラブ:", "ダイヤ:",};
        return typeName[cardType];
    }
    public String CardNumString() {
        return cardNum < 10 ? " " + cardNum : "" + cardNum;
    }
    public int seqCardNum() {
        return 13 * cardType + cardNum;
    }
    public String toString() {
        return getCardTypeString() + CardNumString();
    }
    public boolean equals(Object o) {
        if((o instanceof cardClass) && (((cardClass)o).cardNum==cardNum) && ((cardClass)o).cardType==cardType)	return	true;
        else return false;
    }
    public int hashCode() {
        int h = 17;
        h = 31 * h + cardType;
        h = 31 * h + cardNum;

        return h;
    }
}
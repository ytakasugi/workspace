package Chapter15.Practice;
public class Dice{
    private int val;
    private String color;
    public Dice(int val,String color){
        this.val = val;
        this.color = color;      
    }
    public Dice(String color){
        this(1,color);
    }
    public Dice(){
        this(1,"white");
    }
    public void play(){
        val = (int)(Math.random()*6) + 1;
    }
    public int getVal(){
        return val;
    }
    public void setVal(int val){
        this.val = val;
    }
    public String getColor(){
        return color;
    }
    public void Color(String color){
        this.color = color;
    }
}
package Chapter12.Practice;
public class practice02{
    public static void main(String[] args){
        greet("Tanaka",1);
    }
    public static void greet(String name, int sex){
        if(sex==1){
            System.out.println("Hello"+name+"kun");
        }else{
            System.out.println("Hello"+name+"san");
        }
    }
}
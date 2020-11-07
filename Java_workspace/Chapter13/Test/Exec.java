package Chapter13.Test;

public class Exec{
    public static void main(String[] args){
        Denpyo denpyo_1 = new Denpyo();
        denpyo_1.date = "01-15";
        denpyo_1.item = "Computer";
        denpyo_1.price = 50000;
        denpyo_1.number = 1;

        Denpyo denpyo_2 = new Denpyo();
        denpyo_2.date = "01-16";
        denpyo_2.item = "Copy paper";
        denpyo_2.price = 600;
        denpyo_2.number = 5;

        denpyo_1.disp();
        denpyo_2.disp();
    }
}
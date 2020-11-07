package Chapter10.Test;

public class test04{
    public static void main(String[] args){
        String[] name = {"田中", "前田", "鈴木", "中村", "田辺", "浦川", "島田", "岩田"};
        int[] drinking = {7, 0, 3, 3, 2, 0, 0, 6};
        int[] smoking = {60, 10, 0, 20, 10, 0, 30, 0};
        String level = null;
        for(int i=0; i < name.length; i++){
            int d = drinking[i];
            int s = smoking[i];
            if(d==0 && s==0){
                level = "安全";
            }else if(d==0 && s <=20){
                level = "注意";
            }else if(d<=3 && s==0){
                level = "注意";
            }else if(d<=3 && s<=20){
                level = "要指導";
            }else{
                level = "要検査";
            }

            System.out.println(name[i] + "("+d+", "+s+")" + "\t" + level);

        }
    }
}
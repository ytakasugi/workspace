package Chapter15.Practice;
import java.util.Arrays;
import lib.Input;

public class practice_15_03 {
    public static void main (String[] args) {
        String[] names = new String[5];
        for(int i = 0; i<names.length; i++) {
            names[i] = Input.getString("name");
        }
        System.out.println(Arrays.toString(names));
    }
}
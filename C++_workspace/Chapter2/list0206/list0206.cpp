#include <iostream>

using namespace std;

int main()
{
    int n;
    cout << "Integer:";
    cin >> n;

    if (n > 0)
        //整数を2で割ったときに余りが0のとき
        if (n % 2 == 0)
            cout << "Its value is even. \n";
        //整数を2で割ったときに余りが0でないとき
        else
            cout << "Its value is odd. \n";
    //整数が正の値でないとき
    else
        cout << "\aYou entered a non-positive value. \n";
}
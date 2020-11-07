#include <ctime>
#include <cstdlib>
#include <iostream>

using namespace std;

int main() {
    // 乱数の設定
    srand(time(NULL));

    // 0～99の乱数を生成
    int no = rand() % 100;
    int x;

    // 数あてゲームの開始
    cout << "Stat Numbering game. \n";
    cout << "Plase apply a number from 0 to 99. \n";

    do {
        cout << "Please enter a number:";
        cin >> x;

        if(x > no)
            cout << "Its a smaller value. \n";
        else
            cout << "Its a greater value. \n";
    } while (x != no);

    cout << "Correct answer. \n";
}
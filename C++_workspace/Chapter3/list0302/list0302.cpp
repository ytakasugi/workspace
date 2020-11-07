#include <iostream>

using namespace std;

int main() {
    int hand;

    do {
        // 0・・・グー / 1・・・チョキ / 2・・・パー
        cout << "enter the following number :";
        cin >> hand;
    } while(hand < 0 || hand > 2); //変数handが0～2の範囲内か

    switch(hand) {
        case 0 : cout << "GOO \n";      break;
        case 1 : cout << "CHOKI \n";    break;
        case 2 : cout << "PAR \n";      break;
    }
}
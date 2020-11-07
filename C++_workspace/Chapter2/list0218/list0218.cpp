#include <iostream>

using namespace std;

int main()
{
    int hand;
    // 0・・・グー / 1・・・チョキ / 2・・・パー
    cout << "enter the following number :";
    cin >> hand;

    switch (hand) {
        case 0 : cout << "GOO \n";      break;
        case 1 : cout << "CHOKI \n";    break;
        case 2 : cout << "PAR \n";      break;
    }
}
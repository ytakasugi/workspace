#include <iostream>

using namespace std;

int main() {
    int x;

    cout << "Start the countdown. \n";
    do {
        cout << "Positive Integer value:";
        cin >> x;
    } while (x <= 0);

    while(x >= 0) {
        cout << x << "\n";
        x--;
    }
    cout << x << "\n";
}
#include <string>
#include <iostream>

using namespace std;

int main() {
    string retry;

    do {
        int month;
        cout << "Find the season. \nWhat month is it?: ";
        cin >> month;

        if(month >= 3 && month <= 5)
            cout << "It's spring. \n";
        else if(month >= 6 && month <= 8)
            cout << "It's summer. \n";
        else if(month >= 9 && month <= 11)
            cout << "It's autumn. \n";
        else if(month == 12 && month == 1 && month == 2)
            cout << "It's winter. \n";
        else
            cout << "There is no such month \n";

        cout << "One more? Y/N:";
        cin >> retry;
    } while (retry == "Y" || retry == "y");
}
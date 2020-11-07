#include <iostream>

using namespace std;

int main()
{
    int month;
    cout << "Find the season. please enter the month:";
    cin >> month;

    if (month >= 3 && month <= 5)
        cout << month << " is spring. \n";
    else if (month >= 6 && month <= 8)
        cout << month << " is summer. \n";
    else if (month >= 9 && month <= 11)
        cout << month << " is autumn. \n";
    else if (month == 12 || month == 1 || month == 2)
        cout << month << " is winter. \n";
    else
        cout << "There is no such month. \n";
}
#include <iostream>

using namespace std;

int main() 
{
    int y;
    cout << "Please Enter the year (ex.yyyy):";
    cin >> y;

    cout << "That year is";
    if(y % 4 == 0 && 100 != 0 || y % 400 == 0)
        cout << " a leap year. \n";
    else
        cout << " not a leap year. \n";
}
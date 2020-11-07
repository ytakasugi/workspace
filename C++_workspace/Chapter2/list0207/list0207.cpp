#include <iostream>

using namespace std;

int main()
{
    int n;
    cout << "Integer:";
    cin >> n;

    if (n == 0)
        cout << "Its value is Zero. \n";
    else if (n >= -9 && n <= 9)
        cout << "Its value is one digit. \n";
    else
        cout << "Its value is two or more digits. \n";
}
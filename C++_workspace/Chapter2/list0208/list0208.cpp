#include <iostream>

using namespace std;

int main()
{
    int n;
    cout << "Ineger:";
    cin >> n;

    if (n <= -10 || n >= 10)
        cout << "Its value is two or more digits. \n";
    else
        cout << "Its value is less than s digits. \n";
}
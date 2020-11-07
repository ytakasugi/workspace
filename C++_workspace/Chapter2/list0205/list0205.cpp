#include <iostream>
using namespace std;

int main()
{
    int n;
    cout << "Integer:";
    cin >> n;

    if (n > 0)
        cout << "Its value is positive. \n";
    else if (n < 0)
        cout << "Its value is negative. \n";
    else
        cout << "Its value ilists Zero. \n";
}
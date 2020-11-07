#include <iostream>

using namespace std;

int main()
{
    int a, b, c;

    cout << "Integer a:";   cin >> a;
    cout << "Integer b:";   cin >> b;
    cout << "Integer c:";   cin >> c;

    int max = a;
    if(b > max) max = b;
    if(c > max) max = c;

    cout << "Max value is " << max << ". \n";
}
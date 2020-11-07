#include <iostream>

using namespace std;

int main()
{
    int a, b;

    cout << "Integer a:";   cin >> a;
    cout << "Integer b:";   cin >> b;

    int min, max;

    if(a < b) {
        min = a;
        max = b;
    } else {
        min = b;
        max = a;
    }
    cout << "The smaller value is " << min << ". \n";
    cout << "The larger value is " << max << ". \n";
}
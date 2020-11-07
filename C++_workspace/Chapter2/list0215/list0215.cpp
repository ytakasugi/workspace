#include <iostream>

using namespace std;

int main()
{
    int a, b;

    cout << "Integer a:";   cin >> a;
    cout << "Integer b:";   cin >> b;

    if(a == b) {
        cout << "The two values are the same. \n";
    } else {
        if(a < b) {
            int t = a;
            a = b;
            b = t;
        }
        cout << "Sorted so that a > b. \n";
        cout << "Variable a is " << a << ". \n";
        cout << "Variable b is " << b << ". \n";
    }

}
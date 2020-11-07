#include <iostream>

using namespace std;

int main()
{
    // Declare variables a and b as int type.
    int a, b;

    // Prompt input for variables a and b, and read the input value.
    cout << "Integer a:";   cin >> a;
    cout << "Integer b:";   cin >> b;

    // If variable a is greater than variable b, swap those values.
    if(a > b) {
        int t = a;
        a = b;
        b = t;
    }
    cout << "Sorted so that a<=b. \n";
    cout << "Variable a is " << a << ". \n";
    cout << "Variable b is " << b << ". \n";
}
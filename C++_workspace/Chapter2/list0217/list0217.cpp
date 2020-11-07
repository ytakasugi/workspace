#include <cmath>
#include <iostream>

using namespace std;

int main()
{
    double x;

    cout << "Integer:";
    cin >> x;

    if(double m = fmod(x, 10)) {
        cout << "Its value is not divisible by 10. \n";
        cout << "The least significant digit is " << m << ". \n";
    } else {
        cout << "Its value is divisible by 10. \n";
    }
}
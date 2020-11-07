#include <iostream>

using namespace std;

int main()
{
    int n;

    cout << "Integer:";
    cin >> n;

    if(int mod = n % 10) {
        cout << "Its value is not divisible by 10. \n";
        cout << "The least significant digit is " << mod << ". \n";
    } else {
        cout << "Its value is divisible by 10. \n";
    }
}
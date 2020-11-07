#include <iostream>

using namespace std;

int main()
{
    int a, b;
    cout << "Integer a:"; cin >> a;
    cout << "Integer b:"; cin >> b;

    int min = a < b ? a : b;
    cout << "The smaller value is " << min << ". \n"; 
}
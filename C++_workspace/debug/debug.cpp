#include <iostream>
#include <vector>
#include <string>

using namespace std;

int main()
{

    vector<string> msg {"Alice", "and", "Saber", "very", "very", "cute!!"};

    for (const string& word : msg)
    {
        cout << word << " ";
    }
    cout << endl;
}
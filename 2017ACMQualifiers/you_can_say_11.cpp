#include<iostream>
#include<string>

using namespace std;

int main(){
    string isMult = " is a multiple of 11.";
    string isNotMult = " is not a multiple of 11.";
    int num;
    cin >> num;
    while( num ){
        if( num % 11 == 0 )
            cout << num << isMult << endl;
        else
            cout << num << isNotMult << endl;
        cin >> num;
        //cout << "Got: " << num << endl;
    }
    return 0;
}

#include<cstdio>
#include<iostream>

using namespace std;

int main(){
    int first, second;
    while( cin >> first >> second ){
        if(first == second)
            cout << first << endl;
        else
            cout << max(first, second) << endl;
    }
    return 0;
}

#include<iostream>
#include<sstream>
#include<stdio.h>
#include<string.h>
#include<stdlib.h>

using namespace std;

int main(void){
    long long int a,b;
    while (cin >> a >> b){
        cout << abs(a-b) << endl;
    }
    return 0;
}

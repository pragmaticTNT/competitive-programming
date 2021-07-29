#include<algorithm>
#include<iostream>
#include<stdio.h>
#include<stdlib.h>
#include<vector>

using namespace std;

int main(void){
    long long int r, m, s, count, inc, a;

    while (cin >> r >> m){
        s = 2*max(r,m);
        vector<bool> check(s, false);
        vector<long long int> pasta;
        
        pasta.push_back(r);
        inc = 1 + (r == 1);
        a = r;
        count = 1;

        check[a] = true;
        while (!check[m]){
            check[inc] = true;
            a += inc;
            
            if (a < s)
                check[a] = true;
            for (size_t i=0; i<pasta.size(); i++)
                if (a - pasta[i] < s)
                    check[a-pasta[i]] = true; 
            pasta.push_back(a);
            while (inc < s && check[inc])
                inc++;
            count++;
        }
        cout << count << endl;
    }
    return 0;
}

#include<iostream>
#include<algorithm>
#include<vector>

using namespace std;

int main(){
    int MAX_BLOCKS = 1001;
    bool pad[MAX_BLOCKS];
    vector<int> primes;
    int count = 2;

    fill_n(pad, MAX_BLOCKS, true);
    pad[0] = false; pad[1] = false;

    while( count < MAX_BLOCKS ){
        if( pad[count] ){
            primes.push_back(count);
            for( int i = count; i < MAX_BLOCKS; i += count ){
               pad[i] = false; 
            }
        }
        count++;
    }
    
    /***
    for( size_t i = 0; i < primes.size(); i++ ){
        cout << primes[i] << ' ';
    }
    cout << endl;
    ***/

    int nCase;
    cin >> nCase;
    for( int i = 0; i < nCase; i++ ){

    }
    
    return 0;
}

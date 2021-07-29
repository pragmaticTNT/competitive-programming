#include<cstdio>
#include<iostream>
#include<string>
#include<vector>

using namespace std;

int main(){
    int nGame, nDim;
    string rowStr;

    cin >> nGame;
    for( int i = 0; i < nGame; i++ ){
        cin >> nDim;
        vector< vector<bool> > bomb(nDim+2), seen(nDim);
        bool dead = false;

        // Input Bomb Grid: the extra space is so I don't have to check boundary cases
        for( int row = 0; row < nDim+2; row++ ){
            vector<bool> bombRow(nDim+2);
            if( row != 0 and row != nDim + 1){
                cin >> rowStr; 
                for( int col = 1; col <= nDim; col++ ){
                    rowStr[col-1] == '*' ? bombRow[col] = true : bombRow[col] = false;
                }
            }
            bomb[row] = bombRow;
        }

        /***
        for( size_t row = 0; row < bomb.size(); row++ ){
            for( size_t col = 0; col < bomb.size(); col++ ){
                cout << bomb[row][col];
            }
            cout << endl;
        }
        ***/

        // Input Seen Grid:
        for( int row = 0; row < nDim; row++ ){
            cin >> rowStr; 
            vector<bool> seenRow(nDim);
            for( int col = 0; col < nDim; col++ ){
                rowStr[col] == '.' ? seenRow[col] = false : seenRow[col] = true;
                if( seenRow[col] and bomb[row+1][col+1] )
                    dead = true;
            }
            seen[row] = seenRow;
        }
        
        // Output:
        for( int row = 1; row <= nDim; row++ ){
            for ( int col = 1; col <= nDim; col++ ){
                if( dead and bomb[row][col] ){
                    cout << "*";
                } else if( !seen[row-1][col-1] ){
                    cout << ".";
                } else{ 
                    cout << bomb[row-1][col-1] + bomb[row-1][col] +\ 
                            bomb[row-1][col+1] + bomb[row][col-1] +\
                            bomb[row][col+1] + bomb[row+1][col-1] +\ 
                            bomb[row+1][col] + bomb[row+1][col+1];
                }
            }
            cout << endl;
        }
        if( i != nGame - 1){ 
            cout << endl;
        }
    }
    return 0;
}

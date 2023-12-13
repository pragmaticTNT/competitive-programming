#include <bits/stdc++.h>
#include <algorithm>

using namespace std;

void solve(int test_case) {
    vector<int> output;
    int n;
    cin >> n;
    vector<bool> vis(41, true);

    while (any_of(vis.begin(), vis.end(), [](bool value){return value;})) {
        int total = n, remainder = 41;
        for (int i=41; i>0; i--) {
            if (vis[i-1]) {
                while (total%i == 0 && remainder >= i) {
                    output.push_back(i);
                    remainder -= i;
                    total /= i;
                }
            }
        }
        if (total == 1 && remainder == 0) 
            break;
        else {
            vis[output[0]-1] = false;
            output.clear();
            if (output[0] == 1)
                break;
        }
    }

    cout << "Case #" << test_case << ": ";
    if (output.size() == 0) 
        cout << "-1\n";
    else {
        cout << output.size();
        for (int val : output)
            cout << ' ' << val;
        cout << '\n'; 
    }
}

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    
    int tests;
    cin >> tests;
    for (int tc = 1; tc <= tests; tc++)
        solve(tc); 
}
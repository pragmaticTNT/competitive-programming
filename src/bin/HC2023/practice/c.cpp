#include <bits/stdc++.h>

using namespace std;

void solve(int test_case) {
    int n;
    cin >> n;
    cout << "Case #" << test_case << ": ";
    vector<int> apples;

    int apple;
    for (int i=0; i<n; i++) {
        cin >> apple;
        apples.push_back(apple);
    }
    sort(apples.begin(), apples.end());

    if (n == 1)       
        cout << apples[0] << "\n";
    else if (n == 2) {
        cout << apples[1] + apples[2] - apples[3] << "\n";
    } else {
        for (int i=0; i<n; i++) {
            cout << "-1\n";
            break;
        }
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
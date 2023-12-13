#include <bits/stdc++.h>

using namespace std;

void solve(int test_case) {
    int r, c, a, b;
    cin >> r >> c >> a >> b;
    cout << "Case #" << test_case << ": ";
    cout << ((r > c) ? "YES" : "NO") << '\n';
}

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    
    int tests;
    cin >> tests;
    for (int tc = 1; tc <= tests; tc++)
        solve(tc); 
}
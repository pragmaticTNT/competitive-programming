#include <bits/stdc++.h>

using namespace std;

void solve(int test_case) {
    int s, d, k;
    cin >> s >> d >> k;
    cout << "Case #" << test_case << ": ";
    int bun = 2*(s + d);
    int meat = s + 2*d;
    cout << ((k <= meat && k <= bun-1) ? "YES" : "NO" ) << '\n';
}

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    
    int tests;
    cin >> tests;
    for (int tc = 1; tc <= tests; tc++)
        solve(tc); 
}
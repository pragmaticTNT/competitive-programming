#include <bits/stdc++.h>

using namespace std;

void solve(int test_case) {
    int s_price, d_price, cost;
    cin >> s_price >> d_price >> cost;
    cout << "Case #" << test_case << ": ";
    int bun = 2*(s_price + d_price);
    int meat = s_price + 2*d_price;
    if (cost <= meat && cost <= bun-1)
        cout << "YES" << "\n";
    else 
        cout << "NO" << "\n";
}

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    
    int tests;
    cin >> tests;
    for (int tc = 1; tc <= tests; tc++)
        solve(tc); 
}
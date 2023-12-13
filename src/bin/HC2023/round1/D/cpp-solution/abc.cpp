#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define pb push_back
#define pii pair<int, int>
#define all(a) a.begin(), a.end()
const int mod = 1e9 + 7, N = 100005;

struct Seg {
    int l, r, m, mx, revmx, pos, revpos, lz;
    Seg* ch[2];
    Seg (int _l, int _r, vector <int> &a) : l(_l), r(_r), m(l + r >> 1), lz(0) {
        if (r - l > 1) {
            ch[0] = new Seg(l, m, a);
            ch[1] = new Seg(m, r, a);
            pull();
        } else {
            mx = a[l], revmx = (mod - a[l]) % mod, pos = revpos = l;
        }
    }
    void pull() {
        mx = max(ch[0]->mx, ch[1]->mx);
        revmx = max(ch[0]->revmx, ch[1]->revmx);
        if (ch[0]->mx < ch[1]->mx) {
            pos = ch[1]->pos;
        } else {
            pos = ch[0]->pos;
        }
        if (ch[0]->revmx < ch[1]->revmx) {
            revpos = ch[1]->revpos;
        } else {
            revpos = ch[0]->revpos;
        }
    }
    void give() {
        lz ^= 1;
        swap(mx, revmx), swap(pos, revpos);
    }
    void push() {
        if (lz) {
            ch[0]->give(), ch[1]->give(), lz = 0;
        }
    }
    void modify(int a, int b) {
        if (a <= l && r <= b) {
            give();
            //cout << "Modified: " << l << ", " << r << '\n';
        } else {
            push();
            if (a < m) {
                ch[0]->modify(a, b);
            }
            if (m < b) {
                ch[1]->modify(a, b);
            }
            pull();
        }
    }
};

void solve() {
    int n;
    cin >> n;
    vector <int> a(n);
    for (int i = 0; i < n; ++i) {
        cin >> a[i];
    }
    Seg root(0, n, a);
    ll ans = 0;
    int q; cin >> q;
    while (q--) {
        int l, r; cin >> l >> r;
        //cout << "query: " << l << ", " << r << '\n';
        --l;
        root.modify(l, r);
        ans += root.pos + 1;
        cout << "maximum index: " << root.pos + 1 << '\n';
    }
    cout << ans << '\n';
}

int main() {
    ios::sync_with_stdio(false), cin.tie(0);
    int t;
    cin >> t;
    for (int tc = 1; tc <= t; ++tc) {
        cout << "Case #" << tc << ": ";
        solve();
    }
}
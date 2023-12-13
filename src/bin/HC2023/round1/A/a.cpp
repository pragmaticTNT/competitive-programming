#include <bits/stdc++.h>
#include <algorithm>

using namespace std;

void solve(int test_case) {
    float output;
    int n, elf;
    cin >> n;
    vector<float> elves;
    for (int i=0; i < n; i++){
        cin >> elf;
        elves.push_back(elf);
    }

    if (n == 5) {
        sort(elves.begin(), elves.end(), less<float>());
        if (elves[2] - elves[0] > elves[4]-elves[2])
            output = (elves[4]+elves[2])/2 - (elves[1]+elves[0])/2;
        else 
            output = (elves[4]+elves[3])/2 - (elves[2]+elves[0])/2;
    } else {
        vector<float> ff(elves.begin(), elves.begin()+4);
        sort(ff.begin(), ff.end(), less<float>());
        float l1 = ff[0], l2 = ff[1], r1 = ff[2], r2 = ff[3];

        for (int i=4; i<n; i++) {
            if (elves[i] < l1) {
                swap(l1, l2);
                l1 = elves[i];
            } else if (elves[i] < l2) {
                l2 = elves[i];
            } else if (elves[i] > r2) {
                swap(r1, r2);
                r2 = elves[i];
            } else if (elves[i] > r1) {
                r1 = elves[i];
            }
        }

        output = (r1 + r2)/2 - (l1 + l2)/2;
    }

    cout << "Case #" << test_case << ": ";
    cout << output << '\n';
}

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    
    int tests;
    cin >> tests;
    for (int tc = 1; tc <= tests; tc++)
        solve(tc); 
}
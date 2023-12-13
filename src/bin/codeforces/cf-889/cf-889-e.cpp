#pragma GCC optimize(3)
// #pragma GCC optimize("trapv")
#include<bits/stdc++.h>
#define int long long
using namespace std;
 
#define rep(i, a, b) for(int i = a; i < (b); ++i)
#define all(x) begin(x), end(x)
#define sz(x) (int)(x).size()
typedef long long ll;
typedef pair<int, int> pii;
typedef vector<int> vi;
const int N=507;
const int INF=1e9;
const int mod=1e9+7;
const double pi=acos(-1);
mt19937_64 rng(time(NULL));
 
int n,m;
int dp[N][N];
int a[N];
int inv2=(mod+1)/2;
int solve(int u,int v){
  if (u==v) return 0;
  if (dp[u][v]!=-1) return dp[u][v];
  if (v==m+1) return m+1-u;
  return dp[u][v]=(1+solve(u+1,v)+solve(u,v+1))*inv2%mod;
}
signed main(){
  ios::sync_with_stdio(false);
  cin.tie(0), cout.tie(0);
  cin>>n>>m;
  for (int i=1;i<=m+1;++i){
    for (int j=1;j<=m+1;++j) dp[i][j]=-1;
  }
  for (int i=1;i<=m;++i){
    for (int j=i+1;j<=m;++j) solve(i,j);
  }
  for (int i=0;i<n;++i) cin>>a[i];
  int ans=0;
  ans=m+1-a[n-1];
  // cerr<<dp[2][3]<<endl;
  for (int i=0;i+1<n;++i) ans=(ans+dp[a[i]][a[i+1]])%mod;
  cout<<ans%mod;
}  
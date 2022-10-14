#include <bits/stdc++.h>
using namespace std;

#define rep(i, a, b) for(int i = a; i < (b); ++i)
#define all(x) begin(x), end(x)
#define sz(x) (int)(x).size()
typedef long long ll;
typedef pair<int, int> pii;
typedef vector<int> vi;

template<typename F, typename S>std::ostream& operator<<(std::ostream& os, const std::pair<F, S>& p) {return os << p.first << ' ' << p.second << '\n';} // << for pair
template<typename C, typename = typename std::enable_if<!std::is_same<C, std::string>::value, decltype(begin(std::declval<C>()))>::type>
std::ostream& operator<<(std::ostream& os, const C& c) {for (const auto& e : c) os << e << ' ';return os << '\n';} // << for containers with begin/end
template<typename T>void print(const T& c, std::string sep = " ", std::string end = "\n") {for (const auto& e : c) std::cout << e << sep;std::cout << end;} // pretty print for begin/endables
template <typename T>std::istream& operator,(std::istream& in, T& t){in >> t;return in;}
#define int(...) int __VA_ARGS__;cin>>__VA_ARGS__;
#define str(...) string __VA_ARGS__;cin>>__VA_ARGS__;
#define printd(c) for (const auto& e : c) cout << e << ' '
#define umap unordered_map
#define uset unordered_set
#define pb(x) push_back(x)
#define eb(...) emplace_back(__VA_ARGS__)
#define mp(x, y) make_pair(x, y)
#define Rep(b) for(int i = 0; i < (b); ++i)
#define car const auto&

int main() {
    cin.tie(0)->sync_with_stdio(0); cin.exceptions(cin.failbit);
    
}


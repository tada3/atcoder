import sys
input = sys.stdin.readline
 
def main():
    N,K = map(int,input().split())
    V = list(map(int,input().split()))
    
    if N < K:
        K = N
    ans = -(10**10)
    for pick in range(K+1):
        throw = min(pick,K-pick)
        for left in range(pick+1):
            right = pick-left
            p = []
            minus = 0
            for i in range(left):
                if V[i] < 0:
                    minus += 1
                p.append(V[i])
            for i in range(right):
                if V[-i-1] < 0:
                    minus += 1
                p.append(V[-i-1])
            p.sort()
            if minus >= throw:
                rest = p[throw:]
            else:
                rest = p[minus:]
            ans = max(ans,sum(rest))
    
    print(ans)
    
if __name__ == "__main__":
    main()
import sys
input = sys.stdin.readline

def main():
    N,K = map(int,input().split())
    V = list(map(int,input().split()))
    
    if N < K:
        K = N
    ans = -(10**10)
    for pick in range(K+1):
        throw = min(pick,K-pick)
        for left in range(pick+1):
            right = pick-left
            p = []
            minus = 0
            for i in range(left):
                if V[i] < 0:
                    minus += 1
                p.append(V[i])
            for i in range(right):
                if V[-i-1] < 0:
                    minus += 1
                p.append(V[-i-1])
            p.sort()
            if minus >= throw:
                rest = p[throw:]
            else:
                rest = p[minus:]
            ans = max(ans,sum(rest))
    
    print(ans)
    
if __name__ == "__main__":
    main()

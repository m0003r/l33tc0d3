class Solution:
    def uniquePathsWithObstacles(self, obstacleGrid: List[List[int]]) -> int:
        h = len(obstacleGrid)
        w = len(obstacleGrid[0])

        s = [1] + [0] * (w-1)
        for y in range(h):
            for x in range(w):
                if obstacleGrid[y][x]:
                    s[x] = 0
                elif x > 0:
                    s[x] += s[x - 1]

        return s[w-1]
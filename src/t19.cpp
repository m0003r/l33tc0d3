#include <cassert>
#include <iostream>
#include <ranges>
#include <vector>

struct ListNode
{
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr)
    {
    }
    ListNode(int x) : val(x), next(nullptr)
    {
    }
    ListNode(int x, ListNode *next) : val(x), next(next)
    {
    }

    // Constructor from vector
    ListNode(const std::vector<int> &vec) : val(0), next(nullptr)
    {
        if (!vec.empty())
        {
            val = vec[0];
            ListNode *current = this;
            for (auto &el : vec | std::views::drop(1))
            {
                current->next = new ListNode(el);
                current = current->next;
            }
        }
    }

    // Convert to vector
    std::vector<int> to_vector() const
    {
        std::vector<int> result;
        const ListNode *current = this;
        while (current != nullptr)
        {
            result.push_back(current->val);
            current = current->next;
        }
        return result;
    }
};

class Solution
{
  public:
    ListNode *removeNthFromEnd(ListNode *head, int n)
    {
        if (n == 0)
        {
            return head;
        }
        auto end_finder = head;
        for (int i = 1; i < n; i++)
        {
            if (end_finder->next == nullptr)
            {
                return head;
            }
            end_finder = end_finder->next;
        }

        if (end_finder->next == nullptr)
        {
            return head->next;
        }

        end_finder = end_finder->next;
        auto rem_finder = head;
        while (end_finder->next)
        {
            end_finder = end_finder->next;
            rem_finder = rem_finder->next;
        }

        rem_finder->next = rem_finder->next->next;

        return head;
    }
};

void testcase(std::vector<int> src, int n, std::vector<int> dst) {
    Solution s;
    auto list = ListNode(src);
    auto res = s.removeNthFromEnd(&list, n);
    if (dst.empty()) {
        assert(res == nullptr);
    } else {
        assert(res->to_vector() == dst);
    }
}

int main()
{
    testcase(std::vector({1,2,3,4,5}), 2, std::vector<int>({1,2,3,5}));
    testcase(std::vector({1,2,3,4,5}), 1, std::vector<int>({1,2,3,4}));
    testcase(std::vector({1,2,3,4,5}), 4, std::vector<int>({1,3,4,5}));
    testcase(std::vector({1,2,3,4,5}), 5, std::vector<int>({2,3,4,5}));
    testcase(std::vector({1,2}), 1, std::vector<int>({1}));
    testcase(std::vector({1,2}), 2, std::vector<int>({2}));
    testcase(std::vector({1}), 1, std::vector<int>());
    return 0;
}
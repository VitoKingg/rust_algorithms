# Data Structures and Algorithms in Rust

- [Rust Algorithms](https://github.com/TheAlgorithms/Rust)
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [Data Structures and Algorithms in Rust](https://github.com/QMHTMY/RustBook)

## Data Structures

### [array](<https://en.wikipedia.org/wiki/Array_(data_structure)>)

- [Array Sum](./src/general/array_sum.rs)
- [LeetCode 0448 Find All Numbers Disappeared in an Array (Easy)](./src/leetcode_solutions/leetcode_0448_find_all_numbers_disappeared_in_an_array.rs)
- [LeetCode 0287 Find the Duplicate Number (Medium)](./src/leetcode_solutions/leetcode_0287_find_the_duplicate_number.rs)
- [LeetCode 0048 Rotate Image (Medium)](./src/leetcode_solutions/leetcode_0048_rotate_image.rs)
- [LeetCode 0240 Search a 2D Matrix II (Medium)](./src/leetcode_solutions/leetcode_0240_search_a_2d_matrix_ii.rs)
- [LeetCode 0566 Reshape the Matrix (Easy)](./src/leetcode_solutions/leetcode_0566_reshape_the_matrix.rs)
- [LeetCode 0769 Max Chunks to Make Sorted (Medium)](./src/leetcode_solutions/leetcode_0769_max_chunks_to_make_sorted.rs)

### [stack](<https://en.wikipedia.org/wiki/Stack_(abstract_data_type)>)

- LIFO: last in, first out
- JVM (Java Virtual Machine) 基于 stack
- 操作系统函数调用基于 stack
- 浏览器后退操作基于 stack
- 主流编辑器的操作记录撤销功能基于 stack
-
- [call stack](https://en.wikipedia.org/wiki/Call_stack) 调用栈
  - all function calls go onto the call stack.
  - The call stack can get very large, which takes up a lot of memory.
  - 在大部分操作系统中，每个运行中的二进制程序都配有一个调用栈(call stack) 或执行栈(execution stack)。借助调用栈可以跟踪属于同一程序的所有函数，记录它们之间的调用关系，并保证在每一调用实例执行完毕之后，可以准确地返回。
  - 基本单位: frame (帧)
-
- [LeetCode 0225 Implement Stack using Queues (Easy)](./src/leetcode_solutions/leetcode_0225_implement_stack_using_queues.rs)
- [LeetCode 0232 Implement Queue using Stacks (Easy)](./src/leetcode_solutions/leetcode_0232_implement_queue_using_stacks.rs)
- [LeetCode 0155 Min Stack (Easy)](./src/leetcode_solutions/leetcode_0155_min_stack.rs)
- [LeetCode 0020 Valid Parentheses (Easy)](./src/leetcode_solutions/leetcode_0020_valid_parentheses.rs)
- [LeetCode 0739 Daily Temperatures (Medium)](./src/leetcode_solutions/leetcode_0739_daily_temperatures.rs)
- [LeetCode 0503 Next Greater Element II (Medium)](./src/leetcode_solutions/leetcode_0503_next_greater_element_ii.rs)

### [queue](<https://en.wikipedia.org/wiki/Queue_(abstract_data_type)>)

- FIFO: first in, first out
- [circular buffer](https://en.wikipedia.org/wiki/Circular_buffer) 循环队列
- 操作系统任务调度

### [double-ended queue](https://en.wikipedia.org/wiki/Double-ended_queue)

- [LeetCode 0239 Sliding Window Maximum (Hard)](./src/leetcode_solutions/leetcode_0239_sliding_window_maximum.rs)
- [LeetCode 0217 Contains Duplicate (Easy)](./src/leetcode_solutions/leetcode_0217_contains_duplicate.rs)

### [priority queue](https://en.wikipedia.org/wiki/Priority_queue)

- [LeetCode 0023 Merge k Sorted Lists (Hard)](./src/leetcode_solutions/leetcode_0023_merge_k_sorted_lists.rs)
- [LeetCode 0218 The Skyline Problem (Hard)](./src/leetcode_solutions/leetcode_0218_the_skyline_problem.rs)
- [LeetCode 0313 Super Ugly Number (Medium)](./src/leetcode_solutions/leetcode_0313_super_ugly_number.rs)

### [double-ended priority queue](https://en.wikipedia.org/wiki/Double-ended_priority_queue)

### [linked list](https://en.wikipedia.org/wiki/Linked_list)

- singly linked list
- [doubly linked list](https://en.wikipedia.org/wiki/Doubly_linked_list)
- multiply linked list
- circular linked list
- sentinel node 哨兵节点
-
- [LeetCode tag - Linked List](https://leetcode.com/tag/linked-list/)
-
- [LeetCode 0206 Reverse Linked List (Easy)](./src/leetcode_solutions/leetcode_0206_reverse_linked_list.rs)
- [LeetCode 0021 Merge Two Sorted Lists (Easy)](./src/leetcode_solutions/leetcode_0021_merge_two_sorted_lists.rs)
- [LeetCode 0024 Swap Nodes in Pairs (Medium)](./src/leetcode_solutions/leetcode_0024_swap_nodes_in_pairs.rs)
- [LeetCode 0160 Intersection of Two Linked Lists (Easy)](./src/leetcode_solutions/leetcode_0160_intersection_of_two_linked_lists.rs)
- [LeetCode 0234 Palindrome Linked List (Easy)](./src/leetcode_solutions/leetcode_0234_palindrome_linked_list.rs)
- [LeetCode 0083 Remove Duplicates from Sorted List (Easy)](./src/leetcode_solutions/leetcode_0083_remove_duplicates_from_sorted_list.rs)
- [LeetCode 0328 Odd Even Linked List (Medium)](./src/leetcode_solutions/leetcode_0328_odd_even_linked_list.rs)
- [LeetCode 0019 Remove Nth Node From End of List (Medium)](./src/leetcode_solutions/leetcode_0019_remove_nth_node_from_end_of_list.rs)
- [LeetCode 0148 Sort List (Medium)](./src/leetcode_solutions/leetcode_0148_sort_list.rs)

### [hash table](https://en.wikipedia.org/wiki/Hash_table)

- load factor 装填因子
- acceptable figures of load factor include 0.6 and 0.75
- hash function
- perfect hash function
- collision resolution
  - separate chaining 拉链法
  - opening addressing 开放寻址法
- dynamic resizing 动态扩容
- rehash
-
- associative arrays
- database indexing
- caches
- sets
-
- [LeetCode tag - Hash Table](https://leetcode.com/tag/hash-table/)
- [LeetCode 0001 Two Sum (Easy)](./src/leetcode_solutions/leetcode_0001_two_sum.rs)
- [LeetCode 0128 Longest Consecutive Sequence (Medium)](./src/leetcode_solutions/leetcode_0128_longest_consecutive_sequence.rs)
- [LeetCode 0594 Longest Harmonious Subsequence (Easy)](./src/leetcode_solutions/leetcode_0594_longest_harmonious_subsequence.rs)
- [LeetCode 0149 Max Points on a Line (Hard)](./src/leetcode_solutions/leetcode_0149_max_points_on_a_line.rs)
- [LeetCode 0332 Reconstruct Itinerary (Medium)](./src/leetcode_solutions/leetcode_0332_reconstruct_itinerary.rs)
- [LeetCode 0870 Advantage Shuffle (Medium)](./src/leetcode_solutions/leetcode_0870_advantage_shuffle.rs)

### [tree](<https://en.wikipedia.org/wiki/Tree_(data_structure)>)

- [binary tree](https://en.wikipedia.org/wiki/Binary_tree) 二叉树
-
- [Huffman coding](https://en.wikipedia.org/wiki/Huffman_coding) (also called `Huffman tree`)
-
- TODO: [binary search tree](https://en.wikipedia.org/wiki/Binary_search_tree) 二叉搜索树
- [binary_search_tree.rs](./src/data_structures/binary_search_tree.rs)
- [LeetCode tag - Binary Search Tree](https://leetcode.com/tag/binary-search-tree/)
-
- [self-balancing binary search tree](https://en.wikipedia.org/wiki/Self-balancing_binary_search_tree)
-
- [AVL tree](https://en.wikipedia.org/wiki/AVL_tree) 平衡二叉树
- TODO: [avl_tree.rs](./src/data_structures/avl_tree.rs)
-
- [red-black tree](https://en.wikipedia.org/wiki/Red%E2%80%93black_tree) 红黑树
  - `Completely Fair Schedular` used in current Linux kernels
  - `epoll` system call
-
- [B-tree](https://en.wikipedia.org/wiki/B-tree)
  - The B-tree is well suited for storage systems that read and write relatively large blocks of data, such as databases and file systems.
-
- [B+ tree](https://en.wikipedia.org/wiki/B%2B_tree)
  - databases and file systems
-
- [2-3 tree](https://en.wikipedia.org/wiki/2%E2%80%933_tree)
-
- [2-3-4 tree](https://en.wikipedia.org/wiki/2%E2%80%933%E2%80%934_tree)
-
- [AA tree](https://en.wikipedia.org/wiki/AA_tree)
-
- [dancing tree](https://en.wikipedia.org/wiki/Dancing_tree)
-
- [HTree](https://en.wikipedia.org/wiki/HTree)
-
- [weight-balanced tree](https://en.wikipedia.org/wiki/Weight-balanced_tree)
-
- [interval tree](https://en.wikipedia.org/wiki/Interval_tree)
-
- [splay tree](https://en.wikipedia.org/wiki/Splay_tree) 伸展树
-
- [treap](https://en.wikipedia.org/wiki/Treap)
-
- [scapegoat tree](https://en.wikipedia.org/wiki/Scapegoat_tree)
-
- [tango tree](https://en.wikipedia.org/wiki/Tango_tree)
-
- [k-d tree](https://en.wikipedia.org/wiki/K-d_tree)
  - nearest neighbor search
  - range search
-
- [m-ary tree](https://en.wikipedia.org/wiki/M-ary_tree)
- [quadtree](https://en.wikipedia.org/wiki/Quadtree) 四叉树
- [octree](https://en.wikipedia.org/wiki/Octree) 八叉树
-
- [segment tree](https://en.wikipedia.org/wiki/Segment_tree) 线段树
-
- [LeetCode tag - Tree](https://leetcode.com/tag/tree/)
- [LeetCode 0104 Maximum Depth of Binary Tree (Easy)](./src/leetcode_solutions/leetcode_0104_maximum_depth_of_binary_tree.rs)
- [LeetCode 0110 Balanced Binary Tree (Easy)](./src/leetcode_solutions/leetcode_0110_balanced_binary_tree.rs)
- [LeetCode 0226 Invert Binary Tree (Easy)](./src/leetcode_solutions/leetcode_0226_invert_binary_tree.rs)
- [LeetCode 0543 Diameter of Binary Tree (Easy)](./src/leetcode_solutions/leetcode_0543_diameter_of_binary_tree.rs)
- [LeetCode 0617 Merge Two Binary Trees (Easy)](./src/leetcode_solutions/leetcode_0617_merge_two_binary_trees.rs)
- [LeetCode 0437 Path Sum III (Medium)](./src/leetcode_solutions/leetcode_0437_path_sum_iii.rs)
- [LeetCode 0101 Symmetric Tree (Easy)](./src/leetcode_solutions/leetcode_0101_symmetric_tree.rs)
- [LeetCode 0572 Subtree of Another Tree (Easy)](./src/leetcode_solutions/leetcode_0572_subtree_of_another_tree.rs)
- [LeetCode 0404 Sum of Left Leaves (Easy)](./src/leetcode_solutions/leetcode_0404_sum_of_left_leaves.rs)
- [LeetCode 0513 Find Bottom Left Tree Value (Easy)](./src/leetcode_solutions/leetcode_0513_find_bottom_left_tree_value.rs)
- [LeetCode 0538 Convert BST to Greater Tree (Easy)](./src/leetcode_solutions/leetcode_0538_convert_bst_to_greater_tree.rs)
- [LeetCode 0235 Lowest Common Ancestor of a Binary Search Tree (Easy)](./src/leetcode_solutions/leetcode_0235_lowest_common_ancestor_of_a_binary_search_tree.rs)
- [LeetCode 0530 Minimum Absolute Difference in BST (Easy)](./src/leetcode_solutions/leetcode_0530_minimum_absolute_difference_in_bst.rs)
- [LeetCode 1110 Delete Nodes and Return Forest (Medium)](./src/leetcode_solutions/leetcode_1110_delete_nodes_and_return_forest.rs)
- [LeetCode 0637 Average of Levels in Binary Tree (Easy)](./src/leetcode_solutions/leetcode_0637_average_of_levels_in_binary_tree.rs)
- [LeetCode 0105 Construct Binary Tree from Preorder and Inorder Traversal (Medium)](./src/leetcode_solutions/leetcode_0105_construct_binary_tree_from_preorder_and_inorder_traversal.rs)
- [LeetCode 0144 Binary Tree Preorder Traversal (Medium)](./src/leetcode_solutions/leetcode_0144_binary_tree_preorder_traversal.rs)
- [LeetCode 0099 Recover Binary Search Tree (Hard)](./src/leetcode_solutions/leetcode_0099_recover_binary_search_tree.rs)
- [LeetCode 0699 Trim a Binary Search Tree (Easy)](./src/leetcode_solutions/leetcode_0699_trim_a_binary_search_tree.rs)
- [LeetCode 0208 Implement Trie (Prefix Tree) (Medium)](./src/leetcode_solutions/leetcode_0208_implement_trie_prefix_tree.rs)
- [LeetCode 0889 Construct Binary Tree from Preorder and Postorder Traversal (Medium)](./src/leetcode_solutions/leetcode_0889_construct_binary_tree_from_preorder_and_postorder_traversal.rs)
- [LeetCode 0106 Construct Binary Tree from Inorder and Postorder Traversal (Medium)](./src/leetcode_solutions/leetcode_0106_construct_binary_tree_from_inorder_and_postorder_traversal.rs)
- [LeetCode 0094 Binary Tree Inorder Traversal (Medium)](./src/leetcode_solutions/leetcode_0094_binary_tree_inorder_traversal.rs)
- [LeetCode 0145 Binary Tree Postorder Traversal (Medium)](./src/leetcode_solutions/leetcode_0145_binary_tree_postorder_traversal.rs)
- [LeetCode 0236 Lowest Common Ancestor of a Binary Tree (Medium)](./src/leetcode_solutions/leetcode_0236_lowest_common_ancestor_of_a_binary_tree.rs)
- [LeetCode 0109 Convert Sorted List to Binary Search Tree (Medium)](./src/leetcode_solutions/leetcode_0109_convert_sorted_list_to_binary_search_tree.rs)
- [LeetCode 0897 Increasing Order Search Tree (Easy)](./src/leetcode_solutions/leetcode_0897_increasing_order_search_tree.rs)
- [LeetCode 0653 Two Sum IV - Input is a BST (Easy)](./src/leetcode_solutions/leetcode_0653_two_sum_iv.rs)
- [LeetCode 0450 Delete Node in a BST (Medium)](./src/leetcode_solutions/leetcode_0450_delete_node_in_a_bst.rs)

### [graph](<https://en.wikipedia.org/wiki/Graph_(abstract_data_type)>)

- [adjacency list](https://en.wikipedia.org/wiki/Adjacency_list) 邻接表
- [adjacency matrix](https://en.wikipedia.org/wiki/Adjacency_matrix) 邻接矩阵
- [incidence matrix](https://en.wikipedia.org/wiki/Incidence_matrix) 关联矩阵
-
- [breadth-first search](https://en.wikipedia.org/wiki/Breadth-first_search) 广度优先搜索
- [LeetCode tag - Breadth-First Search](https://leetcode.com/tag/breadth-first-search/)
-
- [depth-first search](https://en.wikipedia.org/wiki/Depth-first_search) 深度优先搜索
- [LeetCode tag - Depth-First Search](https://leetcode.com/tag/depth-first-search/)
-
- [best-first search](https://en.wikipedia.org/wiki/Best-first_search) 最佳优先搜索
-
- [Dijkstra's algorithm (/ˈdaɪkstrəz/)](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
- [Bellman–Ford algorithm](https://en.wikipedia.org/wiki/Bellman%E2%80%93Ford_algorithm)
- [Prim's algorithm](https://en.wikipedia.org/wiki/Prim%27s_algorithm)
-
- [LeetCode tag - Graph](https://leetcode.com/tag/graph/)
- [LeetCode 0785 Is Graph Bipartite (Medium)](./src/leetcode_solutions/leetcode_0785_is_graph_bipartite.rs)
- [LeetCode 0210 Course Schedule II (Medium)](./src/leetcode_solutions/leetcode_0210_course_schedule_ii.rs)
- [LeetCode 1059 All Paths from Source Lead to Destination (Medium)](./src/leetcode_solutions/leetcode_1059_all_paths_from_source_lead_to_destination.rs)
- [LeetCode 1135 Connecting Cities with Minimum Cost (Medium)](./src/leetcode_solutions/leetcode_1135_connecting_cities_with_minimum_cost.rs)
- [LeetCode 0882 Reachable Nodes in Subdivided Graph (Hard)](./src/leetcode_solutions/leetcode_0882_reachable_nodes_in_subdivided_graph.rs)
- [LeetCode 0743 Network Delay Time (Medium)](./src/leetcode_solutions/leetcode_0743_network_delay_time.rs)
- [LeetCode 2203 Minimum Weighted Subgraph With the Required Paths (Hard)](./src/leetcode_solutions/leetcode_2203_minimum_weighted_subgraph_with_the_required_path.rs)
- [LeetCode 1584 Min Cost to Connect All Points (Medium)](./src/leetcode_solutions/leetcode_1584_min_cost_to_connect_all_points.rs)
- [LeetCode 1168 Optimize Water Distribution in a Village (Hard)](./src/leetcode_solutions/leetcode_1168_optimize_water_distribution_in_a_village.rs)
- [LeetCode 1489 Find Critical and Pseudo-Critical Edges in Minimum Spanning Tree (Hard)](./src/leetcode_solutions/leetcode_1489_find_critical_and_pseudo_critical_edges_in_minimum_spanning_tree.rs)

### [heap](<https://en.wikipedia.org/wiki/Heap_(data_structure)>)

- The C++ Standard Library provides the make_heap, push_heap and pop_heap algorithms for heaps (usually implemented as binary heaps), which operate on arbitrary random access iterators.
- [binary heap](https://en.wikipedia.org/wiki/Binary_heap) 二叉堆
- [leftist tree](https://en.wikipedia.org/wiki/Leftist_tree) (also called `leftist heap`) 左式堆
- [binomial heap](https://en.wikipedia.org/wiki/Binomial_heap)
- [Fibonacci heap](https://en.wikipedia.org/wiki/Fibonacci_heap)
- [pairing heap](https://en.wikipedia.org/wiki/Pairing_heap)
- [Brodal queue](https://en.wikipedia.org/wiki/Brodal_queue)
- [rank-pairing heap](https://en.wikipedia.org/wiki/Rank-pairing_heap)
- [2-3 heap](https://en.wikipedia.org/wiki/2%E2%80%933_heap)
-
- [LeetCode tag - Heap (Priority Queue)](https://leetcode.com/tag/heap-priority-queue/)

### [skip list](https://en.wikipedia.org/wiki/Skip_list) 跳跃表 跳转表

- [LeetCode 1206 Design Skiplist](./src/leetcode_solutions/leetcode_1206_design_skiplist.rs)

### [disjoint-set](https://en.wikipedia.org/wiki/Disjoint-set_data_structure) (also called `union-find`) 并查集

- [LeetCode tag - Union Find](https://leetcode.com/tag/union-find/)
-
- LeetCode 0130
- LeetCode 0547
- LeetCode 0684 Redundant Connection (Medium)
- LeetCode 0695

### [Bloom filter](https://en.wikipedia.org/wiki/Bloom_filter) 布隆过滤器

### [cuckoo filter](https://en.wikipedia.org/wiki/Cuckoo_filter) 布谷过滤器

### [quotient filter](https://en.wikipedia.org/wiki/Quotient_filter) 商过滤器

### [count-min sketch](https://en.wikipedia.org/wiki/Count%E2%80%93min_sketch)

### [inverted index](https://en.wikipedia.org/wiki/Inverted_index) 反向索引 倒排索引

## Algorithms

### [big O notation](https://en.wikipedia.org/wiki/Big_O_notation)

### [time complexity](https://en.wikipedia.org/wiki/Time_complexity)

### [space complexity](https://en.wikipedia.org/wiki/Space_complexity)

### [amortized analysis](https://en.wikipedia.org/wiki/Amortized_analysis) 分摊分析 - 评定算法性能的一种重要尺度

### 基础算法

- 0000 Brackets Matching
- LeetCode 1870 Minimum Speed to Arrive on Time (Medium)
- LeetCode 1894 Find the Student that Will Replace the Chalk (Medium)
- LeetCode 1898 Maximum Number of Removable Characters (Medium)
- LeetCode 0912 Sort an Array (Medium)
- LeetCode 0043 Multiply Strings (Medium)
- LeetCode 0370 Range Addition (Medium)
- LeetCode 2132 Stamping the Grid (Hard)
- LeetCode 0191 Number of 1 Bits (Easy)

### [divide-and-conquer algorithm](https://en.wikipedia.org/wiki/Divide-and-conquer_algorithm) 分治

- [LeetCode tag - Divide and Conquer](https://leetcode.com/tag/divide-and-conquer/)
-
- D&C algorithms are recursive algorithms. To solve a problem using D&C, there are two steps:
  - Figure out the base case. This should be the simplest possible case.
  - Divide or decrease your problem until it becomes the base case.
-
- LeetCode 0241 Different Ways to Add Parentheses (Medium)
- LeetCode 0932 Beautiful Array (Medium)
- LeetCode 0312 Burst Balloons (Hard)
-
- 力扣 LCP 04 Broken Board Dominoes (棋盘覆盖问题)
- 循环赛日程安排问题
- 输油管道问题
- quicksort
- merge sort

### [recursion](https://en.wikipedia.org/wiki/Recursion) 递归

- 递归通过将问题由大化小来逐步解决问题
- 递归算法所消耗的空间量取决于递归深度
- 递归往往消耗更多的空间，所以对于运行速度要求极高、存储空间需精打细算的场合，往往应该使用非递归算法
- 递归消除
- [tail recursion](https://en.wikipedia.org/wiki/Tail_call) 尾递归
- 尾递归是递归的优化形式，能在一定程度上减少栈资源的使用

### [dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming) 动态规划

- Every dynamic programming algorithm solution involves a grid.
- Each cell is a subproblem, so think about how you can divide your problem into subproblems.
- 动态规划可用于解决最优化问题，由小问题推导到大问题，推导过程的中间值要缓存起来，推导过程称为状态转移
-
- [LeetCode tag - Dynamic Programming](https://leetcode.com/tag/dynamic-programming/)
-
- LeetCode 0070 Climbing Stairs (Easy)
- LeetCode 0198 House Robber (Easy)
- LeetCode 0413 Arithmetic Slices (Medium)
- LeetCode 0064 Minimum Path Sum (Medium)
- LeetCode 0542 01 Matrix (Medium)
- LeetCode 0221 Maximal Square (Medium)
- LeetCode 0091 Decode Ways (Medium)
- LeetCode 0139 Word Break (Medium)
- LeetCode 0300 Longest Increasing Subsequence (Medium)
- LeetCode 1143 Longest Common Subsequence (Medium)
- LeetCode 0072 Edit Distance (Hard)
- LeetCode 0650 2 Keys Keyboard (Medium)
- LeetCode 0010 Regular Expression Matching (Hard)
- LeetCode 0121 Best Time to Buy and Sell Stock (Easy)
- LeetCode 0188 Best Time to Buy and Sell Stock IV (Hard)
- LeetCode 0309 Best Time to Buy and Sell Stock with Cooldown (Medium)
- LeetCode 0213 House Robber II (Medium)
- LeetCode 0053 Maximum Subarray (Easy)
- LeetCode 0343 Integer Break (Medium)
- LeetCode 0583 Delete Operation for Two Strings (Medium)
- LeetCode 0646 Maximum Length of Pair Chain (Medium)
- LeetCode 0376 Wiggle Subsequence (Medium)
- LeetCode 0714 Best Time to Buy and Sell Stock with Transaction Fee (Medium)
- LeetCode 0118 Pascal's Triangle (Easy)
- LeetCode 0741 Cherry Pickup (Hard)
- LeetCode 1463 Cherry Pickup II (Hard)
- LeetCode 0435 Non-overlapping Intervals (Medium)
- LeetCode 0960 Delete Columns to Make Sorted III (Hard)
- LeetCode 0354 Russian Doll Envelopes (Hard)
- LeetCode 1691 Maximum Height by Stacking Cuboids (Hard)
- LeetCode 1626 Best Team With No Conflicts (Medium)
- LeetCode 0712 Minimum ASCII Delete Sum for Two Strings (Medium)
- LeetCode 0377 Combination Sum IV (Medium)
- LeetCode 2218 Maximum Value of K Coins From Piles (Hard)
-
- [knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem) 背包问题
- LeetCode 0416 Partition Equal Subset Sum (Medium)
- LeetCode 0279 Perfect Squares (Medium)
- [LeetCode 0322 Coin Change (Medium)](./src/leetcode_solutions/leetcode_0322_coin_change.rs)
- LeetCode 0518 Coin Change II
- LeetCode 0474 Ones and Zeroes (Medium)
- LeetCode 0879
- LeetCode 0494 Target Sum (Medium)
- LeetCode 1049 Last Stone Weight II (Medium)
-
- [subset sum problem](https://en.wikipedia.org/wiki/Subset_sum_problem) 子集之和问题

### [greedy algorithm](https://en.wikipedia.org/wiki/Greedy_algorithm) 贪心算法

- graph coloring problem
- NP-complete problems
- traveling salesman problem
-
- [LeetCode tag - Greedy](https://leetcode.com/tag/greedy/)
-
- LeetCode 0122 Best Time to Buy and Sell Stock II
- LeetCode 0135 Candy
- LeetCode 0406 Queue Reconstruction by Height
- LeetCode 0435 Non-overlapping Intervals
- LeetCode 0452 Minimum Number of Arrows to Burst Balloons
- LeetCode 0455 Assign Cookies
- LeetCode 0605 Can Place Flowers
- LeetCode 0665 Non-decreasing Array
- LeetCode 0763 Partition Labels

### [two pointers technique](https://leetcode.com/articles/two-pointer-technique/) 双指针

- [LeetCode tag - Two Pointers](https://leetcode.com/tag/two-pointers/)
-
- LeetCode 0167 Two Sum II - Input array is sorted
- LeetCode 0088 Merge Sorted Array
- LeetCode 0142 Linked List Cycle II
- LeetCode 0076 Minimum Window Substring
- LeetCode 0633 Sum of Square Numbers
- LeetCode 0680 Valid Palindrome II
- LeetCode 0524 Longest Word in Dictionary through Deleting
- LeetCode 0340 Longest Substring with At Most K Distinct Characters

### [pointer jumping technique](https://en.wikipedia.org/wiki/Pointer_jumping)

### [sliding window technique](https://leetcode.com/discuss/study-guide/1773891/Sliding-Window-Technique-and-Question-Bank) 滑动窗口

- [LeetCode sliding window problems](https://leetcode.com/tag/sliding-window/)
-
- fixed sized window
- LeeCode 0030
- LeeCode 0438
- LeeCode 0567
- LeeCode 0643
- LeeCode 1100
- LeeCode 1151
- LeeCode 1176
- LeeCode 1343
- LeeCode 1423
- LeeCode 1876
- LeeCode 2090
-
- variable sized window
- LeeCode 0003
- LeeCode 0076
- LeeCode 0159
- LeeCode 0209
- LeeCode 0340
- LeeCode 0424
- LeeCode 0487
- LeeCode 0904
- LeeCode 1004
- LeeCode 1493
- LeeCode 1695
- LeeCode 1852
- LeeCode 2260
-
- LeeCode 0930
- LeeCode 0978
- LeeCode 0992
- LeeCode 1234
- LeeCode 1248
- LeeCode 1658

### [sorting algorithm](https://en.wikipedia.org/wiki/Sorting_algorithm)

- [Rosetta Code - Sorting algorithms](https://rosettacode.org/wiki/Category:Sorting_Algorithms)
- [Wikipedia Talk:Sorting algorithm](https://en.wikipedia.org/wiki/Talk%3ASorting_algorithm)
- 评价排序算法除了看时间空间复杂度，还要看稳定性
-
- [LeetCode tag - Sorting](https://leetcode.com/tag/sorting/)
- LeetCode 0075 Sort Colors
- LeetCode 0215 Kth Largest Element in an Array
- LeetCode 0347 Top K Frequent Elements
- LeetCode 0451 Sort Characters By Frequency
-
- [bubble sort](https://en.wikipedia.org/wiki/Bubble_sort) 冒泡排序
- [Rosetta Code - Sorting algorithms/Bubble sort](https://rosettacode.org/wiki/Sorting_algorithms/Bubble_sort)
- [bubble_sort.rs](./src/sorting/bubble_sort.rs)
-
- [cocktail shaker sort](https://en.wikipedia.org/wiki/Cocktail_shaker_sort) 鸡尾酒排序
- [Rosetta Code - Sorting algorithms/Cocktail sort](https://rosettacode.org/wiki/Sorting_algorithms/Cocktail_sort)
- [Rosetta Code - Sorting algorithms/Cocktail sort with shifting bounds](https://rosettacode.org/wiki/Sorting_algorithms/Cocktail_sort_with_shifting_bounds)
- [cocktail_shaker_sort.rs](./src/sorting/cocktail_shaker_sort.rs)
-
- [comb sort](https://en.wikipedia.org/wiki/Comb_sort) 梳排序
- [Rosetta Code - Sorting algorithms/Comb sort](https://rosettacode.org/wiki/Sorting_algorithms/Comb_sort)
- [comb_sort.rs](./src/sorting/comb_sort.rs)
-
- [I can't believe that I can prove that it can sort](https://blog.adacore.com/i-cant-believe-that-i-can-prove-that-it-can-sort)
- [i_cant_believe_it_can_sort.rs](./src/sorting/i_cant_believe_it_can_sort.rs)
-
- [quicksort](https://en.wikipedia.org/wiki/Quicksort) 快速排序
- [Rosetta Code - Sorting algorithms/Quicksort](https://rosettacode.org/wiki/Sorting_algorithms/Quicksort)
- [quicksort.rs](./src/sorting/quicksort.rs)
- `pivot` selection:
  - the first element
  - the middle element
  - the last element
  - a random element
  - the median of the first, middle and last element (median-of-three, recommended, 三者取中法)
- 快速排序算法虽然能够确保，划分出来的子任务彼此独立，并且其规模综合保持渐进不变，却不能保证两个子任务的规模大体相当，甚至有可能极不平衡。因此，该算法并不能保证最坏情况下的 O(nlogn) 时间复杂度。尽管如此，在实际应用中快速排序算法仍为首选的排序算法。究其原因在于，快速排序算法易于实现，代码结构紧凑简练，而且对于按通常规律随机分布的输入序列，快速排序算法实际的平均运行时间较之同类算法更少。
- 大多数情况下快速排序算法平均效率为 O(nlogn)，较之其它排序算法，其时间复杂度中的常系数更小。
- 如果待排序数组有大量重复元素，则快速排序会重复比较，造成性能浪费；解决方法是将数组分成三区进行排序，把重复元素放到第三个区域，只对另外两个区域进行排序；选择重复元素作为 pivot，小于 pivot 的元素放入左区，大于 pivot 的放入右区，相等的放入中区，然后对左右区域递归地调用快速排序
-
- [introsort](https://en.wikipedia.org/wiki/Introsort) 内观排序
- `introsort` 是 `quicksort` 的一种改进形式，是 `C++ std::sort()` 的实现
-
- [insertion sort](https://en.wikipedia.org/wiki/Insertion_sort) 插入排序
- [Rosetta Code - Sorting algorithms/Insertion sort](https://rosettacode.org/wiki/Sorting_algorithms/Insertion_sort)
- [insertion_sort.rs](./src/sorting/insertion_sort.rs)
-
- [Shell sort](https://en.wikipedia.org/wiki/Shellsort) 希尔排序
- [Rosetta Code - Sorting algorithms/Shell sort](https://rosettacode.org/wiki/Sorting_algorithms/Shell_sort)
- [shell_sort.rs](./src/sorting/shell_sort.rs)
-
- [merge sort](https://en.wikipedia.org/wiki/Merge_sort) 归并排序
- [Rosetta Code - Sorting algorithms/Merge sort](https://rosettacode.org/wiki/Sorting_algorithms/Merge_sort)
- [merge_sort.rs](./src/sorting/merge_sort.rs)
- [merge-insertion sort](https://en.wikipedia.org/wiki/Merge-insertion_sort) 插入归并排序
- [LeetCode tag - Merge Sort](https://leetcode.com/tag/merge-sort/)
-
- [selection sort](https://en.wikipedia.org/wiki/Selection_sort) 选择排序
- [Rosetta Code - Sorting algorithms/Selection sort](https://rosettacode.org/wiki/Sorting_algorithms/Selection_sort)
- [selection_sort.rs](./src/sorting/selection_sort.rs)
-
- [heapsort](https://en.wikipedia.org/wiki/Heapsort) 堆排序
- [Rosetta Code - Sorting algorithms/Heapsort](https://rosettacode.org/wiki/Sorting_algorithms/Heapsort)
- [heapsort.rs](./src/sorting/heapsort.rs)
-
- [bucket sort](https://en.wikipedia.org/wiki/Bucket_sort) 桶排序
- [bucket_sort.rs](./src/sorting/bucket_sort.rs)
- [LeetCode tag - Bucket Sort](https://leetcode.com/tag/bucket-sort/)
- [flashsort](https://en.wikipedia.org/wiki/Flashsort) 闪排序
-
- [counting sort](https://en.wikipedia.org/wiki/Counting_sort) 计数排序
- [Rosetta Code - Sorting algorithms/Counting sort](https://rosettacode.org/wiki/Sorting_algorithms/Counting_sort)
- [counting_sort.rs](./src/sorting/counting_sort.rs)
- [LeetCode tag - Counting Sort](https://leetcode.com/tag/counting-sort/)
-
- [radix sort](https://en.wikipedia.org/wiki/Radix_sort) 基数排序
- [Rosetta Code - Sorting algorithms/Radix sort](https://rosettacode.org/wiki/Sorting_algorithms/Radix_sort)
- [radix_sort.rs](./src/sorting/radix_sort.rs)
- [LeetCode tag - Radix Sort](https://leetcode.com/tag/radix-sort/)
-
- [Timsort](https://en.wikipedia.org/wiki/Timsort) 蒂姆排序
- Timsort 是一种混合排序算法，结合了 merge sort 和 insertion sort
- 其改进版是许多语言默认的排序算法
-
- [topological sort](https://en.wikipedia.org/wiki/Topological_sorting) 拓扑排序
- [Rosetta Code - Topological sort](https://rosettacode.org/wiki/Topological_sort)
- [LeetCode tag - Topological Sort](https://leetcode.com/tag/topological-sort/)
-
- sleep sort 睡眠排序
- [Rosetta Code - Sorting algorithms/Sleep sort](https://rosettacode.org/wiki/Sorting_algorithms/Sleep_sort)
-
- [slowsort](https://en.wikipedia.org/wiki/Slowsort) 慢速排序

### [search algorithm](https://en.wikipedia.org/wiki/Search_algorithm)

- [linear search](https://en.wikipedia.org/wiki/Linear_search) 线性搜索
- [linear_search.rs](./src/searching/linear_search.rs)
-
- [binary search](https://en.wikipedia.org/wiki/Binary_search_algorithm) 二分搜索
- [binary_search.rs](./src/searching/binary_search.rs)
- [LeetCode tag - Binary Search](https://leetcode.com/tag/binary-search/)
- 如果要查找的数据量很小，则没必要先排序再二分搜索；如果数据量很大，排序很耗时且内存消耗大，或许使用线性搜索性能更好；实际项目中数据量不大不小，很适合使用二分搜索。
-
- [interpolation search](https://en.wikipedia.org/wiki/Interpolation_search) 插值搜索
- [interpolation_search.rs](./src/searching/interpolation_search.rs)
-
- [exponential search](https://en.wikipedia.org/wiki/Exponential_search) 指数搜索
- [exponential_search.rs](./src/searching/exponential_search.rs)
-
- [Fibonacci search technique](https://en.wikipedia.org/wiki/Fibonacci_search_technique)
-
- LeetCode 0069 Sqrt(x)
- LeetCode 0034 Find First and Last Position of Element in Sorted Array (Medium)
- LeetCode 0081 Search in Rotated Sorted Array II
- LeetCode 0151 Find Minimum in Rotated Sorted Array II
- LeetCode 0540 Single Element in a Sorted Array
- LeetCode 0004 Median of Two Sorted Arrays
- LeetCode 0695 Max Area of Island (Easy)
- LeetCode 0547 Number of Provinces (Medium)
- LeetCode 0417 Pacific Atlantic Water Flow (Medium)
- LeetCode 0046 Permutations (Medium)
- LeetCode 0077 Combinations (Medium)
- LeetCode 0079 Word Search (Medium)
- LeetCode 0051 N-Queens (Hard)
- LeetCode 0934 Shortest Bridge (Medium)
- LeetCode 0126 Word Ladder II (Hard)
- LeetCode 0130 Surrounded Regions (Medium)
- LeetCode 0257 Binary Tree Paths (Easy)
- LeetCode 0047 Permutations II (Medium)
- LeetCode 0040 Combination Sum II (Medium)
- LeetCode 0037 Sudoku Solver (Hard)
- LeetCode 0310 Minimum Height Trees (Medium)
- LeetCode 0733 Flood Fill (Easy)
- LeetCode 0200 Number of Islands (Medium)
- LeetCode 1765 Map of Highest Peak (Medium)
- LeetCode 1197 Minimum Knight Moves (Medium)
- LeetCode 1091 Shortest Path in Binary Matrix (Medium)
- LeetCode 1926 Nearest Exit from Entrance in Maze (Medium)
- LeetCode 1293 Shortest Path in a Grid with Obstacles Elimination (Hard)
- LeetCode 0752 Open the Lock (Medium)
- LeetCode 0127 Word Ladder (Hard)
- LeetCode 2059 Minimum Operations to Convert Number (Medium)
- LeetCode 0773 Sliding Puzzle (Hard)
- LeetCode 0847 Shortest Path Visiting All Nodes (Hard)
- LeetCode 0675 Cut Off Trees for Golf Event (Hard)
- LeetCode 1368 Minimum Cost to Make at Least One Valid Path in a Grid (Hard)
- LeetCode 2290 Minimum Obstacle Removal to Reach Corner (Hard)
- LeetCode 0490 The Maze (Medium)
- LeetCode 1219 Path with Maximum Gold (Medium)
- LeetCode 0473 Matchsticks to Square (Medium)
- LeetCode 0698 Partition to K Equal Sum Subsets (Medium)
- LeetCode 1723 Find Minimum Time to Finish All Jobs (Hard)
- LeetCode 2305 Fair Distribution of Cookies (Medium)
- LeetCode 0329 Longest Increasing Path in a Matrix (Hard)
- LeetCode 2328 Number of Increasing Paths in a Grid (Hard)
- LeetCode 0294 Flip Game II (Medium)
- LeetCode 1575 Count All Possible Routes (Hard)
- LeetCode 1444 Number of Ways of Cutting a Pizza (Hard)

### [backtracking](https://en.wikipedia.org/wiki/Backtracking) 回溯算法

- [LeetCode tag - Backtracking](https://leetcode.com/tag/backtracking/)
- LeetCode 0037 Sudoku Solver (Hard)
- LeetCode 0046 Permutations (Medium)
- LeetCode 0047 Permutations II (Medium)
- LeetCode 0051 N-Queens (Hard)
- LeetCode 0052 N-Queens II (Hard)
- LeetCode 0079 Word Search (Medium)

### [k-nearest neighbors algorithm](https://en.wikipedia.org/wiki/K-nearest_neighbors_algorithm)

### math 数学问题

- [LeetCode tag - Math](https://leetcode.com/tag/math/)
- LeetCode 0204 Count Primes (Easy)
- LeetCode 0504 Base 7 (Easy)
- LeetCode 0172 Factorial Trailing Zeroes (Medium)
- LeetCode 0415 Add Strings (Easy)
- LeetCode 0326 Power of Three (Easy)
- LeetCode 0384 Shuffle an Array (Medium)
- LeetCode 0528 Random Pick with Weight (Medium)
- LeetCode 0382 Linked List Random Node (Medium)
- LeetCode 0168 Excel Sheet Column Title (Easy)
- LeetCode 0067 Add Binary (Easy)
- LeetCode 0238 Product of Array Except Self (Medium)
- LeetCode 0462 Minimum Moves to Equal Array Elements II (Medium)
- LeetCode 0169 Majority Element (Easy)
- LeetCode 0470 Implement Rand10() Using Rand7() (Medium)
- LeetCode 0202 Happy Number (Easy)
- [LeetCode 0509 Fibonacci Number](./src//leetcode_solutions/leetcode_0509_fibonacci_number.rs)
- [Greatest Common Divisor (GCD, 最大公约数)](./src/math/gcd_of_n_numbers.rs)
- [Least Common Multiple (LCM, 最小公倍数)](./src/math/lcm_of_n_numbers.rs)
- 冯诺依曼邻居问题 (离散数学)

### [string-matching algorithm](https://en.wikipedia.org/wiki/String-searching_algorithm)

- [LeetCode tag - String Matching](https://leetcode.com/tag/string-matching/)
- LeetCode 0028 Find the Index of the First Occurrence in a String (Medium)

### string 字符串问题

- [LeetCode tag - String](https://leetcode.com/tag/string/)
- LeetCode 0242 Valid Anagram (Easy)
- LeetCode 0205 Isomorphic Strings (Easy)
- LeetCode 0647 Palindromic Substrings (Medium)
- LeetCode 0696 Count Binary Substrings (Easy)
- LeetCode 0227 Basic Calculator II (Medium)
- LeetCode 0772 Basic Calculator III (Hard)
- LeetCode 0409 Longest Palindrome (Easy)
- LeetCode 0003 Longest Substring Without Repeating Characters (Medium)
- LeetCode 0005 Longest Palindromic Substring (Medium)

### [bit manipulation](https://en.wikipedia.org/wiki/Bit_manipulation) 位运算

- [LeetCode tag - Bit Manipulation](https://leetcode.com/tag/bit-manipulation/)
-
- LeetCode 0461 Hamming Distance (Easy)
- LeetCode 0190 Reverse Bits (Easy)
- LeetCode 0136 Single Number (Easy)
- LeetCode 0342 Power of Four (Easy)
- LeetCode 0318 Maximum Product of Word Lengths (Medium)
- LeetCode 0338 Counting Bits (Medium)
- LeetCode 0268 Missing Number (Easy)
- LeetCode 0693 Binary Number with Alternating Bits (Easy)
- LeetCode 0476 Number Complement (Easy)
- LeetCode 0260 Single Number III (Medium)

### [prefix sum](https://en.wikipedia.org/wiki/Prefix_sum) 前缀和

- C++ Standard Library `std::partial_sum` defined in `<numeric>`
-
- LeetCode 0467
- LeetCode 0523
- LeetCode 0560 Subarray Sum Equals K (Medium)
- LeetCode 0724
- LeetCode 0795
- LeetCode 0898
- LeetCode 0904
- LeetCode 0974
- LeetCode 0992
- LeetCode 1109
- LeetCode 1248

### [summed-area table](https://en.wikipedia.org/wiki/Summed-area_table) (also called `interal image` 积分图)

- [LeetCode 0303 Range Sum Query - Immutable (Easy)](./src/leetcode_solutions/leetcode_0303_range_sum_query_immutable.rs)
- [LeetCode 0304 Range Sum Query 2D - Immutable (Medium)](./src/leetcode_solutions/leetcode_0304_range_sum_query_2d_immutable.rs)
- [LeetCode 0307 Range Sum Query - Mutable (Medium)](./src/leetcode_solutions/leetcode_0307_range_sum_query_mutable.rs)

### [linear programming](https://en.wikipedia.org/wiki/Linear_programming) 线性规划

- 购物车凑满减
- network flow problem
- multicommodity flow problem
- Google uses linear programming to stabilize YouTube videos.
-
- [simplex algorithm](https://en.wikipedia.org/wiki/Simplex_algorithm)

### [cipher](https://en.wikipedia.org/wiki/Cipher) 密码学相关算法

- [Advanced Encryption Standard (AES)](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard)
- [Data Encryption Standard](https://en.wikipedia.org/wiki/Data_Encryption_Standard)
- [Triple DES](https://en.wikipedia.org/wiki/Triple_DES)
- [Blowfish](<https://en.wikipedia.org/wiki/Blowfish_(cipher)>)
- [Twofish](https://en.wikipedia.org/wiki/Twofish)
- [Serpent](<https://en.wikipedia.org/wiki/Serpent_(cipher)>)
-
- [RSA](<https://en.wikipedia.org/wiki/RSA_(cryptosystem)>)
- [Diffie–Hellman key exchange](https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange)
- [MD5](https://en.wikipedia.org/wiki/MD5)
-
- [Secure Hash Algorithm (SHA)](https://en.wikipedia.org/wiki/Secure_Hash_Algorithms)
- The Secure Hash Algorithms are a family of cryptographic hash functions, including
  - SHA-0
  - SHA-1
  - SHA-2
  - SHA-3
- If you're using an SHA algorithm for password hashing, use SHA-2 or SHA-3
- The gold standard for password-hashing functions is currently `bcrypt`
- [bcrypt](https://en.wikipedia.org/wiki/Bcrypt)

### [SimHash](https://en.wikipedia.org/wiki/SimHash)

- SimHash is a technique for quickly estimating how similar two sets are. The algorithm is used by the Google Crawler to find near duplicate pages.
- A teacher could use Simhash to see whether a student was copying an essay from the web.

### [HyperLogLog](https://en.wikipedia.org/wiki/HyperLogLog)

- HyperLogLog is an algorithm for the count-distinct problem, approximating the number of distinct elements in a multiset.
- not give an exact answer

### [Fourier transform (FT)](https://en.wikipedia.org/wiki/Fourier_transform)

- signal processing
- compress music (MP3 format)
- JPG format
- predict upcoming earthquakes
- analyze DNA

### [fast Fourier transform (FFT)](https://en.wikipedia.org/wiki/Fast_Fourier_transform)

### [naive Bayes classifier](https://en.wikipedia.org/wiki/Naive_Bayes_classifier) 朴素贝叶斯分类器

- spam filter 垃圾邮件过滤器

### [Gibbs sampling](https://en.wikipedia.org/wiki/Gibbs_sampling) 吉布斯采样

- statistical inference

### [parallel algorithm](https://en.wikipedia.org/wiki/Parallel_algorithm)

### [distributed algorithm](https://en.wikipedia.org/wiki/Distributed_algorithm)

### [MapReduce](https://en.wikipedia.org/wiki/MapReduce)

- MapReduce is a programming model and an associated implementation for processing and generating big data sets with a parallel, distributed algorithm on a cluster.

## 剑指 Offer (专项突击版)

- LeetCode 0029 Divide Two Integers (Medium)
- LeetCode 0067 Add Binary (Easy)
- LeetCode 0338 Counting Bits (Easy)
- LeetCode 0137 Single Number II (Medium)
- LeetCode 0318 Maximum Product of Word Lengths (Medium)
- LeetCode 0167 Two Sum II - Input Array Is Sorted (Medium)
- LeetCode 0015 3 Sum (Medium)
- LeetCode 0209 Minimum Size Subarray Sum (Medium)
- LeetCode 0713 Subarray Product Less Than K (Medium)
- LeetCode 0560 Subarray Sum Equals K (Medium)
- LeetCode 0525 Contiguous Array (Medium)
- LeetCode 0724 Find Pivot Index (Easy)
- LeetCode 0304 Range Sum Query 2D - Immutable (Medium)
- LeetCode 0567 Permutation in String (Medium)
- LeetCode 0438 Find All Anagrams in a String (Medium)
- LeetCode 0003 Longest Substring Without Repeating Characters (Medium)
- LeetCode 0076 Minimum Window Substring (Hard)
- LeetCode 0125 Valid Palindrome (Easy)
- LeetCode 0680 Valid Palindrome II (Easy)
- LeetCode 0647 Palindromic Substrings (Medium)
- LeetCode 0019 Remove Nth Node From End of List (Medium)
- LeetCode 0142 Linked List Cycle II (Medium)
- LeetCode 0160 Intersection of Two Linked Lists (Easy)
- LeetCode 0206 Reverse Linked List (Easy)
- LeetCode 0445 Add Two Numbers II (Medium)
- LeetCode 0143 Reorder List (Medium)
- LeetCode 0234 Palindrome Linked List (Easy)
- LeetCode 0430 Flatten a Multilevel Doubly Linked List (Medium)
- LeetCode 0708 Insert into a Sorted Circular Linked List (Medium)
- LeetCode 0380 Insert Delete GetRandom O(1) (Medium)
- LeetCode 0146 LRU Cache (Medium)
- LeetCode 0242 Valid Anagram (Easy)
- LeetCode 0049 Group Anagrams (Medium)
- LeetCode 0953 Verifying an Alien Dictionary (Easy)
- LeetCode 0539 Minimum Time Difference (Medium)
- LeetCode 0150 Evaluate Reverse Polish Notation (Medium)
- LeetCode 0735 Asteroid Collision (Medium)
- LeetCode 0739 Daily Temperatures (Medium)
- LeetCode 0084 Largest Rectangle in Histogram (Hard)
- LeetCode 0085 Maximal Rectangle (Hard)
- LeetCode 0346 Moving Average from Data Stream (Easy)
- LeetCode 0933 Number of Recent Calls (Easy)
- LeetCode 0919 Complete Binary Tree Inserter (Medium)
- LeetCode 0515 Find Largest Value in Each Tree Row (Medium)
- LeetCode 0513 Find Bottom Left Tree Value (Medium)
- LeetCode 0199 Binary Tree Right Side View (Medium)
- LeetCode 0814 Binary Tree Pruning (Medium)
- LeetCode 0297 Serialize and Deserialize Binary Tree (Hard)
- LeetCode 0129 Sum Root to Leaf Numbers (Medium)
- LeetCode 0437 Path Sum III (Medium)
- LeetCode 0124 Binary Tree Maximum Path Sum (Hard)
- LeetCode 0897 Increasing Order Search Tree (Easy)
- LeetCode 0285 Inorder Successor in BST (Medium)
- LeetCode 0538 Convert BST to Greater Tree (Medium)
- LeetCode 1038 Binary Search Tree to Greater Sum Tree (Medium)
- LeetCode 0173 Binary Search Tree Iterator (Medium)
- LeetCode 0653 Two Sum IV - Input is a BST (Easy)
- LeetCode 0220 Contains Duplicate III (Hard)
- LeetCode 0729 My Calendar I (Medium)
- LeetCode 0703 Kth Largest Element in a Stream (Easy)
- LeetCode 0347 Top K Frequent Elements (Medium)
- LeetCode 0373 Find K Pairs with Smallest Sums (Medium)
- LeetCode 0208 Implement Trie (Prefix Tree) (Medium)
- LeetCode 0648 Replace Words (Medium)
- LeetCode 0676 Implement Magic Dictionary (Medium)
- LeetCode 0820 Short Encoding of Words (Medium)
- LeetCode 0677 Map Sum Pairs (Medium)
- LeetCode 0421 Maximum XOR of Two Numbers in an Array (Medium)
- LeetCode 0035 Search Insert Position (Easy)
- LeetCode 0852 Peak Index in a Mountain Array (Medium)
- LeetCode 0540 Single Element in a Sorted Array (Medium)
- LeetCode 0528 Random Pick with Weight (Medium)
- LeetCode 0069 Sqrt(x) (Easy)
- LeetCode 0875 Koko Eating Bananas (Medium)
- LeetCode 0056 Merge Intervals (Medium)
- LeetCode 1122 Relative Sort Array (Easy)
- LeetCode 0215 Kth Largest Element in an Array (Medium)
- LeetCode 0148 Sort List (Medium)
- LeetCode 0023 Merge k Sorted Lists (Hard)
- LeetCode 0078 Subsets (Medium)
- LeetCode 0077 Combinations (Medium)
- LeetCode 0039 Combination Sum (Medium)
- LeetCode 0040 Combination Sum II (Medium)
- LeetCode 0046 Permutations (Medium)
- LeetCode 0047 Permutations II (Medium)
- LeetCode 0022 Generate Parentheses (Medium)
- LeetCode 0131 Palindrome Partitioning (Medium)
- LeetCode 0093 Restore IP Addresses (Medium)
- LeetCode 0746 Min Cost Climbing Stairs (Easy)
- LeetCode 0198 House Robber (Medium)
- LeetCode 0213 House Robber II (Medium)
- LeetCode 0256 Paint House (Medium)
- LeetCode 0926 Flip String to Monotone Increasing (Medium)
- LeetCode 0873 Length of Longest Fibonacci Subsequence (Medium)
- LeetCode 0132 Palindrome Partitioning II (Hard)
- LeetCode 1143 Longest Common Subsequence (Medium)
- LeetCode 0097 Interleaving String (Medium)
- LeetCode 0115 Distinct Subsequences (Hard)
- LeetCode 0062 Unique Paths (Medium)
- LeetCode 0064 Minimum Path Sum (Medium)
- LeetCode 0120 Triangle (Medium)
- LeetCode 0416 Partition Equal Subset Sum (Medium)
- LeetCode 0494 Target Sum (Medium)
- [LeetCode 0322 Coin Change (Medium)](./src/leetcode_solutions/leetcode_0322_coin_change.rs)
- LeetCode 0377 Combination Sum IV (Medium)
- LeetCode 0695 Max Area of Island (Medium)
- LeetCode 0785 Is Graph Bipartite (Medium)
- LeetCode 0542 01 Matrix (Medium)
- LeetCode 0127 Word Ladder (Hard)
- LeetCode 0752 Open the Lock (Medium)
- LeetCode 0797 All Paths From Source to Target (Medium)
- LeetCode 0399 Evaluate Division (Medium)
- LeetCode 0329 Longest Increasing Path in a Matrix (Hard)
- LeetCode 0210 Course Schedule II (Medium)
- LeetCode 0269 Alien Dictionary (Hard)
- LeetCode 0444 Sequence Reconstruction (Medium)
- LeetCode 0547 Number of Provinces (Medium)
- LeetCode 0839 Similar String Groups (Hard)
- LeetCode 0684 Redundant Connection (Medium)
- LeetCode 0128 Longest Consecutive Sequence (Medium)

## 程序员面试金典（第 6 版）

## TODO: 数据结构应用

- LeetCode 0716 Max Stack (Easy)
- LeetCode 0146 LRU Cache (Medium)
- LeetCode 0380 Insert Delete GetRandom O(1) (Medium)
- LeetCode 0432 All O'one Data Structure (Hard)
- LeetCode 0707 Design Linked List (Medium)
- LeetCode 0496 Next Greater Element I (Easy)
- LeetCode 0739 Daily Temperatures (Medium)
- LeetCode 0907 Sum of Subarray Minimums (Medium)
- LeetCode 0962 Maximum Width Ramp (Medium)
- LeetCode 0768 Max Chunks To Make Sorted II (Hard)
- LeetCode 2104 Sum of Subarray Ranges (Medium)
- LeetCode 1856 Maximum Subarray Min-Product (Medium)
- LeetCode 0239 Sliding Window Maximum (Hard)
- LeetCode 1499 Max Value of Equation (Hard)
- LeetCode 0862 Shortest Subarray with Sum at Least K (Hard)
- LeetCode 1425 Constrained Subsequence Sum (Hard)
- LeetCode 0291 Word Pattern II (Medium)
- LeetCode 0214 Shortest Palindrome (Hard)
- LeetCode 0336 Palindrome Pairs (Hard)
- LeetCode 1044 Longest Duplicate Substring (Hard)
- LeetCode 1316 Distinct Echo Substrings (Hard)
- LeetCode 1559 Detect Cycles in 2D Grid (Medium)
- LeetCode 0399 Evaluate Division (Medium)
- LeetCode 0959 Regions Cut By Slashes (Medium)
- LeetCode 0778 Swim in Rising Water (Hard)
- LeetCode 1202 Smallest String With Swaps (Medium)
- LeetCode 0803 Bricks Falling When Hit (Hard)
- LeetCode 0928 Minimize Malware Spread II (Hard)
- LeetCode 1697 Checking Existence of Edge Length Limited Paths (Hard)
- LeetCode 1579 Remove Max Number of Edges to Keep Graph Fully Traversable (Hard)
- LeetCode 0307 Range Sum Query - Mutable (Medium)
- LeetCode 1649 Create Sorted Array through Instructions (Hard)
- LeetCode 2179 Count Good Triplets in an Array (Hard)
- LeetCode 1505 Minimum Possible Integer After at Most K Adjacent Swaps On Digits
- LeetCode 0308 Range Sum Query 2D - Mutable (Hard)
- LeetCode 0315 Count of Smaller Numbers After Self (Hard)
- LeetCode 0327 Count of Range Sum (Hard)
- LeetCode 0493 Reverse Pairs (Hard)
- LeetCode 0673 Number of Longest Increasing Subsequence (Medium)
- LeetCode 1622 Fancy Sequence (Hard)
- LeetCode 0715 Range Module (Hard)
- LeetCode 0732 My Calendar III (Hard)
- LeetCode 2158 Amount of New Area Painted Each Day (Hard)
- LeetCode 2213 Longest Substring of One Repeating Character (Hard)
- LeetCode 0850 Rectangle Area II (Hard)

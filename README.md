# Data Structures and Algorithms in Rust

## Data Structures

### [array](<https://en.wikipedia.org/wiki/Array_(data_structure)>)

### [stack](<https://en.wikipedia.org/wiki/Stack_(abstract_data_type)>)

- [LeetCode tag - stack](https://leetcode.com/tag/stack/)
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
- monotonic stack 单调栈
- 常见问题模型：next greater element
- [LeetCode tag - monotonic stack](https://leetcode.com/tag/monotonic-stack/)

### [queue](<https://en.wikipedia.org/wiki/Queue_(abstract_data_type)>)

- [LeetCode tag - queue](https://leetcode.com/tag/queue/)
- FIFO: first in, first out
- [circular buffer](https://en.wikipedia.org/wiki/Circular_buffer) 循环队列
- 操作系统任务调度

### [double-ended queue](https://en.wikipedia.org/wiki/Double-ended_queue)

### [priority queue](https://en.wikipedia.org/wiki/Priority_queue)

- [LeetCode tag - heap (priority queue)](https://leetcode.com/tag/heap-priority-queue/)

### [double-ended priority queue](https://en.wikipedia.org/wiki/Double-ended_priority_queue)

### [linked list](https://en.wikipedia.org/wiki/Linked_list)

- [LeetCode tag - linked list](https://leetcode.com/tag/linked-list/)
- singly linked list
- [doubly linked list](https://en.wikipedia.org/wiki/Doubly_linked_list)
- multiply linked list
- circular linked list
- sentinel node 哨兵节点

### [hash table](https://en.wikipedia.org/wiki/Hash_table)

- [LeetCode tag - hash table](https://leetcode.com/tag/hash-table/)
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

### [tree](<https://en.wikipedia.org/wiki/Tree_(data_structure)>)

- [LeetCode tag - Tree](https://leetcode.com/tag/tree/)
-
- [binary tree](https://en.wikipedia.org/wiki/Binary_tree) 二叉树
-
- [Huffman coding](https://en.wikipedia.org/wiki/Huffman_coding) (also called `Huffman tree`)
-
- [binary search tree](https://en.wikipedia.org/wiki/Binary_search_tree) 二叉搜索树
- binary_search_tree.rs
- [LeetCode tag - binary search tree](https://leetcode.com/tag/binary-search-tree/)
-
- [self-balancing binary search tree](https://en.wikipedia.org/wiki/Self-balancing_binary_search_tree)
-
- [AVL tree](https://en.wikipedia.org/wiki/AVL_tree) 平衡二叉树
- avl_tree.rs
-
- [red-black tree](https://en.wikipedia.org/wiki/Red%E2%80%93black_tree) 红黑树
  - `Completely Fair Schedular` used in current Linux kernels
  - `epoll` system call
- red_black_tree.rs
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
- treap.rs
-
- [scapegoat tree](https://en.wikipedia.org/wiki/Scapegoat_tree)
-
- [tango tree](https://en.wikipedia.org/wiki/Tango_tree)
-
- [k-d tree](https://en.wikipedia.org/wiki/K-d_tree)
  - nearest neighbor search
  - range search
-
- [m-ary tree](https://en.wikipedia.org/wiki/M-ary_tree) (also called k-ary tree)
- [quadtree](https://en.wikipedia.org/wiki/Quadtree) 四叉树
- [octree](https://en.wikipedia.org/wiki/Octree) 八叉树
-
- [trie](https://en.wikipedia.org/wiki/Trie) 字典树、前缀树，应用于打字预测、自动补全、拼写检查等场景
- trie.rs
- [LeetCode tag - trie](https://leetcode.com/tag/trie/)
-
- [segment tree](https://en.wikipedia.org/wiki/Segment_tree) 线段树
- segment_tree.rs
- [LeetCode tag - segment tree](https://leetcode.com/tag/segment-tree/)
-
- [Fenwick tree](https://en.wikipedia.org/wiki/Fenwick_tree) (binary indexed tree)
- [LeetCode tag - binary indexed tree](https://leetcode.com/tag/binary-indexed-tree/)

### [graph](<https://en.wikipedia.org/wiki/Graph_(abstract_data_type)>)

- [adjacency list](https://en.wikipedia.org/wiki/Adjacency_list) 邻接表
- [adjacency matrix](https://en.wikipedia.org/wiki/Adjacency_matrix) 邻接矩阵
- [incidence matrix](https://en.wikipedia.org/wiki/Incidence_matrix) 关联矩阵
- graph.rs
- [LeetCode tag - Graph](https://leetcode.com/tag/graph/)
-
- [breadth-first search](https://en.wikipedia.org/wiki/Breadth-first_search) 广度优先搜索
- breadth_first_search.rs
- [LeetCode tag - Breadth-First Search](https://leetcode.com/tag/breadth-first-search/)
-
- [depth-first search](https://en.wikipedia.org/wiki/Depth-first_search) 深度优先搜索
- depth_first_search.rs
- [LeetCode tag - Depth-First Search](https://leetcode.com/tag/depth-first-search/)
-
- [topological sort](https://en.wikipedia.org/wiki/Topological_sorting) 拓扑排序
- topological_sort.rs
- [Rosetta Code - Topological sort](https://rosettacode.org/wiki/Topological_sort)
- [LeetCode tag - Topological Sort](https://leetcode.com/tag/topological-sort/)
-
- [strongly connected components](https://en.wikipedia.org/wiki/Strongly_connected_component) 强连通分量
- strongly_connected_components.rs
-
- [best-first search](https://en.wikipedia.org/wiki/Best-first_search) 最佳优先搜索

#### [shortest path problem](https://en.wikipedia.org/wiki/Shortest_path_problem) 最短路径问题

- [Dijkstra's algorithm (/ˈdaɪkstrəz/)](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
- dijkstra.rs
-
- [Bellman–Ford algorithm](https://en.wikipedia.org/wiki/Bellman%E2%80%93Ford_algorithm)
- bellman_ford.rs
-
- [A\* search algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm)
-
- [Prim's algorithm](https://en.wikipedia.org/wiki/Prim%27s_algorithm)

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
- heap.rs
- [LeetCode tag - heap (priority queue)](https://leetcode.com/tag/heap-priority-queue/)

### [skip list](https://en.wikipedia.org/wiki/Skip_list) 跳跃表 跳转表

### [disjoint-set](https://en.wikipedia.org/wiki/Disjoint-set_data_structure) (also called `union-find`) 并查集

- union_find.rs
- [LeetCode tag - Union Find](https://leetcode.com/tag/union-find/)

### [Bloom filter](https://en.wikipedia.org/wiki/Bloom_filter) 布隆过滤器

- 可用于检索一个元素是否在一个集合中
- 优点是空间效率和查询效率都远超一般的数据结构
- 缺点是有一定的识别误差率且删除较困难
- 本质上是一个概率性数据结构
-
- 应用于 redis
- 数据库用 Bloom filter 来减少实际存取 disk 的 IO 开销
- Chromium 浏览器验证大量恶意链接
- Medium 避免推荐已经推荐过的文章
-
- bloom_filter.rs
- [Rust Algorithm Club - Bloom filter](https://rust-algo.club/collections/bloom_filter/)

### [cuckoo filter](https://en.wikipedia.org/wiki/Cuckoo_filter) 布谷鸟过滤器

- 是改进过的 Bloom filter
- 支持动态添加和删除项
- 查找性能高于 Bloom filter，即使当期接近满载
- 比其他 Bloom filter 的替代品更容易实现，比如 quotient filter
- 应用于 redis

### [quotient filter](https://en.wikipedia.org/wiki/Quotient_filter) 商过滤器

### [count-min sketch](https://en.wikipedia.org/wiki/Count%E2%80%93min_sketch)

### [inverted index](https://en.wikipedia.org/wiki/Inverted_index) 反向索引 倒排索引

## Algorithms

### [big O notation](https://en.wikipedia.org/wiki/Big_O_notation)

### [time complexity](https://en.wikipedia.org/wiki/Time_complexity)

### [space complexity](https://en.wikipedia.org/wiki/Space_complexity)

### [amortized analysis](https://en.wikipedia.org/wiki/Amortized_analysis) 分摊分析 - 评定算法性能的一种重要尺度

### [divide-and-conquer algorithm](https://en.wikipedia.org/wiki/Divide-and-conquer_algorithm) 分治

- [LeetCode tag - Divide and Conquer](https://leetcode.com/tag/divide-and-conquer/)
-
- D&C algorithms are recursive algorithms. To solve a problem using D&C, there are two steps:
  - Figure out the base case. This should be the simplest possible case.
  - Divide or decrease your problem until it becomes the base case.
-
- 力扣 LCP 04 Broken Board Dominoes (棋盘覆盖问题)
- 循环赛日程安排问题
- 输油管道问题

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
- [knapsack problem](https://en.wikipedia.org/wiki/Knapsack_problem) 背包问题
- [knight's tour problem](https://en.wikipedia.org/wiki/Knight%27s_tour)
- [subset sum problem](https://en.wikipedia.org/wiki/Subset_sum_problem) 子集之和问题

### [backtracking](https://en.wikipedia.org/wiki/Backtracking) 回溯算法

- [LeetCode tag - Backtracking](https://leetcode.com/tag/backtracking/)

### [greedy algorithm](https://en.wikipedia.org/wiki/Greedy_algorithm) 贪心算法

- graph coloring problem
- NP-complete problems
- traveling salesman problem
-
- [LeetCode tag - Greedy](https://leetcode.com/tag/greedy/)

### [pointer jumping technique](https://en.wikipedia.org/wiki/Pointer_jumping) 双指针与滑动窗口

- [two pointers technique](https://leetcode.com/articles/two-pointer-technique/) 双指针
- [sliding window technique](https://leetcode.com/discuss/study-guide/1773891/Sliding-Window-Technique-and-Question-Bank) 滑动窗口
-
- [LeetCode tag - Two Pointers](https://leetcode.com/tag/two-pointers/)
- [LeetCode tag - Sliding Window](https://leetcode.com/tag/sliding-window/)

### [sorting algorithm](https://en.wikipedia.org/wiki/Sorting_algorithm)

- [Rosetta Code - Sorting algorithms](https://rosettacode.org/wiki/Category:Sorting_Algorithms)
- [Wikipedia Talk:Sorting algorithm](https://en.wikipedia.org/wiki/Talk%3ASorting_algorithm)
- 评价排序算法除了看时间空间复杂度，还要看稳定性
-
- [LeetCode tag - Sorting](https://leetcode.com/tag/sorting/)
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
- 如果要查找的数据量很小，则没必要先排序再二分搜索；如果数据量很大，排序很耗时且内存消耗大，或许使用线性搜索性能更好；实际项目中数据量不大不小，很适合使用二分搜索。
- [LeetCode tag - Binary Search](https://leetcode.com/tag/binary-search/)
-
- [interpolation search](https://en.wikipedia.org/wiki/Interpolation_search) 插值搜索
- [interpolation_search.rs](./src/searching/interpolation_search.rs)
-
- [exponential search](https://en.wikipedia.org/wiki/Exponential_search) 指数搜索
- [exponential_search.rs](./src/searching/exponential_search.rs)
-
- [Fibonacci search technique](https://en.wikipedia.org/wiki/Fibonacci_search_technique)

### [k-nearest neighbors algorithm](https://en.wikipedia.org/wiki/K-nearest_neighbors_algorithm)

### [string-matching algorithm](https://en.wikipedia.org/wiki/String-searching_algorithm)

- [LeetCode tag - String Matching](https://leetcode.com/tag/string-matching/)

### [bit manipulation](https://en.wikipedia.org/wiki/Bit_manipulation) 位运算

- [LeetCode tag - Bit Manipulation](https://leetcode.com/tag/bit-manipulation/)

### math 数学问题

- [LeetCode tag - Math](https://leetcode.com/tag/math/)
- [Greatest Common Divisor (GCD, 最大公约数)](./src/math/gcd_of_n_numbers.rs)
- [Least Common Multiple (LCM, 最小公倍数)](./src/math/lcm_of_n_numbers.rs)
- 冯诺依曼邻居问题 (离散数学)

### [Minimax](https://en.wikipedia.org/wiki/Minimax) 极小极大化

- game theory
- [LeetCode tag - game theory](https://leetcode.com/tag/game-theory/)

### geometry 几何学

- [LeetCode tag - geometry](https://leetcode.com/tag/geometry/)

### matrix 矩阵

- [LeetCode tag - matrix](https://leetcode.com/tag/matrix/)

### [prefix sum](https://en.wikipedia.org/wiki/Prefix_sum) 前缀和

- C++ Standard Library `std::partial_sum` defined in `<numeric>`
- [summed-area table](https://en.wikipedia.org/wiki/Summed-area_table) (also called `interal image` 积分图)
- [LeetCode tag - prefix sum](https://leetcode.com/tag/prefix-sum/)

### [linear programming](https://en.wikipedia.org/wiki/Linear_programming) 线性规划

- 购物车凑满减
- network flow problem
- multicommodity flow problem
- Google uses linear programming to stabilize YouTube videos.
-
- [simplex algorithm](https://en.wikipedia.org/wiki/Simplex_algorithm)

### [cache algorithms](https://en.wikipedia.org/wiki/Cache_replacement_policies)

- [least recently used](https://en.wikipedia.org/wiki/Cache_replacement_policies#LRU) (LRU) 缓存淘汰算法
- 一种内存管理算法，常用于虚拟页式存储、数据缓存
- 原理简单概括为：如果数据最近被访问过，那么将来被访问的几率也更高
-
- [Pseudo-LRU](https://en.wikipedia.org/wiki/Pseudo-LRU) (PLRU)
-
- [least-frequently used](https://en.wikipedia.org/wiki/Least_frequently_used) (LFU)

### [page replacement algorithm](https://en.wikipedia.org/wiki/Page_replacement_algorithm)

### [consistent hashing](https://en.wikipedia.org/wiki/Consistent_hashing) 一致性哈希算法

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

### [blockchain](https://en.wikipedia.org/wiki/Blockchain) 区块链

- 区块链技术是利用链式数据结构来验证与存储数据、利用分布式节点共识算法来生成和更新数据、利用密码学来保证数据传输和访问安全、利用自动化脚本组成的智能合约来编程和操作数据的一种全新的分布式基础架构与计算范式
- 简单来说，区块链就是去中心化的分布式账本
-
- [Bitcoin: A Peer-to-Peer Electronic Cash System](https://bitcoin.org/bitcoin.pdf) 比特币白皮书
- [比特币白皮书中文版](https://github.com/xiaolai/bitcoin-whitepaper-chinese-translation)
- [Bitcoin](https://en.wikipedia.org/wiki/Bitcoin) 比特币
- [Bitcoin Wiki](https://en.bitcoin.it/wiki/Main_Page)
- [BitcoinWiki](https://en.bitcoinwiki.org/wiki/Main_Page)
-
- [Base58](https://en.bitcoin.it/wiki/Base58Check_encoding)
- 用于表示 Bitcoin 钱包地址
-
- [Merkle tree](https://en.wikipedia.org/wiki/Merkle_tree)

## LeetCode Problems

### LeetCode Problems Part I

- LeetCode 0056 Merge Intervals <`Medium`>
- [LeetCode 0020 Valid Parentheses](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0020_valid_parentheses.rs) <`Easy`> <`stack`>
- LeetCode 0394 Decode String <`Medium`> <`stack`>
- LeetCode 0155 Min Stack <`Medium`> <`stack`>
- LeetCode 1249 Minimum Remove to Make Valid Parentheses <`Medium`> <`stack`>
- [LeetCode 0739 Daily Temperatures](./src/leetcode_solutions/leetcode_0700_0799/leetcode_0739_daily_temperatures.rs) <`Medium`> <`stack`> <`monotonic stack`>
- LeetCode 0084 Largest Rectangle in Histogram <`Hard`> <`stack`> <`monotonic stack`>
-
- LeetCode 0088 Merge Sorted Array <`Easy`> <`two pointers`>
- [LeetCode 0125 Valid Palindrome](./src/leetcode_solutions/leetcode_0100_0199/leetcode_0125_valid_palindrome.rs) <`Easy`> <`two pointers`>
- LeetCode 0283 Move Zeroes <`Easy`> <`two pointers`>
- LeetCode 0844 Backspace String Compare <`Easy`> <`two pointers`> <`stack`>
- LeetCode 0189 Rotate Array <`Medium`> <`math`> <`two pointers`>
- [LeetCode 0015 3Sum](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0015_3sum.rs) <`Medium`> <`two pointers`>
- LeetCode 0075 Sort Colors <`Medium`> <`two pointers`>
- [LeetCode 0021 Merge Two Sorted Lists](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0021_merge_two_sorted_lists.rs) <`Easy`> <`linked list`>
- LeetCode 0206 Reverse Linked List <`Easy`> <`linked list`>
- LeetCode 0092 Reverse Linked List II <`Medium`> <`linked list`>
- LeetCode 0203 Remove Linked List Elements <`Easy`> <`linked list`>
- LeetCode 0237 Delete Node in a Linked List <`Easy`> <`linked list`>
- LeetCode 0328 Odd Even Linked List <`Medium`> <`linked list`>
- LeetCode 0876 Middle of the Linked List <`Easy`> <`linked list`> <`two pointers`>
- LeetCode 0234 Palindrome Linked List <`Easy`> <`linked list`> <`two pointers`> <`stack`>
- LeetCode 0019 Remove Nth Node From End of List <`Medium`> <`linked list`> <`two pointers`>
- [LeetCode 0028 Find the Index of the First Occurrence in a String](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0028_find_the_index_of_the_first_occurrence_in_a_string.rs) <`Medium`> <`two pointers`> <`string matching`>
- LeetCode 0143 Reorder List <`Medium`> <`linked list`> <`two pointers`> <`stack`>
- LeetCode 0141 Linked List Cycle <`Easy`> <`hash table`> <`linked list`> <`two pointers`> (no Rust solution)
- LeetCode 0142 Linked List Cycle II <`Medium`> <`hash table`> <`linked list`> <`two pointers`> (no Rust solution)
- LeetCode 0114 Flatten Binary Tree to Linked List <`Medium`> <`linked list`> <`stack`> <`dfs`> <`binary tree`>
- LeetCode 0430 Flatten a Multilevel Doubly Linked List <`Medium`> <`linked list`> <`doubly linked list`> <`dfs`>
- LeetCode 0002 Add Two Numbers <`Medium`> <`linked list`> <`math`>
- LeetCode 0445 Add Two Numbers II <`Medium`> <`linked list`> <`math`> <`stack`>
- LeetCode 0148 Sort List <`Medium`> <`linked list`> <`two pointers`> <`divide and conquer`> <`merge sort`>
- LeetCode 0160 Intersection of Two Linked Lists <`Easy`> <`hash table`> <`linked list`> <`two pointers`>
- LeetCode 0138 Copy List with Random Pointer <`Medium`> <`hash table`> <`linked list`>
- LeetCode 0167 Two Sum II - Input Array Is Sorted <`Medium`> <`two pointers`> <`binary search`>
-
- [LeetCode 0001 Two Sum](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0001_two_sum.rs) <`Easy`> <`hash table`>
- LeetCode 0217 Contains Duplicate <`Easy`> <`hash table`>
- [LeetCode 0242 Valid Anagram](./src/leetcode_solutions/leetcode_0200_0299/leetcode_0242_valid_anagram.rs) <`Easy`> <`hash table`>
- [LeetCode 0049 Group Anagrams](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0049_group_anagrams.rs) <`Medium`> <`hash table`>
- [LeetCode 0387 First Unique Character in a String](./src/leetcode_solutions/leetcode_0300_0399/leetcode_0387_first_unique_character_in_a_string.rs) <`Easy`> <`hash table`> <`queue`>
-
- [LeetCode 0146 LRU Cache](./src/leetcode_solutions/leetcode_0100_0199/leetcode_0146_lru_cache.rs) <`Medium`> <`hash table`> <`linked list`> <`doubly linked list`>
- LeetCode 0287 Find the Duplicate Number <`Medium`> <`two pointers`> <`binary search`> <`bit manipulation`>
-
- [LeetCode 0023 Merge k Sorted Lists](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0023_merge_k_sorted_lists.rs) <`Hard`> <`linked list`> <`divide and conquer`> <`heap (priority queue)`> <`merge sort`>
- LeetCode 0003 Longest Substring Without Repeating Characters <`Medium`> <`hash table`> <`sliding window`>
- LeetCode 0713 Subarray Product Less Than K <`Medium`> <`sliding window`>
- LeetCode 0340 Longest Substring with At Most K Distinct Characters <`Medium`> <`hash table`> <`sliding window`>
- LeetCode 0438 Find All Anagrams in a String <`Medium`> <`hash table`> <`sliding window`>
- LeetCode 0076 Minimum Window Substring <`Hard`> <`hash table`> <`sliding window`>
- LeetCode 0239 Sliding Window Maximum <`Hard`> <`queue`> <`sliding window`> <`heap (priority queue)`> <`monotonic stack`>
- LeetCode 0567 Permutation in String <`Medium`> <`hash table`> <`two pointers`> <`sliding window`>
-
- [LeetCode 0455 Assign Cookies](./src/leetcode_solutions/leetcode_0400_0499/leetcode_0455_assign_cookies.rs) <`Easy`> <`two pointers`> <`greedy`>
- LeetCode 0680 Valid Palindrome II <`Easy`> <`two pointers`> <`greedy`>
- LeetCode 0202 Happy Number <`Easy`> <`hash table`> <`math`> <`two pointers`>
- [LeetCode 0011 Container With Most Water](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0011_container_with_most_water.rs) <`Medium`> <`two pointers`> <`greedy`>
- LeetCode 0295 Find Median from Data Stream <`Hard`> <`two pointers`> <`heap (priority queue)`>
- [LeetCode 0215 Kth Largest Element in an Array](./src/leetcode_solutions/leetcode_0200_0299/leetcode_0215_kth_largest_element_in_an_array.rs) <`Medium`> <`divide and conquer`> <`heap (priority queue)`>
- LeetCode 0347 Top K Frequent Elements <`Medium`> <`hash table`> <`divide and conquer`> <`heap (priority queue)`>
- LeetCode 0050 Pow(x, n) <`Medium`> <`math`>
-
- LeetCode 0121 Best Time to Buy and Sell Stock <`Easy`> <`dynamic programming`>
- LeetCode 0070 Climbing Stairs <`Easy`> <`math`> <`dynamic programming`>
- LeetCode 0118 Pascal's Triangle <`Easy`> <`dynamic programming`>
- [LeetCode 0509 Fibonacci Number](./src/leetcode_solutions/leetcode_0500_0599/leetcode_0509_fibonacci_number.rs) <`Easy`> <`math`> <`dynamic programming`>
- LeetCode 0746 Min Cost Climbing Stairs <`Easy`> <`dynamic programming`>
- [LeetCode 0322 Coin Change](./src/leetcode_solutions/leetcode_0300_0399/leetcode_0322_coin_change.rs) <`Medium`> <`dynamic programming`> <`bfs`>
- LeetCode 0300 Longest Increasing Subsequence <`Medium`> <`dynamic programming`> <`binary search`>
- LeetCode 0647 Palindromic Substrings <`Medium`> <`dynamic programming`>
- LeetCode 0005 Longest Palindromic Substring <`Medium`> <`dynamic programming`>
- LeetCode 0198 House Robber <`Medium`> <`dynamic programming`>
- LeetCode 0213 House Robber II <`Medium`> <`dynamic programming`>
- LeetCode 0337 House Robber III <`Medium`> <`dynamic programming`> <`dfs`> <`binary tree`>
- LeetCode 0091 Decode Ways <`Medium`> <`dynamic programming`>
- LeetCode 0120 Triangle <`Medium`> <`dynamic programming`>
- LeetCode 0152 Maximum Product Subarray <`Medium`> <`dynamic programming`>
- LeetCode 1143 Longest Common Subsequence <`Medium`> <`dynamic programming`>
- LeetCode 0053 Maximum Subarray <`Medium`> <`dynamic programming`> <`divide and conquer`>
- LeetCode 0062 Unique Paths <`Medium`> <`dynamic programming`> <`math`>
- LeetCode 0063 Unique Paths II <`Medium`> <`dynamic programming`> <`matrix`>
- LeetCode 0980 Unique Paths III <`Hard`> <`backtracking`> <`bit manipulation`> <`matrix`>
- LeetCode 0010 Regular Expression Matching <`Hard`> <`dynamic programming`>
- LeetCode 0064 Minimum Path Sum <`Medium`> <`dynamic programming`> <`matrix`>
- LeetCode 0122 Best Time to Buy and Sell Stock II <`Medium`> <`dynamic programming`> <`greedy`>
- LeetCode 0435 Non-overlapping Intervals <`Medium`> <`dynamic programming`> <`greedy`>
- LeetCode 0055 Jump Game <`Medium`> <`dynamic programming`> <`greedy`>
- LeetCode 0045 Jump Game II <`Medium`> <`dynamic programming`> <`greedy`>
- LeetCode 0279 Perfect Squares <`Medium`> <`dynamic programming`> <`bfs`> <`math`>
- LeetCode 0542 01 Matrix <`Medium`> <`dynamic programming`> <`bfs`> <`matrix`>
- LeetCode 0022 Generate Parentheses <`Medium`> <`dynamic programming`> <`backtracking`>
- LeetCode 0494 Target Sum <`Medium`> <`dynamic programming`> <`backtracking`>
- LeetCode 0131 Palindrome Partitioning <`Medium`> <`dynamic programming`> <`backtracking`>
- LeetCode 0139 Word Break <`Medium`> <`hash table`> <`dynamic programming`> <`trie`>
- LeetCode 0416 Partition Equal Subset Sum <`Medium`> <`dynamic programming`>
- [LeetCode 0072 Edit Distance](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0072_edit_distance.rs) <`Hard`> <`dynamic programming`>
- LeetCode 0044 Wildcard Matching <`Hard`> <`dynamic programming`> <`greedy`>
- LeetCode 0124 Binary Tree Maximum Path Sum <`Hard`> <`dynamic programming`> <`dfs`> <`binary tree`>
- LeetCode 0042 Trapping Rain Water <`Hard`> <`two pointers`> <`dynamic programming`> <`stack`> <`monotonic stack`>
- LeetCode 0085 Maximal Rectangle <`Hard`> <`dynamic programming`> <`stack`> <`monotonic stack`> <`matrix`>
- LeetCode 0046 Permutations <`Medium`> <`backtracking`>
- LeetCode 0047 Permutations II <`Medium`> <`backtracking`>
- LeetCode 0077 Combinations <`Medium`> <`backtracking`>
- LeetCode 0078 Subsets <`Medium`> <`backtracking`> <`bit manipulation`>
- LeetCode 0017 Letter Combinations of a Phone Number <`Medium`> <`hash table`> <`backtracking`>
- LeetCode 0169 Majority Element <`Easy`> <`hash table`> <`divide and conquer`>
- LeetCode 0079 Word Search <`Medium`> <`backtracking`> <`matrix`>
- LeetCode 0212 Word Search II <`Hard`> <`backtracking`> <`trie`> <`matrix`>
- LeetCode 0127 Word Ladder <`Hard`> <`hash table`> <`bfs`>
- LeetCode 0126 Word Ladder II <`Hard`> <`hash table`> <`backtracking`> <`bfs`>
- LeetCode 0036 Valid Sudoku <`Medium`> <`hash table`> <`matrix`>
- LeetCode 0037 Sudoku Solver <`Hard`> <`backtracking`> <`matrix`>
- LeetCode 0051 N-Queens <`Hard`> <`backtracking`>
- LeetCode 0052 N-Queens II <`Hard`> <`backtracking`>
-
- LeetCode 0069 Sqrt(x) <`Easy`> <`binary search`>
- LeetCode 0268 Missing Number <`Easy`> <`hash table`> <`math`> <`binary search`> <`bit manipulation`>
- LeetCode 0033 Seach in Rotated Sorted Array <`Medium`> <`binary search`>
- LeetCode 0540 Single Element in a Sorted Array <`Medium`> <`binary search`>
- LeetCode 0034 Find First and Last Position of Element in Sorted Array <`Medium`> <`binary search`>
- LeetCode 0240 Search a 2D Matrix II <`Medium`> <`binary search`> <`divide and conquer`> <`matrix`>
- LeetCode 0004 Median of Two Sorted Arrays <`Hard`> <`binary search`> <`divide and conquer`>
- LeetCode 0378 Kth Smallest Element in a Sorted Matrix <`Medium`> <`binary search`> <`heap (priority queue)`> <`matrix`>
-
- LeetCode 0144 Binary Tree Preorder Traversal <`Easy`> <`stack`> <`dfs`> <`binary tree`>
- LeetCode 0094 Binary Tree Inorder Traversal <`Easy`> <`stack`> <`dfs`> <`binary tree`>
- LeetCode 0102 Binary Tree Level Order Traversal <`Medium`> <`bfs`> <`binary tree`>
- LeetCode 0145 Binary Tree Postorder Traversal <`Easy`> <`stack`> <`dfs`> <`binary tree`>
- LeetCode 0107 Binary Tree Level Order Traversal II <`Medium`> <`bfs`> <`binary tree`>
- LeetCode 0103 Binary Tree Zigzag Level Order Traversal <`Medium`> <`bfs`> <`binary tree`>
- LeetCode 0110 Balanced Binary Tree <`Easy`> <`dfs`> <`binary tree`>
- LeetCode 0226 Invert Binary Tree <`Easy`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0617 Merge Two Binary Trees <`Easy`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0104 Maximum Depth of Binary Tree <`Easy`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0111 Minimum Depth of Binary Tree <`Easy`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0108 Convert Sorted Array to Binary Search Tree <`Easy`> <`divide and conquer`> <`binary search tree`> <`binary tree`>
- LeetCode 0236 Lowest Common Ancestor of a Binary Tree <`Medium`> <`dfs`> <`binary tree`>
- LeetCode 0116 Populating Next Right Pointers in Each Node <`Medium`> <`linked list`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0230 Kth Smallest Element in a BST <`Medium`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0101 Symmetric Tree <`Easy`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0105 Construct Binary Tree from Preorder and Inorder Traversal <`Medium`> <`hash table`> <`divide and conquer`> <`binary tree`>
- LeetCode 0098 Validate Binary Search Tree <`Medium`> <`dfs`> <`binary search tree`> <`binary tree`>
- LeetCode 0199 Binary Tree Right Side View <`Medium`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0297 Serialize and Deserialize Binary Tree <`Hard`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0173 Binary Search Tree Iterator <`Medium`> <`stack`> <`binary search tree`> <`binary tree`>
-
- LeetCode 0128 Longest Consecutive Sequence <`Medium`> <`hash table`> <`union find`>
- LeetCode 0207 Course Schedule <`Medium`> <`dfs`> <`bfs`> <`graph`> <`topological sort`>
- LeetCode 0210 Course Schedule II <`Medium`> <`dfs`> <`bfs`> <`graph`> <`topological sort`>
- LeetCode 1135 Connecting Cities with Minimum Cost <`Medium`> <`union find`> <`graph`> <`heap (priority queue)`> <`minimum spanning tree`>
- LeetCode 0130 Surrounded Regions <`Medium`> <`dfs`> <`bfs`> <`union find`> <`matrix`>
- LeetCode 0200 Number of Islands <`Medium`> <`dfs`> <`bfs`> <`union find`> <`matrix`>
- LeetCode 0695 Max Area of Island <`Medium`> <`dfs`> <`bfs`> <`union find`> <`matrix`>
- LeetCode 0547 Number of Provinces <`Medium`> <`dfs`> <`bfs`> <`union find`> <`graph`>
- LeetCode 0399 Evaluate Division <`Medium`> <`dfs`> <`bfs`> <`union find`> <`graph`>
- LeetCode 0684 Redundant Connection <`Medium`> <`dfs`> <`bfs`> <`union find`> <`graph`>
- LeetCode 0329 Longest Increasing Path in a Matrix <`Hard`> <`dynamic programming`> <`dfs`> <`bfs`> <`graph`> <`topological sort`> <`matrix`>
-
- [LeetCode 0208 Implement Trie (Prefix Tree)](./src/leetcode_solutions/leetcode_0200_0299/leetcode_0208_implement_trie.rs) <`Medium`> <`hash table`> <`trie`>
-
- LeetCode 0066 Plus One <`Easy`> <`math`>
- LeetCode 0326 Power of Three <`Easy`> <`math`>
- LeetCode 0204 Count Primes <`Easy`> <`math`>
- LeetCode 0380 Insert Delete GetRandom O(1) <`Medium`> <`hash table`> <`math`>
- LeetCode 0048 Rotate Image <`Medium`> <`math`> <`matrix`>
- LeetCode 0073 Set Matrix Zeroes <`Medium`> <`hash table`> <`matrix`>
- LeetCode 0150 Evaluate Reverse Polish Notation <`Medium`> <`math`> <`stack`>
- LeetCode 0224 Basic Calculator <`Hard`> <`math`> <`stack`>
- LeetCode 0227 Basic Calculator II <`Medium`> <`math`> <`stack`>
- LeetCode 0772 Basic Calculator III <`Hard`> <`math`> <`stack`>
- LeetCode 0149 Max Points on a Line <`Hard`> <`hash table`> <`math`>
- LeetCode 0067 Add Binary <`Easy`> <`math`> <`bit manipulation`>
- LeetCode 0136 Single Number <`Easy`> <`bit manipulation`>
- LeetCode 0231 Power of Two <`Easy`> <`math`> <`bit manipulation`>
- LeetCode 0190 Reverse Bits <`Easy`> <`divide and conquer`> <`bit manipulation`>
- LeetCode 0191 Number of 1 Bits <`Easy`> <`divide and conquer`> <`bit manipulation`>
- LeetCode 0137 Single Number II <`Medium`> <`bit manipulation`>
- LeetCode 0260 Single Number III <`Medium`> <`bit manipulation`>
- LeetCode 0318 Maximum Product of Word Lengths <`Medium`> <`bit manipulation`>
- LeetCode 0338 Counting Bits <`Medium`> <`dynamic programming`> <`bit manipulation`>
-
- LeetCode 0303 Range Sum Query - Immutable <`Easy`> <`prefix sum`>
- LeetCode 0560 Subarray Sum Equals K <`Medium`> <`hash table`> <`prefix sum`>
- LeetCode 0304 Range Sum Query 2D - Immutable <`Medium`> <`prefix sum`> <`matrix`>
- LeetCode 0307 Range Sum Query - Mutable <`Medium`> <`binary index tree`> <`segment tree`>
- LeetCode 0238 Product of Array Except Self <`Medium`> <`prefix sum`>
- LeetCode 0218 The Skyline Problem <`Hard`> <`divide and conquer`> <`binary index tree`> <`segment tree`> <`heap (priority queue)`>
- LeetCode 0315 Count of Smaller Numbers After Self <`Hard`> <`binary search`> <`divide and conquer`> <`binary index tree`> <`segment tree`> <`merge sort`>

### LeetCode Problems Part II

- LeetCode 0665 Non-decreasing Array <`Medium`>
- [LeetCode 0225 Implement Stack using Queues](./src/leetcode_solutions/leetcode_0200_0299/leetcode_0225_implement_stack_using_queues.rs) <`Easy`> <`stack`> <`queue`>
- [LeetCode 0232 Implement Queue using Stacks](./src/leetcode_solutions/leetcode_0200_0299/leetcode_0232_implement_queue_using_stacks.rs)) <`Easy`> <`stack`> <`queue`>
- LeetCode 1047 Remove All Adjacent Duplicates In String <`Easy`> <`stack`>
- LeetCode 0346 Moving Average from Data Stream <`Easy`> <`queue`>
- LeetCode 0735 Asteroid Collision <`Medium`> <`stack`>
- LeetCode 0071 Simplify Path <`Medium`> <`stack`>
- LeetCode 0856 Score of Parentheses <`Medium`> <`stack`>
- LeetCode 0946 Validate Stack Sequences <`Medium`> <`stack`>
- LeetCode 1190 Reverse Substrings Between Each Pair of Parentheses <`Medium`> <`stack`>
- LeetCode 0496 Next Greater Element I <`Easy`> <`hash table`> <`stack`>
- LeetCode 0503 Next Greater Element II <`Medium`> <`stack`> <`monotonic stack`>
- LeetCode 0316 Remove Duplicate Letters <`Medium`> <`stack`> <`monotonic stack`> <`greedy`>
- LeetCode 2104 Sum of Subarray Ranges <`Medium`> <`stack`> <`monotonic stack`>
- LeetCode 0402 Remove K Digits <`Medium`> <`stack`> <`monotonic stack`> <`greedy`>
- LeetCode 0456 132 Pattern <`Medium`> <`binary search`> <`stack`> <`monotonic stack`>
- LeetCode 0907 Sum of Subarray Minimums <`Medium`> <`dynamic programming`> <`stack`> <`monotonic stack`>
- LeetCode 0525 Contiguous Array <`Medium`> <`hash table`> <`prefix sum`>
- LeetCode 1124 Longest Well-Performing Interval <`Medium`> <`hash table`> <`stack`> <`monotonic stack`> <`prefix sum`>
- LeetCode 1856 Maximum Subarray Min-Product <`Medium`> <`stack`> <`monotonic stack`> <`prefix sum`>
- LeetCode 0321 Create Maximum Number <`Hard`> <`stack`> <`monotonic stack`> <`greedy`>
- LeetCode 0083 Remove Duplicates from Sorted List <`Easy`> <`linked list`>
- LeetCode 0708 Insert into a Sorted Circular Linked List <`Medium`> <`linked list`>
- LeetCode 1669 Merge In Between Linked Lists <`Medium`> <`linked list`>
- LeetCode 0024 Swap Nodes in Pairs <`Medium`> <`linked list`>
- LeetCode 0725 Split Linked List in Parts <`Medium`> <`linked list`>
- LeetCode 0382 Linked List Random Node <`Medium`> <`linked list`> <`math`>
- LeetCode 0025 Reverse Nodes in k-Group <`Hard`> <`linked list`>
- LeetCode 0622 Design Circular Queue <`Medium`> <`linked list`> <`queue`>
- LeetCode 0641 Design Circular Deque <`Medium`> <`linked list`> <`queue`>
- LeetCode 0832 Flipping an Image <`Easy`> <`two pointers`> <`matrix`>
- LeetCode 0082 Remove Duplicates from Sorted List II <`Medium`> <`linked list`> <`two pointers`>
- LeetCode 0086 Partition List <`Medium`> <`linked list`> <`two pointers`>
- LeetCode 0061 Rotate List <`Medium`> <`linked list`> <`two pointers`>
- LeetCode 0977 Squares of a Sorted Array <`Easy`> <`two pointers`>
- LeetCode 0524 Longest Word in Dictionary through Deleting <`Medium`> <`two pointers`>
- LeetCode 0633 Sum of Square Numbers <`Medium`> <`math`> <`two pointers`> <`binary search`>
- LeetCode 0460 LFU Cache <`Hard`> <`hash table`> <`linked list`> <`doubly linked list`>
- LeetCode 1248 Count Number of Nice Subarrays <`Medium`> <`hash table`> <`math`> <`sliding window`>
- LeetCode 0904 Fruit Into Baskets <`hash table`> <`sliding window`>
- LeetCode 0992 Subarrays with K Different Integers <`Hard`> <`hash table`> <`sliding window`>
- LeetCode 0424 Longest Repeating Character Replacement <`Medium`> <`hash table`> <`sliding window`>
- LeetCode 1044 Longest Duplicate Substring <`Hard`> <`binary search`> <`sliding window`>
- [LeetCode 0344 Reverse String](./src/leetcode_solutions/leetcode_0300_0399/leetcode_0344_reverse_string.rs) <`Easy`> <`two pointers`>
- LeetCode 0696 Count Binary Substrings <`Easy`> <`two pointers`>
- LeetCode 0016 3Sum Closest <`Medium`> <`two pointers`>
- LeetCode 0795 Number of Subarrays with Bounded Maximum <`Medium`> <`two pointers`>
- LeetCode 0870 Advantage Shuffle <`Medium`> <`two pointers`> <`greedy`>
- LeetCode 0106 Construct Binary Tree from Inorder and Postorder Traversal <`Medium`> <`divide and conquer`><`hash table`> <`binary tree`>
- LeetCode 0109 Convert Sorted List to Binary Search Tree <`Medium`> <`linked list`> <`divide and conquer`> <`binary search tree`> <`binary tree`>
- LeetCode 0367 Valid Perfect Square <`Easy`> <`binary search`>
- LeetCode 0860 Lemonade Change <`Easy`> <`greedy`>
- LeetCode 0452 Minimum Number of Arrows to Burst Balloons <`Medium`> <`greedy`>
- LeetCode 0135 Candy <`Hard`> <`greedy`>
- LeetCode 0205 Isomorphic Strings <`Easy`> <`hash table`>
- LeetCode 0448 Find All Numbers Disappeared in an Array (Easy) <`hash table`>
- LeetCode 0763 Partition Labels <`Medium`> <`hash table`> <`two pointers`> <`greedy`>
- [LeetCode 0409 Longest Palindrome](./src/leetcode_solutions/leetcode_0400_0499/leetcode_0409_longest_palindrome.rs) <`Easy`> <`hash table`> <`greedy`>
- [LeetCode 0594 Longest Harmonious Subsequence](./src/leetcode_solutions/leetcode_0500_0599/leetcode_0594_longest_harmonious_subsequence.rs) <`Easy`> <`hash table`>
- LeetCode 0953 Verifying an Alien Dictionary <`Easy`> <`hash table`>
- LeetCode 1122 Relative Sort Array <`Easy`> <`hash table`>
- LeetCode 0705 Design HashSet <`Easy`> <`hash table`> <`linked list`>
- LeetCode 0706 Design HashMap <`Easy`> <`hash table`> <`linked list`>
- LeetCode 0290 Word Pattern <`Easy`> <`hash table`>
- LeetCode 0291 Word Pattern II <`Medium`> <`hash table`> <`backtracking`>
- LeetCode 0726 Number of Atoms <`Hard`> <`hash table`> <`stack`>
- LeetCode 0769 Max Chunks to Make Sorted <`Medium`> <`stack`> <`greedy`> <`monotonic stack`>
- LeetCode 0605 Can Place Flowers <`Easy`> <`greedy`>
- LeetCode 1046 Last Stone Weight <`Easy`> <`heap (priority queue)`>
- LeetCode 0767 Reorganize String <`Medium`> <`hash table`> <`greedy`> <`heap (priority queue)`>
- LeetCode 0583 Delete Operation for Two Strings <`Medium`> <`dynamic programming`>
- LeetCode 0518 Coin Change II <`Medium`> <`dynamic programming`>
- LeetCode 0221 Maximal Square <`Medium`> <`dynamic programming`> <`matrix`>
- LeetCode 0039 Combination Sum <`Medium`> <`backtracking`>
- LeetCode 0040 Combination Sum II <`Medium`> <`backtracking`>
- LeetCode 0216 Combination Sum III <`Medium`> <`backtracking`>
- LeetCode 0377 Combination Sum IV <`Medium`> <`dynamic programming`>
- LeetCode 0473 Matchsticks to Square <`Medium`> <`dynamic programming`> <`backtracking`> <`bit manipulation`>
- LeetCode 0698 Partition to K Equal Sum Subsets <`Medium`> <`dynamic programming`> <`backtracking`> <`bit manipulation`>
- LeetCode 0093 Restore IP Addresses <`Medium`> <`backtracking`>
- LeetCode 0679 24 Game <`Hard`> <`math`> <`backtracking`>
- LeetCode 0090 Subsets II <`Medium`> <`backtracking`> <`bit manipulation`>
- LeetCode 0313 Super Ugly Number <`Medium`> <`math`> <`dynamic programming`>
- LeetCode 1425 Constrained Subsequence Sum <`Hard`> <`dynamic programming`> <`queue`> <`sliding window`> <`heap (priority queue)`>
- LeetCode 1575 Count All Possible Routes <`Hard`> <`dynamic programming`>
- LeetCode 1444 Number of Ways of Cutting a Pizza <`Hard`> <`dynamic programming`> <`matrix`>
- LeetCode 2218 Maximum Value of K Coins From Piles <`Hard`> <`dynamic programming`> <`prefix sum`>
- LeetCode 0312 Burst Balloons <`Hard`> <`dynamic programming`>
- LeetCode 0032 Longest Valid Parentheses <`Hard`> <`dynamic programming`> <`stack`>
- LeetCode 0887 Super Egg Drop <`Hard`> <`math`> <`binary search`> <`dynamic programming`>
- LeetCode 0309 Best Time to Buy and Sell Stock with Cooldown <`Medium`> <`dynamic programming`>
- LeetCode 0123 Best Time to Buy and Sell Stock III <`Hard`> <`dynamic programming`>
- LeetCode 0188 Best Time to Buy and Sell Stock IV <`Hard`> <`dynamic programming`>
- LeetCode 0714 Best Time to Buy and Sell Stock with Transaction Fee <`Medium`> <`dynamic programming`> <`greedy`>
- LeetCode 0403 Frog Jump <`Hard`> <`dynamic programming`>
- LeetCode 0410 Split Array Largest Sum <`Hard`> <`binary search`> <`dynamic programming`> <`greedy`>
- LeetCode 0294 Flip Game II <`Medium`> <`math`> <`dynamic programming`> <`backtracking`> <`game theory`>
- LeetCode 0464 Can I Win <`Medium`> <`math`> <`dynamic programming`> <`bit manipulation`> <`game theory`>
- LeetCode 0486 Predict the Winner <`Medium`> <`math`> <`dynamic programming`> <`game theory`>
- LeetCode 0898 Bitwise ORs of Subarrays <`Medium`> <`dynamic programming`> <`bit manipulation`>
- LeetCode 1723 Find Minimum Time to Finish All Jobs <`Hard`> <`dynamic programming`> <`backtracking`> <`bit manipulation`>
- LeetCode 2305 Fair Distribution of Cookies <`Medium`> <`dynamic programming`> <`backtracking`> <`bit manipulation`>
- LeetCode 0264 Ugly Number II <`Medium`> <`hash table`> <`math`> <`dynamic programming`> <`heap (priority queue)`>
- LeetCode 0787 Cheapest Flights Within K Stops <`Medium`> <`dynamic programming`> <`dfs`> <`bfs`> <`graph`> <`heap (priority queue)`>
- LeetCode 0467 Unique Substrings in Wraparound String <`Medium`> <`dynamic programming`>
- LeetCode 0096 Unique Binary Search Trees <`Medium`> <`math`> <`dynamic programming`> <`binary search tree`> <`binary tree`>
- LeetCode 0095 Unique Binary Search Trees II <`Medium`> <`dynamic programming`> <`backtracking`> <`binary search tree`> <`binary tree`>
- LeetCode 0741 Cherry Pickup <`Hard`> <`dynamic programming`> <`matrix`>
- LeetCode 1463 Cherry Pickup II <`Hard`> <`dynamic programming`> <`matrix`>
- LeetCode 0960 Delete Columns to Make Sorted III <`Hard`> <`dynamic programming`>
- LeetCode 1691 Maximum Height by Stacking Cuboids <`Hard`> <`dynamic programming`>
- LeetCode 1626 Best Team With No Conflicts <`Medium`> <`dynamic programming`>
- LeetCode 0712 Minimum ASCII Delete Sum for Two Strings <`Medium`> <`dynamic programming`>
- LeetCode 0673 Number of Longest Increasing Subsequence <`Medium`> <`dynamic programming`> <`binary index tree`> <`segment tree`>
- LeetCode 0354 Russian Doll Envelopes <`Hard`> <`dynamic programming`> <`binary search`>
- LeetCode 0132 Palindrome Partitioning II <`Hard`> <`dynamic programming`>
- LeetCode 0474 Ones and Zeroes <`Medium`> <`dynamic programming`>
- LeetCode 1049 Last Stone Weight II <`Medium`> <`dynamic programming`>
- LeetCode 0688 Knight Probability in Chessboard <`Medium`> <`dynamic programming`>
- LeetCode 0413 Arithmetic Slices <`Medium`> <`dynamic programming`>
- LeetCode 0650 2 Keys Keyboard <`Medium`> <`dynamic programming`> <`math`>
- LeetCode 0241 Different Ways to Add Parentheses <`Medium`> <`dynamic programming`> <`math`>
- LeetCode 0343 Integer Break <`Medium`> <`dynamic programming`> <`math`>
- LeetCode 0646 Maximum Length of Pair Chain <`Medium`> <`dynamic programming`> <`greedy`>
- LeetCode 0376 Wiggle Subsequence <`Medium`> <`dynamic programming`> <`greedy`>
- LeetCode 0256 Paint House <`Medium`> <`dynamic programming`>
- LeetCode 0926 Flip String to Monotone Increasing <`Medium`> <`dynamic programming`>
- LeetCode 0873 Length of Longest Fibonacci Subsequence <`Medium`> <`dynamic programming`> <`hash table`>
- LeetCode 0097 Interleaving String <`Medium`> <`dynamic programming`>
- LeetCode 0115 Distinct Subsequences <`Hard`> <`dynamic programming`>
- LeetCode 0879 Profitable Schemes <`Hard`> <`dynamic programming`>
- LeetCode 0973 K Closest Points to Origin <`Medium`> <`math`> <`divide and conquer`> <`heap (priority queue)`>
- LeetCode 0154 Find Minimum in Rotated Sorted Array II <`Hard`> <`binary search`>
- LeetCode 0621 Task Scheduler <`Medium`> <`hash table`> <`greedy`> <`heap (priority queue)`>
- LeetCode 0451 Sort Characters By Frequency <`Medium`> <`hash table`> <`heap (priority queue)`> <`bucket sort`>
- LeetCode 0912 Sort an Array <`Medium`> <`divide and conquer`> <`heap (priority queue)`> <`merge sort`> <`bucket sort`>
- LeetCode 0421 Maximum XOR of Two Numbers in an Array <`Medium`> <`hash table`> <`bit manipulation`> <`trie`>
- LeetCode 0336 Palindrome Pairs <`Hard`> <`hash table`> <`trie`>
- LeetCode 0692 Top K Frequent Words <`Medium`> <`hash table`> <`trie`> <`heap (priority queue)`> <`bucket sort`>
- LeetCode 0648 Replace Words <`Medium`> <`hash table`> <`trie`>
- LeetCode 0676 Implement Magic Dictionary <`Medium`> <`hash table`> <`trie`>
- LeetCode 0820 Short Encoding of Words <`Medium`> <`hash table`> <`trie`>
- LeetCode 0677 Map Sum Pairs <`Medium`> <`hash table`> <`trie`>
- LeetCode 0589 N-ary Tree Preorder Traversal <`Easy`> <`stack`> <`dfs`>
- LeetCode 0590 N-ary Tree Postorder Traversal <`Easy`> <`stack`> <`dfs`>
- LeetCode 0429 N-ary Tree Level Order Traversal <`Medium`> <`bfs`>
- LeetCode 0404 Sum of Left Leaves <`Easy`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0543 Diameter of Binary Tree <`Easy`> <`dfs`> <`binary tree`>
- LeetCode 0513 Find Bottom Left Tree Value <`Easy`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0515 Find Largest Value in Each Tree Row <`Medium`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0743 Network Delay Time <`Medium`> <`dfs`> <`bfs`> <`graph`> <`heap (priority queue)`>
- LeetCode 0653 Two Sum IV - Input is a BST <`Easy`> <`hash table`> <`two pointers`> <`dfs`> <`bfs`> <`binary tree`> <`binary search tree`>
- LeetCode 0100 Same Tree <`Easy`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0112 Path Sum <`Easy`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0113 Path Sum II <`Medium`> <`backtracking`> <`dfs`> <`binary tree`>
- LeetCode 0129 Sum Root to Leaf Numbers <`Medium`> <`dfs`> <`binary tree`>
- LeetCode 0704 Binary Search <`Easy`> <`binary search`>
- LeetCode 0035 Search Insert Position <`Easy`> <`binary search`>
- LeetCode 0349 Intersection of Two Arrays <`Easy`> <`hash table`> <`two pointers`> <`binary search`>
- LeetCode 0278 First Bad Version <`Easy`> <`binary search`>
- LeetCode 0285 Inorder Successor in BST <`Medium`> <`dfs`> <`binary search tree`> <`binary tree`>
- LeetCode 1038 Binary Search Tree to Greater Sum Tree <`Medium`> <`dfs`> <`binary search tree`> <`binary tree`>
- LeetCode 0814 Binary Tree Pruning <`Medium`> <`dfs`> <`binary tree`>
- LeetCode 0919 Complete Binary Tree Inserter <`Medium`> <`bfs`> <`binary tree`>
- LeetCode 0875 Koko Eating Bananas <`Medium`> <`binary search`>
- LeetCode 0074 Search a 2D Matrix <`Medium`> <`binary search`> <`matrix`>
- LeetCode 0081 Search in Rotated Sorted Array II <`Medium`> <`binary search`>
- LeetCode 0018 4Sum <`Medium`> <`two pointers`>
- LeetCode 0454 4Sum II <`Medium`> <`hash table`>
- LeetCode 0718 Maximum Length of Repeated Subarray <`Medium`> <`binary search`> <`dynamic programming`> <`sliding window`>
- LeetCode 0222 Count Complete Tree Nodes <`Medium`> <`binary search`> <`dfs`> <`binary tree`>
- LeetCode 0637 Average of Levels in Binary Tree <`Easy`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0257 Binary Tree Paths <`Easy`> <`backtracking`> <`dfs`> <`binary tree`>
- LeetCode 0662 Maximum Width of Binary Tree <`Medium`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0889 Construct Binary Tree from Preorder and Postorder Traversal <`Medium`> <`hash table`> <`divide and conquer`> <`binary tree`>
- LeetCode 1008 Construct Binary Search Tree from Preorder Traversal <`Medium`> <`stack`> <`monotonic stack`> <`binary search tree`> <`binary tree`>
- LeetCode 0897 Increasing Order Search Tree <`Easy`> <`stack`> <`dfs`> <`binary search tree`> <`binary tree`>
- LeetCode 0433 Minimum Genetic Mutation <`Medium`> <`hash table`> <`bfs`>
- LeetCode 0572 Subtree of Another Tree <`Easy`> <`dfs`> <`string matching`> <`binary tree`>
- LeetCode 0530 Minimum Absolute Difference in BST <`Easy`> <`dfs`> <`bfs`> <`binary tree`> <`binary tree`>
- LeetCode 0437 Path Sum III <`Medium`> <`dfs`> <`binary tree`>
- LeetCode 1110 Delete Nodes and Return Forest <`Medium`> <`dfs`> <`binary tree`>
- LeetCode 0538 Convert BST to Greater Tree <`Medium`> <`dfs`> <`binary search tree`> <`binary tree`>
- LeetCode 0099 Recover Binary Search Tree <`Hard`> <`dfs`> <`binary search tree`> <`binary tree`>
- LeetCode 0669 Trim a Binary Search Tree <`Medium`> <`dfs`> <`binary search tree`> <`binary tree`>
- LeetCode 0450 Delete Node in a BST <`Medium`> <`binary search tree`> <`binary tree`>
- LeetCode 0235 Lowest Common Ancestor of a Binary Search Tree <`Easy`> <`dfs`> <`binary search tree`> <`binary search`>
- LeetCode 0752 Open the Lock <`Medium`> <`hash table`> <`bfs`>
- LeetCode 1203 Sort Items by Groups Respecting Dependencies <`Hard`> <`dfs`> <`bfs`> <`graph`> <`topological sort`>
- LeetCode 2127 Maximum Employees to Be Invited to a Meeting <`Hard`> <`dfs`> <`graph`> <`topological sort`>
- LeetCode 0417 Pacific Atlantic Water Flow <`Medium`> <`dfs`> <`bfs`> <`matrix`>
- LeetCode 0529 Minesweeper <`Medium`> <`dfs`> <`bfs`> <`matrix`>
- LeetCode 1091 Shortest Path in Binary Matrix <`Medium`> <`bfs`> <`matrix`>
- LeetCode 1926 Nearest Exit from Entrance in Maze <`Medium`> <`bfs`> <`matrix`>
- LeetCode 1219 Path with Maximum Gold <`Medium`> <`backtracking`> <`matrix`>
- LeetCode 1059 All Paths from Source Lead to Destination <`Medium`> <`dfs`> <`graph`>
- LeetCode 1584 Min Cost to Connect All Points <`Medium`> <`union find`> <`graph`>
- LeetCode 0310 Minimum Height Trees <`Medium`> <`dfs`> <`bfs`> <`graph`> <`topological sort`>
- LeetCode 0490 The Maze <`Medium`> <`dfs`> <`bfs`> <`graph`>
- LeetCode 2203 Minimum Weighted Subgraph With the Required Paths <`Hard`> <`graph`>
- LeetCode 1168 Optimize Water Distribution in a Village <`Hard`> <`union find`> <`graph`>
- LeetCode 2328 Number of Increasing Paths in a Grid <`Hard`> <`dynamic programming`> <`dfs`> <`bfs`> <`graph`> <`topological sort`> <`matrix`>
- LeetCode 0690 Employee Importance <`Medium`> <`hash table`> <`dfs`> <`bfs`>
- LeetCode 0934 Shortest Bridge <`Medium`> <`dfs`> <`bfs`> <`matrix`>
- LeetCode 0994 Rotting Oranges <`Medium`> <`bfs`> <`matrix`>
- LeetCode 1162 As Far from Land as Possible <`Medium`> <`dynamic programming`> <`bfs`> <`matrix`>
- LeetCode 0815 Bus Routes <`Hard`> <`hash table`> <`bfs`>
- LeetCode 1765 Map of Highest Peak <`Medium`> <`bfs`> <`matrix`>
- LeetCode 2059 Minimum Operations to Convert Number <`Medium`> <`bfs`>
- LeetCode 1293 Shortest Path in a Grid with Obstacles Elimination <`Hard`> <`bfs`> <`matrix`>
- LeetCode 0773 Sliding Puzzle <`Hard`> <`bfs`> <`matrix`>
- LeetCode 0827 Making A Large Island <`Hard`> <`dfs`> <`bfs`> <`union find`> <`matrix`>
- LeetCode 0847 Shortest Path Visiting All Nodes <`Hard`> <`dynamic programming`> <`bit manipulation`> <`bfs`> <`graph`>
- LeetCode 0797 All Paths From Source to Target <`Medium`> <`backtracking`> <`dfs`> <`bfs`> <`graph`>
- LeetCode 0269 Alien Dictionary <`Hard`> <`dfs`> <`bfs`> <`graph`> <`topological sort`>
- LeetCode 1368 Minimum Cost to Make at Least One Valid Path in a Grid <`Hard`> <`bfs`> <`graph`> <`heap (priority queue)`> <`matrix`>
- LeetCode 2290 Minimum Obstacle Removal to Reach Corner <`Hard`> <`bfs`> <`graph`> <`heap (priority queue)`> <`matrix`>
- LeetCode 0785 Is Graph Bipartite <`Medium`> <`dfs`> <`bfs`> <`union find`> <`graph`>
- LeetCode 0721 Accounts Merge <`Medium`> <`dfs`> <`bfs`> <`union find`>
- LeetCode 1631 Path With Minimum Effort <`Medium`> <`binary search`> <`dfs`> <`bfs`> <`union find`> <`heap (priority queue)`> <`matrix`>
- LeetCode 1319 Number of Operations to Make Network Connected <`Medium`> <`dfs`> <`bfs`> <`union find`> <`graph`>
- LeetCode 0765 Couples Holding Hands <`Hard`> <`dfs`> <`bfs`> <`union find`> <`graph`> <`greedy`>
- LeetCode 1559 Detect Cycles in 2D Grid <`Medium`> <`dfs`> <`bfs`> <`union find`> <`matrix`>
- LeetCode 0959 Regions Cut By Slashes <`Medium`> <`dfs`> <`bfs`> <`union find`> <`graph`>
- LeetCode 1202 Smallest String With Swaps <`Medium`> <`hash table`> <`dfs`> <`bfs`> <`union find`>
- LeetCode 0839 Similar String Groups <`Hard`> <`dfs`> <`bfs`> <`union find`>
- LeetCode 0778 Swim in Rising Water <`Hard`> <`binary search`> <`dfs`> <`bfs`> <`union find`> <`heap (priority queue)`> <`matrix`>
- LeetCode 0803 Bricks Falling When Hit <`Hard`> <`union find`> <`matrix`>
- LeetCode 0928 Minimize Malware Spread II <`Hard`> <`dfs`> <`bfs`> <`union find`> <`matrix`>
- LeetCode 1489 Find Critical and Pseudo-Critical Edges in Minimum Spanning Tree <`Hard`> <`union find`> <`graph`>
- LeetCode 1697 Checking Existence of Edge Length Limited Paths <`Hard`> <`union find`> <`graph`>
- LeetCode 1579 Remove Max Number of Edges to Keep Graph Fully Traversable <`Hard`> <`union find`> <`graph`>
- LeetCode 0363 Max Sum of Rectangle No Larger Than K <`Hard`> <`binary search`> <`matrix`> <`prefix sum`>
- LeetCode 0370 Range Addition <`Medium`> <`prefix sum`>
- LeetCode 0528 Random Pick with Weight <`Medium`> <`math`> <`binary search`> <`prefix sum`>
- LeetCode 1894 Find the Student that Will Replace the Chalk <`Medium`> <`binary search`> <`prefix sum`>
- LeetCode 2132 Stamping the Grid <`Hard`> <`greedy`> <`matrix`> <`prefix sum`>
- LeetCode 0209 Minimum Size Subarray Sum <`Medium`> <`binary search`> <`sliding window`> <`prefix sum`>
- LeetCode 0153 Find Minimum in Rotated Sorted Array <`Medium`> <`binary search`>
- LeetCode 0162 Find Peak Element <`Medium`> <`binary search`>
- LeetCode 1499 Max Value of Equation <`Hard`> <`queue`> <`sliding window`> <`heap (priority queue)`>
- LeetCode 0373 Find K Pairs with Smallest Sums <`Medium`> <`heap (priority queue)`>
- LeetCode 0862 Shortest Subarray with Sum at Least K <`Hard`> <`binary search`> <`queue`> <`sliding window`> <`heap (priority queue)`> <`prefix sum`>
- LeetCode 1870 Minimum Speed to Arrive on Time <`Medium`> <`binary search`>
- LeetCode 1894 Find the Student that Will Replace the Chalk <`Medium`> <`binary search`> <`prefix sum`>
- LeetCode 1898 Maximum Number of Removable Characters <`Medium`> <`binary search`>
- LeetCode 0724 Find Pivot Index <`Easy`> <`prefix sum`>
- LeetCode 1109 Corporate Flight Bookings <`Medium`> <`prefix sum`>
- LeetCode 0523 Continuous Subarray Sum <`Medium`> <`hash table`> <`math`> <`prefix sum`>
- LeetCode 0974 Subarray Sums Divisible by K <`Medium`> <`hash table`> <`prefix sum`>
- LeetCode 0406 Queue Reconstruction by Height <`Medium`> <`greedy`> <`binary index tree`> <`segment tree`>
- LeetCode 1353 Maximum Number of Events That Can Be Attended <`Medium`> <`greedy`> <`heap (priority queue)`>
- LeetCode 1157 Online Majority Element In Subarray <`Hard`> <`binary search`> <`binary index tree`> <`segment tree`>
- LeetCode 0699 Falling Squares <`Hard`> <`segment tree`>
- LeetCode 0850 Rectangle Area II <`Hard`> <`segment tree`>
- LeetCode 0715 Range Module <`Hard`> <`segment tree`>
- LeetCode 0729 My Calendar I <`Medium`> <`binary search`> <`segment tree`>
- LeetCode 0732 My Calendar III <`Hard`> <`binary search`> <`segment tree`>
- LeetCode 1622 Fancy Sequence <`Hard`> <`math`> <`segment tree`>
- LeetCode 1505 Minimum Possible Integer After at Most K Adjacent Swaps On Digits <`Hard`> <`greedy`> <`binary index tree`> <`segment tree`>
- LeetCode 0493 Reverse Pairs <`Hard`> <`binary search`> <`divide and conquer`> <`binary index tree`> <`segment tree`> <`merge sort`>
- LeetCode 1649 Create Sorted Array through Instructions <`Hard`> <`binary search`> <`divide and conquer`> <`binary index tree`> <`segment tree`> <`merge sort`>
- LeetCode 0327 Count of Range Sum <`Hard`> <`binary search`> <`divide and conquer`> <`binary index tree`> <`segment tree`> <`merge sort`>
- LeetCode 2179 Count Good Triplets in an Array <`Hard`> <`binary search`> <`divide and conquer`> <`binary index tree`> <`segment tree`> <`merge sort`>
- LeetCode 2213 Longest Substring of One Repeating Character <`Hard`> <`segment tree`>
- LeetCode 0733 Flood Fill <`Easy`> <`dfs`> <`bfs`> <`matrix`>
- LeetCode 0214 Shortest Palindrome <`Hard`> <`string matching`>
- LeetCode 0566 Reshape the Matrix <`Easy`> <`matrix`>
- LeetCode 0867 Transpose Matrix <`Easy`> <`matrix`>
- LeetCode 0054 Spiral Matrix <`Medium`> <`matrix`>
- LeetCode 0059 Spiral Matrix II <`Medium`> <`matrix`>
- LeetCode 0462 Minimum Moves to Equal Array Elements II <`Medium`> <`math`>
- LeetCode 0168 Excel Sheet Column Title <`Easy`> <`math`>
- LeetCode 0415 Add Strings <`Easy`> <`math`>
- LeetCode 0504 Base 7 <`Easy`> <`math`>
- LeetCode 0628 Maximum Product of Three Numbers <`Easy`> <`math`>
- LeetCode 0384 Shuffle an Array <`Medium`> <`math`>
- LeetCode 0539 Minimum Time Difference <`Medium`> <`math`>
- LeetCode 0976 Largest Perimeter Triangle <`Easy`> <`math`> <`greedy`>
- LeetCode 0587 Erect the Fence <`Hard`> <`math`> <`geometry`>
- LeetCode 1232 Check If It Is a Straight Line <`Easy`> <`math`> <`geometry`>
- LeetCode 1266 Minimum Time Visiting All Points <`Easy`> <`math`> <`geometry`>
- LeetCode 0963 Minimum Area Rectangle II <`Medium`> <`math`> <`geometry`>
- LeetCode 0292 Nim Game <`Easy`> <`math`> <`game theory`>
- LeetCode 0342 Power of Four <`Easy`> <`bit manipulation`> <`math`>
- LeetCode 0389 Find the Difference <`Easy`> <`hash table`> <`bit manipulation`>
- LeetCode 0405 Convert a Number to Hexadecimal <`Easy`> <`math`> <`bit manipulation`>
- [LeetCode 0461 Hamming Distance](./src/leetcode_solutions/leetcode_0400_0499/leetcode_0461_hamming_distance.rs) <`Easy`> <`bit manipulation`>
- LeetCode 0476 Number Complement <`Easy`> <`bit manipulation`>
- LeetCode 0693 Binary Number with Alternating Bits <`Easy`> <`bit manipulation`>
- LeetCode 1318 Minimum Flips to Make a OR b Equal to c <`Medium`> <`bit manipulation`>
- LeetCode 1408 String Matching in an Array <`Easy`> <`string matching`>
- LeetCode 1206 Design Skiplist <`Hard`> <`linked list`>

### 常用算法面试题

- 做 LeetCode 算法题有助于编程语言的学习和实践，但这并不能增强该编程语言的工程能力，因此做题要适可而止
- 做题过程中要考虑边界问题，做到全面覆盖，争取一次提交就通过
-
- 栈与队列，哈希表，堆（优先队列）
- [LeetCode 0225 Implement Stack using Queues](./src/leetcode_solutions/leetcode_0200_0299/leetcode_0225_implement_stack_using_queues.rs) <`Easy`> <`stack`> <`queue`>
- [LeetCode 0232 Implement Queue using Stacks](./src/leetcode_solutions/leetcode_0200_0299/leetcode_0232_implement_queue_using_stacks.rs)) <`Easy`> <`stack`> <`queue`>
- [LeetCode 0020 Valid Parentheses](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0020_valid_parentheses.rs) <`Easy`> <`stack`>
- [LeetCode 0739 Daily Temperatures](./src/leetcode_solutions/leetcode_0700_0799/leetcode_0739_daily_temperatures.rs) <`Medium`> <`stack`> <`monotonic stack`>
- [LeetCode 0001 Two Sum](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0001_two_sum.rs) <`Easy`> <`hash table`>
- [LeetCode 0242 Valid Anagram](./src/leetcode_solutions/leetcode_0200_0299/leetcode_0242_valid_anagram.rs) <`Easy`> <`hash table`>
- [LeetCode 0049 Group Anagrams](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0049_group_anagrams.rs) <`Medium`> <`hash table`>
- [LeetCode 0387 First Unique Character in a String](./src/leetcode_solutions/leetcode_0300_0399/leetcode_0387_first_unique_character_in_a_string.rs) <`Easy`> <`hash table`> <`queue`>
- TODO:[LeetCode 0215 Kth Largest Element in an Array](./src/leetcode_solutions/leetcode_0200_0299/leetcode_0215_kth_largest_element_in_an_array.rs) <`Medium`> <`divide and conquer`> <`heap (priority queue)`>
- TODO:[LeetCode 0023 Merge k Sorted Lists](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0023_merge_k_sorted_lists.rs) <`Hard`> <`linked list`> <`divide and conquer`> <`heap (priority queue)`> <`merge sort`>
- [LeetCode 0409 Longest Palindrome](./src/leetcode_solutions/leetcode_0400_0499/leetcode_0409_longest_palindrome.rs) <`Easy`> <`hash table`> <`greedy`>
- [LeetCode 0594 Longest Harmonious Subsequence](./src/leetcode_solutions/leetcode_0500_0599/leetcode_0594_longest_harmonious_subsequence.rs) <`Easy`> <`hash table`>
-
- 双指针与滑动窗口
- [LeetCode 0011 Container With Most Water](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0011_container_with_most_water.rs) <`Medium`> <`two pointers`> <`greedy`>
- [LeetCode 0125 Valid Palindrome](./src/leetcode_solutions/leetcode_0100_0199/leetcode_0125_valid_palindrome.rs) <`Easy`> <`two pointers`>
- [LeetCode 0028 Find the Index of the First Occurrence in a String](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0028_find_the_index_of_the_first_occurrence_in_a_string.rs) <`Medium`> <`two pointers`> <`string matching`>
- [LeetCode 0455 Assign Cookies](./src/leetcode_solutions/leetcode_0400_0499/leetcode_0455_assign_cookies.rs) <`Easy`> <`two pointers`> <`greedy`>
- [LeetCode 0344 Reverse String](./src/leetcode_solutions/leetcode_0300_0399/leetcode_0344_reverse_string.rs) <`Easy`> <`two pointers`>
- [LeetCode 0015 3Sum](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0015_3sum.rs) <`Medium`> <`two pointers`>
- LeetCode 0287 Find the Duplicate Number <`Medium`> <`two pointers`> <`binary search`> <`bit manipulation`>
- LeetCode 0003 Longest Substring Without Repeating Characters <`Medium`> <`hash table`> <`sliding window`>
- LeetCode 0076 Minimum Window Substring <`Hard`> <`hash table`> <`sliding window`>
-
- 链表
- LeetCode 0206 Reverse Linked List <`Easy`> <`linked list`>
- LeetCode 0203 Remove Linked List Elements <`Easy`> <`linked list`>
- LeetCode 0234 Palindrome Linked List <`Easy`> <`linked list`> <`two pointers`> <`stack`>
- [LeetCode 0021 Merge Two Sorted Lists](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0021_merge_two_sorted_lists.rs) <`Easy`> <`linked list`>
- LeetCode 0160 Intersection of Two Linked Lists <`Easy`> <`hash table`> <`linked list`> <`two pointers`>
- (no Rust solution) LeetCode 0141 Linked List Cycle <`Easy`> <`hash table`> <`linked list`> <`two pointers`>
- (no Rust solution) LeetCode 0142 Linked List Cycle II <`Medium`> <`hash table`> <`linked list`> <`two pointers`>
- LeetCode 0025 Reverse Nodes in k-Group <`Hard`> <`linked list`>
- LeetCode 0148 Sort List <`Medium`> <`linked list`> <`two pointers`> <`divide and conquer`> <`merge sort`>
- LeetCode 0002 Add Two Numbers <`Medium`> <`linked list`> <`math`>
- LeetCode 0445 Add Two Numbers II <`Medium`> <`linked list`> <`math`> <`stack`>
- TODO:[LeetCode 0146 LRU Cache](./src/leetcode_solutions/leetcode_0100_0199/leetcode_0146_lru_cache.rs) <`Medium`> <`hash table`> <`linked list`> <`doubly linked list`>
- 实现单链表 [linked_list.rs](./src/data_structures/linked_list.rs)
-
- 十大常用排序算法，时间复杂度，空间复杂度，稳定性
- [bubble sort](https://en.wikipedia.org/wiki/Bubble_sort) 冒泡排序
- [bubble_sort.rs](./src/sorting/bubble_sort.rs)
- [selection sort](https://en.wikipedia.org/wiki/Selection_sort) 选择排序
- [selection_sort.rs](./src/sorting/selection_sort.rs)
- [insertion sort](https://en.wikipedia.org/wiki/Insertion_sort) 插入排序
- [insertion_sort.rs](./src/sorting/insertion_sort.rs)
- [quicksort](https://en.wikipedia.org/wiki/Quicksort) 快速排序
- [quicksort.rs](./src/sorting/quicksort.rs)
- [merge sort](https://en.wikipedia.org/wiki/Merge_sort) 归并排序
- [merge_sort.rs](./src/sorting/merge_sort.rs)
-
- [Shell sort](https://en.wikipedia.org/wiki/Shellsort) 希尔排序原理复杂度
- [heapsort](https://en.wikipedia.org/wiki/Heapsort) 堆排序原理复杂度
- [bucket sort](https://en.wikipedia.org/wiki/Bucket_sort) 桶排序原理复杂度
- [counting sort](https://en.wikipedia.org/wiki/Counting_sort) 计数排序原理复杂度
- [radix sort](https://en.wikipedia.org/wiki/Radix_sort) 基数排序原理复杂度
-
- 动态规划，回溯
- LeetCode 0121 Best Time to Buy and Sell Stock <`Easy`> <`dynamic programming`>
- LeetCode 0070 Climbing Stairs <`Easy`> <`math`> <`dynamic programming`>
- [LeetCode 0072 Edit Distance](./src/leetcode_solutions/leetcode_0000_0099/leetcode_0072_edit_distance.rs) <`Hard`> <`dynamic programming`>
- [LeetCode 0509 Fibonacci Number](./src/leetcode_solutions/leetcode_0500_0599/leetcode_0509_fibonacci_number.rs) <`Easy`> <`math`> <`dynamic programming`>
- [LeetCode 0322 Coin Change](./src/leetcode_solutions/leetcode_0300_0399/leetcode_0322_coin_change.rs) <`Medium`> <`dynamic programming`> <`bfs`>
- LeetCode 0053 Maximum Subarray <`Medium`> <`dynamic programming`> <`divide and conquer`>
- LeetCode 0300 Longest Increasing Subsequence <`Medium`> <`dynamic programming`> <`binary search`>
- LeetCode 0647 Palindromic Substrings <`Medium`> <`dynamic programming`>
- LeetCode 0005 Longest Palindromic Substring <`Medium`> <`dynamic programming`>
- LeetCode 0198 House Robber <`Medium`> <`dynamic programming`>
- LeetCode 1143 Longest Common Subsequence <`Medium`> <`dynamic programming`>
- LeetCode 0062 Unique Paths <`Medium`> <`dynamic programming`> <`math`>
- LeetCode 0542 01 Matrix <`Medium`> <`dynamic programming`> <`bfs`> <`matrix`>
- LeetCode 0139 Word Break <`Medium`> <`hash table`> <`dynamic programming`> <`trie`>
- LeetCode 0046 Permutations <`Medium`> <`backtracking`>
- LeetCode 0047 Permutations II <`Medium`> <`backtracking`>
- LeetCode 0077 Combinations <`Medium`> <`backtracking`>
- LeetCode 0039 Combination Sum <`Medium`> <`backtracking`>
- LeetCode 0040 Combination Sum II <`Medium`> <`backtracking`>
- LeetCode 0131 Palindrome Partitioning <`Medium`> <`dynamic programming`> <`backtracking`>
- LeetCode 0079 Word Search <`Medium`> <`backtracking`> <`matrix`>
- LeetCode 0127 Word Ladder <`Hard`> <`hash table`> <`bfs`>
- LeetCode 0126 Word Ladder II <`Hard`> <`hash table`> <`backtracking`> <`bfs`>
- LeetCode 0037 Sudoku Solver <`Hard`> <`backtracking`> <`matrix`>
- LeetCode 0051 N-Queens <`Hard`> <`backtracking`>
- LeetCode 0052 N-Queens II <`Hard`> <`backtracking`>
-
- 查找算法，二叉树
- LeetCode 0069 Sqrt(x) <`Easy`> <`binary search`>
- LeetCode 0033 Seach in Rotated Sorted Array <`Medium`> <`binary search`>
- LeetCode 0144 Binary Tree Preorder Traversal <`Easy`> <`stack`> <`dfs`> <`binary tree`>
- LeetCode 0094 Binary Tree Inorder Traversal <`Easy`> <`stack`> <`dfs`> <`binary tree`>
- LeetCode 0102 Binary Tree Level Order Traversal <`Medium`> <`bfs`> <`binary tree`>
- LeetCode 0145 Binary Tree Postorder Traversal <`Easy`> <`stack`> <`dfs`> <`binary tree`>
- LeetCode 0110 Balanced Binary Tree <`Easy`> <`dfs`> <`binary tree`>
- LeetCode 0226 Invert Binary Tree <`Easy`> <`dfs`> <`bfs`> <`binary tree`>
- LeetCode 0236 Lowest Common Ancestor of a Binary Tree <`Medium`> <`dfs`> <`binary tree`>
-
- BFS 与 DFS，并查集与线段树
- LeetCode 0200 Number of Islands <`Medium`> <`dfs`> <`bfs`> <`union find`> <`matrix`>
- LeetCode 0128 Longest Consecutive Sequence <`Medium`> <`hash table`> <`union find`>
- LeetCode 0207 Course Schedule <`Medium`> <`dfs`> <`bfs`> <`graph`> <`topological sort`>
- LeetCode 0547 Number of Provinces <`Medium`> <`dfs`> <`bfs`> <`union find`> <`graph`>
-
- 位运算，数学问题
- LeetCode 0190 Reverse Bits <`Easy`> <`divide and conquer`> <`bit manipulation`>
- LeetCode 0338 Counting Bits <`Medium`> <`dynamic programming`> <`bit manipulation`>
- LeetCode 0415 Add Strings <`Easy`> <`math`>
-
- 原理问题
- Bloom filter 原理
- cuckoo filter 原理
- red-black tree 原理
- B tree, B+ tree 原理

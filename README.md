## What is this?

In this repo I keep pieces of code I write as I try to `wrap` my mind
around Rust and its ownership system.
As a first step, I want to solve an IOI2016 problem named `molecules` in Rust,
since the solution I have in mind involves a generic algorithm
as a subroutine and I will try to implement this subroutine as generally as
possible using Rust generics and curious to see how this interacts with the
ownership system. The problem goes as follows:

> We are given <img alt="\inline n" src="https://latex.codecogs.com/png.latex?%5Cinline%20n" align="center"/> nonnegative integers <img alt="\inline w_1" src="https://latex.codecogs.com/png.latex?%5Cinline%20w_1" align="center"/>, <img alt="\inline w_2, \ldots, w_n\," src="https://latex.codecogs.com/png.latex?%5Cinline%20w_2%2C%20%5Cldots%2C%20w_n%5C%2C" align="center"/> and an 
> inclusive range <img alt="\inline [l, u]" src="https://latex.codecogs.com/png.latex?%5Cinline%20%5Bl%2C%20u%5D" align="center"/>, with the promise that <img alt="\inline u - l \ge
  \mathrm{max}_i w_i - \mathrm{min}_i w_i" src="https://latex.codecogs.com/png.latex?%5Cinline%20u%20-%20l%20%5Cge%0A%20%20%5Cmathrm%7Bmax%7D_i%20w_i%20-%20%5Cmathrm%7Bmin%7D_i%20w_i" align="center"/>.
> Find a subset of <img alt="\inline w_i" src="https://latex.codecogs.com/png.latex?%5Cinline%20w_i" align="center"/>'s which sum to a number in the range <img alt="\inline [l, u]" src="https://latex.codecogs.com/png.latex?%5Cinline%20%5Bl%2C%20u%5D" align="center"/> or if no
> such set exists, output the empty set.

Assume for a moment that <img alt="\inline w_i" src="https://latex.codecogs.com/png.latex?%5Cinline%20w_i" align="center"/>'s are sorted in increasing order:

<p align=center><img alt="\displaystyle{
0 \le w_1 \le w_2 \le \ldots \le w_n.
}" src="https://latex.codecogs.com/png.latex?%5Cdisplaystyle%7B%0A0%20%5Cle%20w_1%20%5Cle%20w_2%20%5Cle%20%5Cldots%20%5Cle%20w_n.%0A%7D"/></p>


Let <img alt="\inline t" src="https://latex.codecogs.com/png.latex?%5Cinline%20t" align="center"/> be the largest index such that <img alt="\inline w_1 + w_2 +\cdots + w_t \lt l" src="https://latex.codecogs.com/png.latex?%5Cinline%20w_1%20%2B%20w_2%20%2B%5Ccdots%20%2B%20w_t%20%3C%20l" align="center"/>. If 
the sum of the first <img alt="\inline t+1" src="https://latex.codecogs.com/png.latex?%5Cinline%20t%2B1" align="center"/> elements is at most <img alt="\inline u" src="https://latex.codecogs.com/png.latex?%5Cinline%20u" align="center"/>, then we have found a
solution: a correct answer to the problem is <img alt="\inline [1..t+1]" src="https://latex.codecogs.com/png.latex?%5Cinline%20%5B1..t%2B1%5D" align="center"/>.

If the other hand <img alt="\inline w_1+ \ldots + w_{t+1} \gt u" src="https://latex.codecogs.com/png.latex?%5Cinline%20w_1%2B%20%5Cldots%20%2B%20w_%7Bt%2B1%7D%20%3E%20u" align="center"/>, then we know that the smallest
<img alt="\inline t+1" src="https://latex.codecogs.com/png.latex?%5Cinline%20t%2B1" align="center"/>-element subset is too large and the smallest <img alt="\inline t" src="https://latex.codecogs.com/png.latex?%5Cinline%20t" align="center"/>-element subset is too
small. Therefore if there is a correct subset, it must have exactly <img alt="\inline t" src="https://latex.codecogs.com/png.latex?%5Cinline%20t" align="center"/>
elements.

Now consider the largest <img alt="\inline t" src="https://latex.codecogs.com/png.latex?%5Cinline%20t" align="center"/>-element subset: <img alt="\inline w_{n-t+1}, \ldots, w_{n}" src="https://latex.codecogs.com/png.latex?%5Cinline%20w_%7Bn-t%2B1%7D%2C%20%5Cldots%2C%20w_%7Bn%7D" align="center"/>.
If this subset sums to a value smaller than <img alt="\inline l" src="https://latex.codecogs.com/png.latex?%5Cinline%20l" align="center"/>, then there are no solutions
by our previous observation. If this subset sums to a value no smaller than 
<img alt="\inline l" src="https://latex.codecogs.com/png.latex?%5Cinline%20l" align="center"/> on the other hand, a solution is guaranteed to exists and here is why:
Let 
<p align=center><img alt="\displaystyle{
S = \{1, \ldots, t\} \text{, and } T=\{n-t+1, \ldots, n\}.
}" src="https://latex.codecogs.com/png.latex?%5Cdisplaystyle%7B%0AS%20%3D%20%5C%7B1%2C%20%5Cldots%2C%20t%5C%7D%20%5Ctext%7B%2C%20and%20%7D%20T%3D%5C%7Bn-t%2B1%2C%20%5Cldots%2C%20n%5C%7D.%0A%7D"/></p>

As long as <img alt="\inline \sum_{i\in S} w_i" src="https://latex.codecogs.com/png.latex?%5Cinline%20%5Csum_%7Bi%5Cin%20S%7D%20w_i" align="center"/> is smaller than <img alt="\inline l" src="https://latex.codecogs.com/png.latex?%5Cinline%20l" align="center"/>, exchange the smallest
index in <img alt="\inline S" src="https://latex.codecogs.com/png.latex?%5Cinline%20S" align="center"/> with the largest index in <img alt="\inline T" src="https://latex.codecogs.com/png.latex?%5Cinline%20T" align="center"/>. This exchange is guaranteed to 
increase the sum <img alt="\inline \sum_{i\in S} w_i" src="https://latex.codecogs.com/png.latex?%5Cinline%20%5Csum_%7Bi%5Cin%20S%7D%20w_i" align="center"/>, but no single exchange can push the value 
from smaller than <img alt="\inline l" src="https://latex.codecogs.com/png.latex?%5Cinline%20l" align="center"/> to larger than <img alt="\inline u" src="https://latex.codecogs.com/png.latex?%5Cinline%20u" align="center"/> due to the promise <img alt="\inline u - l \ge
\mathrm{max}_i w_i - \mathrm{min}_i w_i" src="https://latex.codecogs.com/png.latex?%5Cinline%20u%20-%20l%20%5Cge%0A%5Cmathrm%7Bmax%7D_i%20w_i%20-%20%5Cmathrm%7Bmin%7D_i%20w_i" align="center"/>. So we have a value that is initially
smaller than <img alt="\inline l" src="https://latex.codecogs.com/png.latex?%5Cinline%20l" align="center"/>, and eventually becomes larger than <img alt="\inline u" src="https://latex.codecogs.com/png.latex?%5Cinline%20u" align="center"/> and in no single step
jumps from smaller than <img alt="\inline l" src="https://latex.codecogs.com/png.latex?%5Cinline%20l" align="center"/> to larger than <img alt="\inline u" src="https://latex.codecogs.com/png.latex?%5Cinline%20u" align="center"/>. This means that in at least one
step the value must be in <img alt="\inline [l,u]" src="https://latex.codecogs.com/png.latex?%5Cinline%20%5Bl%2Cu%5D" align="center"/>.

This leads to a greedy algorithm in a direct way: Given the <img alt="\inline w_i" src="https://latex.codecogs.com/png.latex?%5Cinline%20w_i" align="center"/> values, sort
them, set <img alt="\inline S = \{1,\ldots, t\}" src="https://latex.codecogs.com/png.latex?%5Cinline%20S%20%3D%20%5C%7B1%2C%5Cldots%2C%20t%5C%7D" align="center"/>, <img alt="\inline T=\{n-t+1,\ldots, n\}" src="https://latex.codecogs.com/png.latex?%5Cinline%20T%3D%5C%7Bn-t%2B1%2C%5Cldots%2C%20n%5C%7D" align="center"/> and update <img alt="\inline S,T" src="https://latex.codecogs.com/png.latex?%5Cinline%20S%2CT" align="center"/>
as described above in a single pass over the <img alt="\inline w_i" src="https://latex.codecogs.com/png.latex?%5Cinline%20w_i" align="center"/> values and output <img alt="\inline S" src="https://latex.codecogs.com/png.latex?%5Cinline%20S" align="center"/>.
The runtime of this algorithm is <img alt="\inline O(n\log n)" src="https://latex.codecogs.com/png.latex?%5Cinline%20O%28n%5Clog%20n%29" align="center"/>, dominated by the sorting
subroutine. This is also the best runtime given by the IOI solutions.

Here we will solve this problem in <img alt="\inline O(n)" src="https://latex.codecogs.com/png.latex?%5Cinline%20O%28n%29" align="center"/> time--without sorting--by implementing
the above idea more carefully. First we need a generic subroutine which I call
the "sumth_element", which is a bit like the `std::nth_element` in C++
(a.k.a. quickselect) but is concerned with tail sums.

### sumth_element

> Suppose we are give an unsorted array <img alt="\inline a_1, a_2,\ldots, a_n" src="https://latex.codecogs.com/png.latex?%5Cinline%20a_1%2C%20a_2%2C%5Cldots%2C%20a_n" align="center"/> and a number <img alt="\inline S" src="https://latex.codecogs.com/png.latex?%5Cinline%20S" align="center"/>.
> Find the largest <img alt="\inline t" src="https://latex.codecogs.com/png.latex?%5Cinline%20t" align="center"/> such that the sum of the smallest <img alt="\inline t" src="https://latex.codecogs.com/png.latex?%5Cinline%20t" align="center"/> elements of <img alt="\inline a" src="https://latex.codecogs.com/png.latex?%5Cinline%20a" align="center"/>
> is at most <img alt="\inline S" src="https://latex.codecogs.com/png.latex?%5Cinline%20S" align="center"/>.

I call this the "sumth_element" for lack of a better name (and imagination) and 
is meant to be in analogy to `std::nth_element`
(if you know the proper name for this do let me know).

If the array was sorted and we had oracle access to prefix sums of <img alt="\inline a" src="https://latex.codecogs.com/png.latex?%5Cinline%20a" align="center"/>,
we could have found this index in <img alt="\inline O(\log n)" src="https://latex.codecogs.com/png.latex?%5Cinline%20O%28%5Clog%20n%29" align="center"/> time
by a binary search:
```rust
let mut l = 0;
let mut r = a.len();

while l < r {
    let m = l + (r - l) / 2;
    let tail_sum = a[l..=m].iter().sum();
    if tail_sum <= sum {
        l = m + 1;
        sum -= tail_sum;
    } else {
        r = m;
    }
}
l
```
Any binary search working on sorted arrays can be translated mechanically to 
work on arbitrary arrays in <img alt="\inline O(n)" src="https://latex.codecogs.com/png.latex?%5Cinline%20O%28n%29" align="center"/> time by using the quickselect algorithm.
It looks like rust standard library does not have a quickselect
implementation and I'm not even going to think about attempting to implement
it (I don't think I have ever implemented it, in any language).
It turns out there are several crates implementing this. I will go with the
`order_stat` crate:
```toml
# Cargo.toml
[dependencies]
order-stat = "0.1"
```

Here is the version that works on arbitrary arrays:
```rust
let mut l = 0;
let mut r = a.len();

while l < r {
    let m = l + (r - l) / 2;
    order_stat::kth(&mut a[l..r], m - l); // added line
    let tail_sum = a[l..=m].iter().sum();
    if tail_sum <= sum {
        l = m + 1;
        sum -= tail_sum;
    } else {
        r = m;
    }
}
l
```
That is, wherever the binary search is about to query an index <img alt="\inline m" src="https://latex.codecogs.com/png.latex?%5Cinline%20m" align="center"/>, we do a
minimal amount of sorting so as to ensure the property required by the binary
search:
<p align=center><img alt="\displaystyle{ a[i] \le a[m] \text{ for } i\ltm \text{ and } a[m] \ge a[i] \text{ for } i\gtm.}" src="https://latex.codecogs.com/png.latex?%5Cdisplaystyle%7B%20a%5Bi%5D%20%5Cle%20a%5Bm%5D%20%5Ctext%7B%20for%20%7D%20i%3Cm%20%5Ctext%7B%20and%20%7D%20a%5Bm%5D%20%5Cge%20a%5Bi%5D%20%5Ctext%7B%20for%20%7D%20i%3Em.%7D"/></p>

Each invocation of the `order_stat::kth` takes time linear in the slice
we pass to it. Since the slice size is halved in each iteration, the total
runtime comes to <img alt="\inline O(n)" src="https://latex.codecogs.com/png.latex?%5Cinline%20O%28n%29" align="center"/>.

Here if the full code with the generics [sumth_elements.rs](sumth_element.rs).
If you have cloned this repo, you can run the unit tests by
```sh
cargo test sumth_element --release
```

### Linear time solution

Let us go back to `molecues`. Now with the `sumth_element` in hand, here is 
the linear time solution. First we map the <img alt="\inline w_i" src="https://latex.codecogs.com/png.latex?%5Cinline%20w_i" align="center"/> array into the array of
tuples <img alt="\inline (i, w_i)" src="https://latex.codecogs.com/png.latex?%5Cinline%20%28i%2C%20w_i%29" align="center"/> since at the end we need to output the indices.
We use `u32` for both the "weights" <img alt="\inline w_i" src="https://latex.codecogs.com/png.latex?%5Cinline%20w_i" align="center"/> and the indices so as to make layout
as compact as possible for cache efficiency. However, we need `u64`s 
whenever we need to sum weights since for large instances it will overflow.

```rust
struct WeightIndex {
    pub w: u32,
    pub i: u32,
}

pub fn find_subset(l: u32, u: u32, w: &[u32]) -> Vec<u32> {
    let l = l as u64;
    let u = u as u64;
    let mut wi: Vec<_> = w
        .iter()
        .enumerate()
        .map(|(i, &w)| WeightIndex { w, i: i as u32 })
        .collect();

    let (t, slack) = sumth_element(&mut wi, l - 1);
    if t == wi.len() {
        return vec![];
    }
    order_stat::kth(&mut wi[t..], 0);

    let sum = l - 1 - slack;
    if sum + wi[t].w as u64 <= u {
        return wi[..=t].iter().map(|wi| wi.i).collect();
    }

    if t + t + 1 < w.len() && t > 0 {
        order_stat::kth(&mut wi[t + 1..], w.len() - (t + t + 1));
    }

    let mut sum = sum;
    let mut j = 0;
    let mut k = wi.len() - 1;
    while sum < l && j < t {
        sum += (wi[k].w - wi[j].w) as u64;
        wi.swap(j, k);
        j += 1;
        k -= 1;
    }
    if sum >= l {
        wi[..t].iter().map(|wi| wi.i).collect()
    } else {
        vec![]
    }
}
```
Rust is really expressive! I can't imagine transforming the weights array to
WeightIndex array in C++ standard library.
Here is the full solution: [molecules.rs](molecules.rs)

### Official testsuite

Let us finally try our solution on the official test suite.
I am having a real tough time finding a good way to parse whitespace separated
integers from text file in a streaming fashion with the Rust standard library.
The test files can be quite large, especially for problems with small 
complexity (such as <img alt="\inline O(n\log n)" src="https://latex.codecogs.com/png.latex?%5Cinline%20O%28n%5Clog%20n%29" align="center"/>), so a  streaming reader is really needed.
The best thing I could come up with [bench.rs](bench.rs)#`test_from_file` is not
only super ugly, but also still involves an extra string copy per integer.
I implemented the test runner as a cargo bench to get some timing info.
You can run the bechmark as so:
```sh
cargo bench

     Running target/release/deps/bench-a9da5db63e60670a

running 4 tests
test huge_tests   ... bench:  14,444,431 ns/iter (+/- 912,981)
test large_tests  ... bench: 172,175,577 ns/iter (+/- 950,302)
test medium_tests ... bench: 156,179,196 ns/iter (+/- 1,020,305)
test small_tests  ... bench:  34,498,891 ns/iter (+/- 409,675)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out

```
The numbers are from my Intel(R) Celeron(R) 2955U @ 1.40GHz beast of a 
workstation (a chromebox-turned-xubuntu monstrosity).
# Fast nearest neighbors testing

These measurement are from calling `poinctl explain -i cube100k.csv -r reduced-cube100k.csv foobar.csv`, here _cube100k_ are point in 3D along a cube's faces. _reduced-cube100k.csv_ is a PCA based dimension reduction of the dataset to 2D. both contain 100_000 points. We output the explanation to foobar.csv. Realtime is measured using the coreutil `time`.

| algorithm | revision | running time querying all | real time 100k |
| -- | -- | -- | -- |
| brute force | initial try (7d6a117) | n^2 | 28 sec |
| r tree | first try (d8a1fc6) | n log n | 36 sec |
| r tree | initial optimization (30061b6) | n log n | 16 sec |
| r tree | bounded neighborhood (1697a44) | n log n | 4.5 sec |
| hnsw<sup>1</sup> | branch hnsw (bff1a1f) | log n | 31 sec |


1: Efficient and robust approximate nearest neighbor search using Hierarchical Navigable Small World graphs - https://arxiv.org/abs/1603.09320

Just watching the overall running time it seems odd that the _smart_ algorithms perform this poorly. To find out why I enabled debugging symbols in release build and `perf` to generate flame graphs. With this I quickly found that the _r*-tree_ implementation did far more work than needed, instead of limiting the search to the radius it ordered _all_ points on distance for every radius based nearest neighbor query. After some limiting the radius searched I was able to optimize the query time significantly. At this time the main slowdown now comes from calculating the distances for the local contributions. I believe implementing a distance LUT would greatly aid this. Another advantage _r*-trees_ has is that the initial search structure can be generated rather quickly, it does this by inserting everything in bulk.

The other promising option I implemented used Hierarchical Navigable Small World graphs, this promised extremely fast nearest neighbor querying. After implementing and benchmarking I found that 80% of the programs time is spend building the search structure though. This is _extremely_ slow but the structure does deliver on the promise of very quick searching. The search is limited though, the distance need to be integer values and you can not directly query on distance. This structure might be worth revisiting later but I am first going to explore the _r*-trees_.

## Limiting the neighborhood size

After the latests journey into the r-tree method I found that most time was spend on calculating distance contributions in the neighborhood. This was because every neighborhood was _very_ large and I've since decided to introduce a bound on these neighbors. I will first retrieve every neighbor in a certain distance of the point, then if there are more points than allowed in a neighborhood I will sample without replacement from the found neighbors. This method yielded a 3x speedup when limiting to 250 neighbors for the 100k points benchmark. Visually there is no difference.
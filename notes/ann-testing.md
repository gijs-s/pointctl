# Fast nearest neighbors testing

These measurement are from calling `poinctl explain -i cube100k.csv -r reduced-cube100k.csv foobar.csv`, here _cube100k_ are point in 3D along a cube's faces. _reduced-cube100k.csv_ is a PCA based dimension reduction of the dataset to 2D. both contain 100_000 points. We output the explanation to foobar.csv. Realtime is measured using the coreutil `time`.

| algorithm | Running time single query | running time querying all | real time 100k |
| -- | -- | -- | -- |
| brute force | n | n^2 | 28 sec |
| r tree | log n | n log n | 36 sec |
| hnsw<sup>1</sup> | log n | n log n | 31 sec |

1: Efficient and robust approximate nearest neighbor search using Hierarchical Navigable Small World graphs - https://arxiv.org/abs/1603.09320


It is odd to see that the _smart_ algorithms perform this poorly, to find out why I am turning to profiling tools.

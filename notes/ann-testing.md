# Fast nearest neighbors testing

| algorithm | Running time single query | running time querying all | real time 100k |
| -- | -- | -- | -- |
| brute force | n | n^2 | 28 sec |
| r tree | log n | n log n | 36 sec |
| hnsw<sup>1</sup> | log n | n log n | 31 sec |

1: Efficient and robust approximate nearest neighbor search using Hierarchical Navigable Small World graphs - https://arxiv.org/abs/1603.09320

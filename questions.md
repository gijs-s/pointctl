# File for collecting questions.

1. The da silva explanation relies on neighborhoods bound by `p`. It seems like TSNE aims to perserve the
relative distance but not the absolute one, this means that `p` should differ wildly. Should I scale the TSNE
output to match in a unit cube and then continue the calculation?
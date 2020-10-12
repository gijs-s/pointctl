# An overview of the implemented pipeline

## Initial planning as discussed with Telea

We discussed the outline of the processing pipeline. We looked at the MVP way of building these modules.

Lets first look at the simple offline phase, this will only consist off ways to preprocess the data. Part of this is to prefrom the reduction (using tsne) and make both nD and 3D data sets available to the tool.

The next _box_ so to say would be the explanation. This takes both datasets and a few parameters and will output a set of annotated points. These will consist of the original point and a few additional attributes.

Finally we have the visualization, this will take as input the annotated points and a viewport and create an image.

## First revision goal.

The key goal of the first revision is to create a tool that can calculate the single attribute approach from da silva. This means we annotate each point with a category and a confidence [0.0..1.0].

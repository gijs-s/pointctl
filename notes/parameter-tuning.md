# Guide on tuning the parameters of the viewer

The tool is very much directed at people that have knowledge of the underlying mechanisms, it can be seen as a very Academic implementation that allows for tweaking many parameters. This small guide will explain some of the options and intend.

## Continuous representation (Blob size)

For the continuos representation we default to the average first nearest neighbor distance as size. This _should_ approximate the blob size required for all points to just touch each other. This however might always be true set correctly, we allow the user to change the actual blob size as well! For this the goal is to get most points to just touch each other, below we have an example. The first two images show a blob size just a little to low, the bottom left is the correct size and the bottom right is far to large.

When collecting data tune the blob size so we in the areas with numerous white points you cannot see the white background anymore.

<div>
    <img
        src="https://git.science.uu.nl/g.j.vansteenpaal/pointctl/-/raw/master/notes/assets/blob-size-0.png"
        alt="blob far to small"
        style="margin-right: 2px; width: 33%"
    />
    <img
        src="https://git.science.uu.nl/g.j.vansteenpaal/pointctl/-/raw/master/notes/assets/blob-size-1.png"
        alt="blob to small"
        style="margin-right: 2px; width: 33%"
    />
    <img
        src="https://git.science.uu.nl/g.j.vansteenpaal/pointctl/-/raw/master/notes/assets/blob-size-2.png"
        alt="blob just right"
        style="margin-right: 2px; width: 33%"
    />
    <img
        src="https://git.science.uu.nl/g.j.vansteenpaal/pointctl/-/raw/master/notes/assets/blob-size-3.png"
        alt="blob to large"
        style="margin-right: 2px; width: 33%"
    />
</div>

Generally the default value will look a little bit to small, upping it a bit will aid the perception when looking at explanations.

## Gamma / confidence normalization

The second point of configuration is related to the brightness of points, very important as this is used to encode the confidence of our explanations. We make use of an isoluminant categorical color encoding, very nice in theory but it sadly also limits the color space we can use. We allow the user to override the gamma to attempt and unify the images with generally darker or lighter images.

Lets look at a use case with 4 examples. This is an explanation that generally has high confidence but has some outliers with very low confidence. The 1st image shows a washed out picture with far to high a gamma value, the second is much better and arguably the bottom left is best.

The last step here is to handle the low confidence outliers, we can do this by tweaking the lower normalization bounds resulting in the bottom right image.

<div>
    <img
        src="https://git.science.uu.nl/g.j.vansteenpaal/pointctl/-/raw/master/notes/assets/gamma-0.png"
        alt="gamma to high"
        style="margin-right: 2px; width: 33%"
    />
    <img
        src="https://git.science.uu.nl/g.j.vansteenpaal/pointctl/-/raw/master/notes/assets/gamma-1.png"
        alt="gamma oke ish"
        style="margin-right: 2px; width: 33%"
    />
    <img
        src="https://git.science.uu.nl/g.j.vansteenpaal/pointctl/-/raw/master/notes/assets/gamma-2.png"
        alt="gamma good"
        style="margin-right: 2px; width: 33%"
    />
    <img
        src="https://git.science.uu.nl/g.j.vansteenpaal/pointctl/-/raw/master/notes/assets/gamma-3.png"
        alt="gamma + normalization"
        style="margin-right: 2px; width: 33%"
    />
</div>

Generally the default gamma should be great, tweaking the confidence bounds is a must. We have no mechanism in place to remove these outliers and to bring the confidence gradation into focus will require some tuning.

## Choosing R for the explanation

The R is short for Radius, it the normalized form of the projection width. When selecting this value it is crucial to keep the square–cube law in mind, R: 0.25 will enclose approximately 1/4th of the 2D region but the same radius will only enclose 1/8th of the 3D regions!

When directly comparing 2D with 3D you ought to select a slightly higher radius to include a similar amount of points.

## Shading

When we are running the 3D explanation another aspect comes into play, namely pseudo shading. This is implemented in the form on another explanation mechanism,it will attempt to find an approximate normal based on the neighborhood. It is best to tune this with only grey points, this is again best illustrated with some images.

The top left shows now shading at all, the top right shows shading with a tiny neighborhood resulting in noisy normals. Bottom left we see a proper shading but and te bottom left we tuned down the shading intensity. This last feature is nice when we are looking an a explanation as we then try to convey both confidence and orientation using the brightness.

<div>
    <img
        src="https://git.science.uu.nl/g.j.vansteenpaal/pointctl/-/raw/master/notes/assets/shading-0.png"
        alt="no shading"
        style="margin-right: 2px; width: 33%"
    />
    <img
        src="https://git.science.uu.nl/g.j.vansteenpaal/pointctl/-/raw/master/notes/assets/shading-1.png"
        alt="shading noisy"
        style="margin-right: 2px; width: 33%"
    />
    <img
        src="https://git.science.uu.nl/g.j.vansteenpaal/pointctl/-/raw/master/notes/assets/shading-2.png"
        alt="shading good"
        style="margin-right: 2px; width: 33%"
    />
    <img
        src="https://git.science.uu.nl/g.j.vansteenpaal/pointctl/-/raw/master/notes/assets/shading-3.png"
        alt="shading tuned down"
        style="margin-right: 2px; width: 33%"
    />
</div>

Generally a radius of about 0.2 yields good results, when looking at explanations a shading intensity of about 0.9 is adviced.

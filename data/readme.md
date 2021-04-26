# Dataset

A large folder containing all the datasets, here we have all the datasets provided by Matheus with numerous projections and others without any. This file gives a quick overview of all the datasets.

Note that only some of the datasets are present, the rest is available via the 'get_datasets.py' python script!

These files are all hosted via git LFS, to get them you can run the follow these commands:

```
git lfs install
git lfs pull
```

# List of datasets:

## AirQuality

The dataset contains 9358 instances of hourly averaged responses from an array
of 5 metal oxide chemical sensors embedded in an Air Quality Chemical
Multisensor Device. The device was located on the field in a significantly
polluted area, at road level,within an Italian city. Data were recorded from
March 2004 to February 2005 (one year)representing the longest freely available
recordings of on field deployed air quality chemical sensor devices responses.
Ground Truth hourly averaged concentrations for CO, Non Metanic Hydrocarbons,
Benzene, Total Nitrogen Oxides (NOx) and Nitrogen Dioxide (NO2) and were
provided by a co-located reference certified analyzer. Evidences of
cross-sensitivities as well as both concept and sensor drifts are present as
described in De Vito et al., Sens. And Act. B, Vol. 129,2,2008 (citation
required) eventually affecting sensors concentration estimation capabilities

URL: https://archive.ics.uci.edu/ml/datasets/Air+Quality

| Characteristics |  |
| -- | -- |
| Data Set Characteristics | Multivariate, Time-Series |
| Number of Instances | 9358 |
| Area | Computer |
| Attribute Characteristics | Real |
| Number of Attributes | 15 |
| Associated Tasks | Regression |
| Missing Values? | Yes |

### Attribute Information:

```
0 Date (DD/MM/YYYY)
1 Time (HH.MM.SS)
2 True hourly averaged concentration CO in mg/m^3 (reference analyzer)
3 PT08.S1 (tin oxide) hourly averaged sensor response (nominally CO targeted)
4 True hourly averaged overall Non Metanic HydroCarbons concentration in microg/m^3 (reference analyzer)
5 True hourly averaged Benzene concentration in microg/m^3 (reference analyzer)
6 PT08.S2 (titania) hourly averaged sensor response (nominally NMHC targeted)
7 True hourly averaged NOx concentration in ppb (reference analyzer)
8 PT08.S3 (tungsten oxide) hourly averaged sensor response (nominally NOx targeted)
9 True hourly averaged NO2 concentration in microg/m^3 (reference analyzer)
10 PT08.S4 (tungsten oxide) hourly averaged sensor response (nominally NO2 targeted)
11 PT08.S5 (indium oxide) hourly averaged sensor response (nominally O3 targeted)
12 Temperature in Â°C
13 Relative Humidity (%)
14 AH Absolute Humidity
```



## CityPollution

Not found

## Concrete

URL: https://archive.ics.uci.edu/ml/datasets/concrete+compressive+strength

| Characteristics |  |
| -- | -- |
| Data Set Characteristics | Multivariate |
| Number of Instances | 1030 |
| Area | Physical |
| Attribute Characteristics | Real |
| Number of Attributes | 9 |
| Associated Tasks | Regression |
| Missing Values? | N/A |

### Attribute Information:

```
Name -- Data Type -- Measurement -- Description

Cement (component 1) -- quantitative -- kg in a m3 mixture -- Input Variable
Blast Furnace Slag (component 2) -- quantitative -- kg in a m3 mixture -- Input Variable
Fly Ash (component 3) -- quantitative -- kg in a m3 mixture -- Input Variable
Water (component 4) -- quantitative -- kg in a m3 mixture -- Input Variable
Superplasticizer (component 5) -- quantitative -- kg in a m3 mixture -- Input Variable
Coarse Aggregate (component 6) -- quantitative -- kg in a m3 mixture -- Input Variable
Fine Aggregate (component 7) -- quantitative -- kg in a m3 mixture -- Input Variable
Age -- quantitative -- Day (1~365) -- Input Variable
Concrete compressive strength -- quantitative -- MPa -- Output Variable
```


## default of credit card clients / defaultcc

This research aimed at the case of customers' default payments in Taiwan and
compares the predictive accuracy of probability of default among six data mining
methods. From the perspective of risk management, the result of predictive
accuracy of the estimated probability of default will be more valuable than the
binary result of classification - credible or not credible clients. Because the
real probability of default is unknown, this study presented the novel
â€œSorting Smoothing Method's to estimate the real probability of default. With
the real probability of default as the response variable (Y), and the predictive
probability of default as the independent variable (X), the simple linear
regression result (Y = A + BX) shows that the forecasting model produced by
artificial neural network has the highest coefficient of determination; its
regression intercept (A) is close to zero, and regression coefficient (B) to
one. Therefore, among the six data mining techniques, artificial neural network
is the only one that can accurately estimate the real probability of default.

ULR: https://archive.ics.uci.edu/ml/datasets/default+of+credit+card+clients

| Characteristics |  |
| -- | -- |
| Data Set Characteristics | Multivariate |
| Number of Instances | 30000 |
| Area | Business |
| Attribute Characteristics | Integer, Real |
| Number of Attributes | 24 |
| Associated Tasks | Classification |
| Missing Values? | N/A |

### Attribute Information:

```
This research employed a binary variable, default payment (Yes = 1, No = 0), as the response variable. This study reviewed the literature and used the following 23 variables as explanatory variables:
X1: Amount of the given credit (NT dollar): it includes both the individual consumer credit and his/her family (supplementary) credit.
X2: Gender (1 = male; 2 = female).
X3: Education (1 = graduate school; 2 = university; 3 = high school; 4 = others).
X4: Marital status (1 = married; 2 = single; 3 = others).
X5: Age (year).
X6 - X11: History of past payment. We tracked the past monthly payment records (from April to September, 2005) as follows: X6 = the repayment status in September, 2005; X7 = the repayment status in August, 2005; . . .;X11 = the repayment status in April, 2005. The measurement scale for the repayment status is: -1 = pay duly; 1 = payment delay for one month; 2 = payment delay for two months; . . .; 8 = payment delay for eight months; 9 = payment delay for nine months and above.
X12-X17: Amount of bill statement (NT dollar). X12 = amount of bill statement in September, 2005; X13 = amount of bill statement in August, 2005; . . .; X17 = amount of bill statement in April, 2005.
X18-X23: Amount of previous payment (NT dollar). X18 = amount paid in September, 2005; X19 = amount paid in August, 2005; . . .;X23 = amount paid in April, 2005.
```

## nd-clusters

Synthetic generated dataset, distinct clusters in projected space where the first has noise across 1 dimension in nD, the next across 2 dimensions etc.

## nd-fade

Synthetic generated dataset, 7 adjacent squares in projected space where the first has noise across 1 dimension in nD, the next across 2 dimensions etc.

## Reuters

A bag of words type dataset to for reuters news publications with as output one of 46 topics. The numbers we see actually encode the n-th most common word.

URL: https://keras.io/api/datasets/reuters/

| Characteristics |  |
| -- | -- |
| Data Set Characteristics | Multivariate |
| Number of Instances | 11228 |
| Area | N/A |
| Attribute Characteristics | Integer |
| Number of Attributes | 1000 |
| Associated Tasks | Classification |
| Missing Values? | N/A |

## Software

Dataset that

| Characteristics |  |
| -- | -- |
| Data Set Characteristics| Multivariate |
| Number of Instances| 6773 |
| Area | Computer |
| Attribute Characteristics| Real, Integer |
| Number of Attributes| 12 |
| Associated Tasks| Classification, Clustering |
| Missing Values?| N/A |

### Attribute information

```
• Afferent Connections per Class(ACC),
• Coupling between Objects(CBO),
• Coupling Factor(COF),
• Depth of Inheritance Tree(DIT),
• Lack of Cohesion on Methods/Functions(LCOM4),
• Lines of Code(LOC),
• Lines per Method/Function(AMZ_Size),
• Number of Attributes/Variables(NOV),
• Number of Children per Class(NOC),
• Number of Methods/Functions(NOF),
• Number of Classes/Module(NM),
• Number of Public Attributes/Variables(NPV),
• Number of Public Methods/Functions(NPF),
• Response for Class(RFC).
```

## Wine / winequality

A dataset that consists of 6498 wines divided into 4898 red and 1600 white ones.

URL for red wines: https://archive.ics.uci.edu/ml/datasets/wine+quality

| Characteristics |  |
| -- | -- |
| Data Set Characteristics| Multivariate |
| Number of Instances| 6498 |
| Area | Business |
| Attribute Characteristics| Real |
| Number of Attributes| 12 |
| Associated Tasks| Classification, Regression |
| Missing Values?| N/A |

### Attribute information:

```
Input variables (based on physicochemical tests):
1 - fixed acidity
2 - volatile acidity
3 - citric acid
4 - residual sugar
5 - chlorides
6 - free sulfur dioxide
7 - total sulfur dioxide
8 - density
9 - pH
10 - sulphates
11 - alcohol
Output variable (based on sensory data):
12 - quality (score between 0 and 10)
```

## WisconsinBreastCancer

URL: https://archive.ics.uci.edu/ml/datasets/Breast+Cancer+Wisconsin+(Diagnostic)

| Characteristics |  |
| -- | -- |
| Data Set Characteristics | Multivariate |
| Number of Instances | 569 |
| Area | Life |
| Attribute Characteristics | Real |
| Number of Attributes | 32 |
| Associated Tasks | Classification |
| Missing Values? | No |

### Attribute information:

```
1) ID number
2) Diagnosis (M = malignant, B = benign)
3-32)

Ten real-valued features are computed for each cell nucleus:

a) radius (mean of distances from center to points on the perimeter)
b) texture (standard deviation of gray-scale values)
c) perimeter
d) area
e) smoothness (local variation in radius lengths)
f) compactness (perimeter^2 / area - 1.0)
g) concavity (severity of concave portions of the contour)
h) concave points (number of concave portions of the contour)
i) symmetry
j) fractal dimension ("coastline approximation" - 1)
```

# Reduction abbreviations used:

 - AE: Autoencoder
 - DM: Diffusion Maps
 - FA: Factor Analysis
 - F-ICA: Fast ICA
 - G-RP: Gaussian Random Projection
 - H-LLE: Hessian LLE
 - I-PCA: Incremental PCA
 - ISO: Isomap
 - K-PCA-P: Kernel PCA (rbf kernel)
 - K-PCA-R: Kernel PCA (polynomial kernel)
 - K-PCA-S: Kernel PCA (sigmoid kernel)
 - LE: Laplacian Eigenmaps (aka Spectral Embedding)
 - LLE: Locally Linear Embedding
 - L-LTSA: Linear LTSA
 - L-MDS: Landmark MDS
 - LPP: Locality Preserving Projections
 - LTSA: Local Tangent Space Alignment (variant of LLE)
 - MDS: Metric MDS
 - M-LLE: Modified LLE
 - N-MDS: Nonmetric MDS
 - NMF: Nonnegative Matrix Factorization
 - NPE: Neighborhood Preserving Embedding
 - PCA: Principal Component Analysis
 - S-PCA: Sparse PCA
 - SPE: Stochastic Proximity Embedding
 - S-RP: Sparse Random Projection
 - TSNE: t-distributed Stochastic Neighbor Embedding
 - T-SVD: Truncated SVD (aka Latent Semantic Analysis)
 - UMAP: Uniform Manifold Approximation and Projection

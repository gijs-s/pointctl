# Dataset

A large folder containing all the datasets, here we have all the datasets provided by Matheus with numerous projections and others without any. This file gives a quick overview of all the datasets.

Note that only some of the datasets are present, the rest is available via the 'get_datasets.py' python script!

These files are all hosted via git LFS, to get them you can run the follow these commands:

```
git lfs install
git lfs pull
```

# List of datasets:

## Abalone

Predicting the age of abalone from physical measurements. The age of abalone is
determined by cutting the shell through the cone, staining it, and counting the
number of rings through a microscope -- a boring and time-consuming task. Other
measurements, which are easier to obtain, are used to predict the age. Further
information, such as weather patterns and location (hence food availability) may
be required to solve the problem.

URL: https://archive.ics.uci.edu/ml/datasets/Abalone

| Characteristics |  |
| -- | -- |
| Data Set Characteristics | Multivariate |
| Number of Instances | 4177 |
| Area | Life |
| Attribute Characteristics | Categorical, Integer, Real |
| Number of Attributes | 8 |
| Associated Tasks | Classification |
| Missing Values? | No |

### Attribute Information:

```
Given is the attribute name, attribute type, the measurement unit and a brief description. The number of rings is the value to predict: either as a continuous value or as a classification problem.


Name / Data Type / Measurement Unit / Description
-----------------------------
Sex / nominal / -- / M, F, and I (infant)
Length / continuous / mm / Longest shell measurement
Diameter / continuous / mm / perpendicular to length
Height / continuous / mm / with meat in shell
Whole weight / continuous / grams / whole abalone
Shucked weight / continuous / grams / weight of meat
Viscera weight / continuous / grams / gut weight (after bleeding)
Shell weight / continuous / grams / after being dried
Rings / integer / -- / +1.5 gives the age in years
```

### Absenteeism at work

Predicting the age of abalone from physical measurements. The age of abalone is
determined by cutting the shell through the cone, staining it, and counting the
number of rings through a microscope -- a boring and time-consuming task. Other
measurements, which are easier to obtain, are used to predict the age. Further
information, such as weather patterns and location (hence food availability) may
be required to solve the problem.

URL: https://archive.ics.uci.edu/ml/datasets/Absenteeism+at+work

| Characteristics |  |
| -- | -- |
| Data Set Characteristics | Multivariate, Time-Series |
| Number of Instances | 740 |
| Area | Business |
| Attribute Characteristics | Integer, Real |
| Number of Attributes | 21  |
| Associated Tasks | Classification, Clustering |
| Missing Values? | N/A |

### Attribute Information:

```
1. Individual identification (ID)
2. Reason for absence (ICD).
Absences attested by the International Code of Diseases (ICD) stratified into 21 categories (I to XXI) as follows:

I Certain infectious and parasitic diseases
II Neoplasms
III Diseases of the blood and blood-forming organs and certain disorders involving the immune mechanism
IV Endocrine, nutritional and metabolic diseases
V Mental and behavioural disorders
VI Diseases of the nervous system
VII Diseases of the eye and adnexa
VIII Diseases of the ear and mastoid process
IX Diseases of the circulatory system
X Diseases of the respiratory system
XI Diseases of the digestive system
XII Diseases of the skin and subcutaneous tissue
XIII Diseases of the musculoskeletal system and connective tissue
XIV Diseases of the genitourinary system
XV Pregnancy, childbirth and the puerperium
XVI Certain conditions originating in the perinatal period
XVII Congenital malformations, deformations and chromosomal abnormalities
XVIII Symptoms, signs and abnormal clinical and laboratory findings, not elsewhere classified
XIX Injury, poisoning and certain other consequences of external causes
XX External causes of morbidity and mortality
XXI Factors influencing health status and contact with health services.

And 7 categories without (CID) patient follow-up (22), medical consultation (23), blood donation (24), laboratory examination (25), unjustified absence (26), physiotherapy (27), dental consultation (28).
3. Month of absence
4. Day of the week (Monday (2), Tuesday (3), Wednesday (4), Thursday (5), Friday (6))
5. Seasons (summer (1), autumn (2), winter (3), spring (4))
6. Transportation expense
7. Distance from Residence to Work (kilometers)
8. Service time
9. Age
10. Work load Average/day
11. Hit target
12. Disciplinary failure (yes=1; no=0)
13. Education (high school (1), graduate (2), postgraduate (3), master and doctor (4))
14. Son (number of children)
15. Social drinker (yes=1; no=0)
16. Social smoker (yes=1; no=0)
17. Pet (number of pet)
18. Weight
19. Height
20. Body mass index
```

### AirQuality

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

## bank

The data is related with direct marketing campaigns of a Portuguese banking
institution. The marketing campaigns were based on phone calls. Often, more than
one contact to the same client was required, in order to access if the product
(bank term deposit) would be ('yes') or not ('no') subscribed. The
classification goal is to predict if the client will subscribe (yes/no) a term
deposit (variable y).

URL: https://archive.ics.uci.edu/ml/datasets/Bank+Marketing

| Characteristics |  |
| -- | -- |
| Data Set Characteristics | Multivariate |
| Number of Instances | 45211 |
| Area | Business |
| Attribute Characteristics | Real |
| Number of Attributes | 17 |
| Associated Tasks | Classification |
| Missing Values? | N/A |

### Attribute Information:

```
Input variables:
# bank client data:
1 - age (numeric)
2 - job : type of job (categorical: 'admin.','blue-collar','entrepreneur','housemaid','management','retired','self-employed','services','student','technician','unemployed','unknown')
3 - marital : marital status (categorical: 'divorced','married','single','unknown'; note: 'divorced' means divorced or widowed)
4 - education (categorical: 'basic.4y','basic.6y','basic.9y','high.school','illiterate','professional.course','university.degree','unknown')
5 - default: has credit in default? (categorical: 'no','yes','unknown')
6 - housing: has housing loan? (categorical: 'no','yes','unknown')
7 - loan: has personal loan? (categorical: 'no','yes','unknown')
# related with the last contact of the current campaign:
8 - contact: contact communication type (categorical: 'cellular','telephone')
9 - month: last contact month of year (categorical: 'jan', 'feb', 'mar', ..., 'nov', 'dec')
10 - day_of_week: last contact day of the week (categorical: 'mon','tue','wed','thu','fri')
11 - duration: last contact duration, in seconds (numeric). Important note: this attribute highly affects the output target (e.g., if duration=0 then y='no'). Yet, the duration is not known before a call is performed. Also, after the end of the call y is obviously known. Thus, this input should only be included for benchmark purposes and should be discarded if the intention is to have a realistic predictive model.
# other attributes:
12 - campaign: number of contacts performed during this campaign and for this client (numeric, includes last contact)
13 - pdays: number of days that passed by after the client was last contacted from a previous campaign (numeric; 999 means client was not previously contacted)
14 - previous: number of contacts performed before this campaign and for this client (numeric)
15 - poutcome: outcome of the previous marketing campaign (categorical: 'failure','nonexistent','success')
# social and economic context attributes
16 - emp.var.rate: employment variation rate - quarterly indicator (numeric)
17 - cons.price.idx: consumer price index - monthly indicator (numeric)
18 - cons.conf.idx: consumer confidence index - monthly indicator (numeric)
19 - euribor3m: euribor 3 month rate - daily indicator (numeric)
20 - nr.employed: number of employees - quarterly indicator (numeric)

Output variable (desired target):
21 - y - has the client subscribed a term deposit? (binary: 'yes','no')
```

## banknote

Data were extracted from images that were taken from genuine and forged
banknote-like specimens. For digitization, an industrial camera usually used for
print inspection was used. The final images have 400x 400 pixels. Due to the
object lens and distance to the investigated object gray-scale pictures with a
resolution of about 660 dpi were gained.

URL: https://archive.ics.uci.edu/ml/datasets/banknote+authentication

| Characteristics |  |
| -- | -- |
| Data Set Characteristics | Multivariate |
| Number of Instances | 1372 |
| Area | Computer |
| Attribute Characteristics | Real |
| Number of Attributes | 5 |
| Associated Tasks | Classification |
| Missing Values? | N/A |

### Attribute Information:

```
1. variance of Wavelet Transformed image (continuous)
2. skewness of Wavelet Transformed image (continuous)
3. curtosis of Wavelet Transformed image (continuous)
4. entropy of image (continuous)
5. class (integer)
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

## cube

Synthetic 3D dataset with point along 3 faces of a cube and projected to 2D using PCA.

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

## diabetes

This has been collected using direct questionnaires from the patients of Sylhet Diabetes
Hospital in Sylhet, Bangladesh and approved by a doctor.

URL: https://archive.ics.uci.edu/ml/datasets/Early+stage+diabetes+risk+prediction+dataset.

| Characteristics |  |
| -- | -- |
| Data Set Characteristics | Multivariate |
| Number of Instances | 520 |
| Area | Computer |
| Attribute Characteristics | N/A |
| Number of Attributes | 17 |
| Associated Tasks | Classification |
| Missing Values? | Yes |

### Attribute information:

```
Age 1.20-65
Sex 1. Male, 2.Female
Polyuria 1.Yes, 2.No.
Polydipsia 1.Yes, 2.No.
sudden weight loss 1.Yes, 2.No.
weakness 1.Yes, 2.No.
Polyphagia 1.Yes, 2.No.
Genital thrush 1.Yes, 2.No.
visual blurring 1.Yes, 2.No.
Itching 1.Yes, 2.No.
Irritability 1.Yes, 2.No.
delayed healing 1.Yes, 2.No.
partial paresis 1.Yes, 2.No.
muscle stiness 1.Yes, 2.No.
Alopecia 1.Yes, 2.No.
Obesity 1.Yes, 2.No.
Class 1.Positive, 2.Negative.
```

## epileptic

URL: https://archive.ics.uci.edu/ml/datasets/Epileptic+Seizure+Recognition

| Characteristics |  |
| -- | -- |
| Data Set Characteristics | Multivariate, Time-Series |
| Number of Instances | 11500 |
| Area | Life |
| Attribute Characteristics | Integer, Real |
| Number of Attributes | 179 |
| Associated Tasks | Classification, Clustering |
| Missing Values? | N/A |

### Attribute information:

```
The original dataset from the reference consists of 5 different folders, each with 100 files, with each file representing a single subject/person. Each file is a recording of brain activity for 23.6 seconds. The corresponding time-series is sampled into 4097 data points. Each data point is the value of the EEG recording at a different point in time. So we have total 500 individuals with each has 4097 data points for 23.5 seconds.

We divided and shuffled every 4097 data points into 23 chunks, each chunk contains 178 data points for 1 second, and each data point is the value of the EEG recording at a different point in time. So now we have 23 x 500 = 11500 pieces of information(row), each information contains 178 data points for 1 second(column), the last column represents the label y {1,2,3,4,5}.

The response variable is y in column 179, the Explanatory variables X1, X2, ..., X178

y contains the category of the 178-dimensional input vector. Specifically y in {1, 2, 3, 4, 5}:
5 - eyes open, means when they were recording the EEG signal of the brain the patient had their eyes open
4 - eyes closed, means when they were recording the EEG signal the patient had their eyes closed
3 - Yes they identify where the region of the tumor was in the brain and recording the EEG activity from the healthy brain area
2 - They recorder the EEG from the area where the tumor was located
1 - Recording of seizure activity

All subjects falling in classes 2, 3, 4, and 5 are subjects who did not have epileptic seizure. Only subjects in class 1 have epileptic seizure. Our motivation for creating this version of the data was to simplify access to the data via the creation of a .csv version of it. Although there are 5 classes most authors have done binary classification, namely class 1 (Epileptic seizure) against the rest.
```

## happiness

It is a case of supervised learning with the use of Receiver Operating Characteristic (ROC) to select the minimal set of attributes preserving or increasing predictability of the data.

URL: https://archive.ics.uci.edu/ml/datasets/Somerville+Happiness+Survey

| Characteristics |  |
| -- | -- |
| Data Set Characteristics | Multivariate |
| Number of Instances | 143 |
| Area | Life |
| Attribute Characteristics | Integer |
| Number of Attributes | 7 |
| Associated Tasks | Classification |
| Missing Values? | N/A |

### Attribute Information:

```
D = decision attribute (D) with values 0 (unhappy) and 1 (happy)
X1 = the availability of information about the city services
X2 = the cost of housing
X3 = the overall quality of public schools
X4 = your trust in the local police
X5 = the maintenance of streets and sidewalks
X6 = the availability of social community events

Attributes X1 to X6 have values 1 to 5.
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


## seismic

Mining activity was and is always connected with the occurrence of dangers which
are commonly called mining hazards. A special case of such threat is a seismic
hazard which frequently occurs in many underground mines. Seismic hazard is the
hardest detectable and predictable of natural hazards and in this respect it is
comparable to an earthquake. More and more advanced seismic and seismoacoustic
monitoring systems allow a better understanding rock mass processes and
definition of seismic hazard prediction methods.

URL: https://archive.ics.uci.edu/ml/datasets/seismic-bumps

| Characteristics |  |
| -- | -- |
| Data Set Characteristics | Multivariate |
| Number of Instances | 2584 |
| Area | N/A |
| Attribute Characteristics | Real |
| Number of Attributes | 19 |
| Associated Tasks | Classification |
| Missing Values? | N/A |

### Attribute information:

```
1. seismic: result of shift seismic hazard assessment in the mine working obtained by the seismic
method (a - lack of hazard, b - low hazard, c - high hazard, d - danger state);
2. seismoacoustic: result of shift seismic hazard assessment in the mine working obtained by the
seismoacoustic method;
3. shift: information about type of a shift (W - coal-getting, N -preparation shift);
4. genergy: seismic energy recorded within previous shift by the most active geophone (GMax) out of
geophones monitoring the longwall;
5. gpuls: a number of pulses recorded within previous shift by GMax;
6. gdenergy: a deviation of energy recorded within previous shift by GMax from average energy recorded
during eight previous shifts;
7. gdpuls: a deviation of a number of pulses recorded within previous shift by GMax from average number
of pulses recorded during eight previous shifts;
8. ghazard: result of shift seismic hazard assessment in the mine working obtained by the
seismoacoustic method based on registration coming form GMax only;
9. nbumps: the number of seismic bumps recorded within previous shift;
10. nbumps2: the number of seismic bumps (in energy range [10^2,10^3)) registered within previous shift;
11. nbumps3: the number of seismic bumps (in energy range [10^3,10^4)) registered within previous shift;
12. nbumps4: the number of seismic bumps (in energy range [10^4,10^5)) registered within previous shift;
13. nbumps5: the number of seismic bumps (in energy range [10^5,10^6)) registered within the last shift;
14. nbumps6: the number of seismic bumps (in energy range [10^6,10^7)) registered within previous shift;
15. nbumps7: the number of seismic bumps (in energy range [10^7,10^8)) registered within previous shift;
16. nbumps89: the number of seismic bumps (in energy range [10^8,10^10)) registered within previous shift;
17. energy: total energy of seismic bumps registered within previous shift;
18. maxenergy: the maximum energy of the seismic bumps registered within previous shift;
19. class: the decision attribute - '1' means that high energy seismic bump occurred in the next shift
('hazardous state'), '0' means that no high energy seismic bumps occurred in the next shift
('non-hazardous state').

```

## shading

Synthetic datasets used for debugging the pseudo shading found in my program

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

## US countries

Source not found and dataset not cleaned up yet

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

## Model Bestiary

### process model
f(x) = lambda (process model - constant) (density)

### data model:
- count - discrete model
- zero bound
- somewhat unbounded on the upper
- poisson is the natural choice, counting process
- other: negative binomial - is pois(lambda)*gamma(lambda|a,b)
y ~ Pois(lambda)
L = Pois(y|lambda) mem=lambda, var = lambda
rest mechanical, see photo
best guess after calculation: mean of the data

### Likelihood
- L = Pr(data|model parameters)
- each analysis should be approached individually
- sometimes you have a pre-existing function
- but don't start from there

### Data Models
- continuous
- integer/count
- Boolean(0/1) (survivor, either live or don't)
- Factor/categorical 

range restrictions?
- negative values allowed?
- is there an upper bound?
- Are there observable data near the bounds?
- if not, you can kinda ignore the boundaries
- if we had assumed a log distribution, we'd have negative fish
- if you predict 30% negative fish, you chose a bad distributed - Dr Seuss

Dirilichet - multivariate generalization of binomial

### How is the data modeled?
- What are the dominant sources of variability in the data?
- All statistical models assume ONLY measurement error
- Environmental/Social Sciences very much not true!!
- Process variability: Data Model needs to account for these sources of variability, process model might not be able too
- Missing Data

- Are there multiple processes involved?
- zero-inflated data
- Observed data may be probability of two composite processes - can only count if "there"

- Multiple Sources of Data?
- Tree growth - trees can have negative growth through dryness
- Tree fecundity - cone counts + seed trap
- Tree crown

### How is the process modeled?
- constant mean
- Multiple means by factor (ANOVA)
- As a function of covariates 
    - Linear models
    - generalized linear models
    - Nonlinear models
- Hierarchical models
- Mechanistic models

- Sometimes there's mechanistic example, sometimes it just fits the data well
- parabola of throw example
- mechanistic models can fail

### A bestiary of functions
- Polynomials
- piecewise polynomials
- Rational (ratio based)
- Exponential based
- Power based

Machine learning are more parameters, more flexible

### Polynomial:
- Linear with respect to model parameters
- Taylor series: Can approximate with any smooth continuous function
- A lot of data is asymptotical, but can also just look like it, may be completely different outside of range
- The more asymptotical, the worse the polynomial - trying to model a constant, which a polynomial can't
- Good approximation over a small range
- Flexible model always fails outside of range of observable data

### Piecewise
- Change point analysis
- Threshold
- Hockey stick (also threshold)
- general piecewise linear
- Splines ("f(x) is complicated")

### Rational
- Ratio
- Brings us towards nonlinear
- Hyperbolic: x in denominator, nonlinear by definition (downward curve)
- b is halfway between maximum and zero
- Michaelis-Menten: x also in numerator
- great example that is used a lot because it fits the data very well
- saturates
- Holling type III: allows for quadratics
- predator prey relationship -> read paper
- Holling type IV: shoots

### Exponential
- negative exponential: Decay
- monomulecular - similar to michaelis-menton, goes to asymptote more quickly
- Ricker: unimodal, goes back to zero -> PEEE

### Power
- power laws
- von beratalanffy
- Shepherd Hassel - often in fisheries
- non-rectangular hyperbola

### Linear Regression
- iid assumption in statistics
- intercept derivation
- slope derivation
- derivation depending on different parameters for the two above

### Questions
- MLE?
- L = likelihood?
- Dirilichet - multivariate generalization of binomial
- intercept in linear regression
- remember: product of means is not equal to mean of product
- get ugly equation of covariance and variance for slope
- once you know slope, just plug in to get intercept second
- ugly and unintuitive
- variance: once again: take derivative
- maximum likelihood variance estimate

Next: Matrix Notation -> wednesday
ANOVA and Linear Regression are the same problem
